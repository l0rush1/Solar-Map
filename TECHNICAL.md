## technical notes

this is a small side project, but actually has some technical depth (also used to all my repos having technical docs kinda refuse to push em without it now), this doc talks about the code and data flow, not the idea.

### stack overview

- **rust static library**: crate `astronomy_engine` compiled as a `staticlib` and exported with a c abi.
- **zig build**: `build.zig` runs `cargo build --release`, then builds a zig exe called `solarmap` and links it against the rust library.
- **ffi boundary**: zig sees the rust symbols as plain c functions and calls them to get satellite and star positions.

### rust side

the rust crate is configured in `Cargo.toml`:

- crate type is `staticlib`, entry file is `lib.rs`.
- dependencies:
  - `chrono` (no default features) for utc time handling.
  - `sgp4` (no default features, `libm` enabled) for orbit propagation of tle elements.

the build script `build.rs` is intentionally tiny and only emits `cargo:rerun-if-changed=build.rs` so cargo will rerun the build script if that file changes.

#### core types

in `lib.rs` you define a few c friendly types using `#[repr(C)]`:

- `Vec3 { x, y, z: f64 }`  
  simple 3d vector, used both for satellite positions and internal transforms. there is a helper `Vec3::nan()` that returns a vector full of `NaN` as a generic error sentinel.

- `AzEl { az, alt: f64 }`  
  azimuth and altitude in degrees, also with a `AzEl::nan()` helper for error cases.

- `StarAzEl { az: f64, alt: f64, mag: f32 }`  
  final per star output type: where in the sky and how bright.

there is also an internal `StarEntry { ra_deg, dec_deg, mag }` plus a constant array `STAR_CATALOG` with 200 of these entries (right ascension, declination, magnitude). `STAR_CATALOG_LEN` is `usize = 200` and is returned by the api so the caller knows how many entries are written.

#### math and coordinate helpers

key constants:

- `TAU = 2 * PI`
- wgs 84 earth shape:
  - `EARTH_A` (semi major axis, km)
  - `EARTH_F` (flattening)
  - `EARTH_E2` (eccentricity squared)

basic helpers:

- `dot(a: Vec3, b: Vec3) -> f64`  
  simple 3d dot product.

- `sub(a: Vec3, b: Vec3) -> Vec3`  
  vector subtraction.

time conversion:

- `to_naive(time: f64) -> Option<NaiveDateTime>`  
  interprets `time` as unix seconds, splits into integer seconds and nanoseconds, validates ranges using `i64::MIN/MAX`, then calls `Utc.timestamp_opt` and returns a `NaiveDateTime` if valid. this is the safe bridge from a raw `f64` coming in from ffi to chrono’s time api.

sgp4 propagation:

- `propagate_position(line1: &[u8], line2: &[u8], time: f64) -> Option<Vec3>`  
  - parses tle using `Elements::from_tle`.
  - converts the unix time to a chrono datetime with `to_naive`.
  - gets minutes since epoch with `elements.datetime_to_minutes_since_epoch`.
  - builds `Constants` from the elements and calls `constants.propagate`.
  - returns the resulting position `[x, y, z]` as a `Vec3`.
  - all of these use `Result::ok()?`, so any failure short circuits to `None`.

earth rotation and frames:

- `unix_to_jd(time: f64) -> f64`  
  converts unix seconds to julian date using a standard offset.

- `gst_from_jd(jd: f64) -> f64`  
  computes greenwich sidereal time in radians using a polynomial in `t` (centuries since j2000). angle is normalized into `[0, 2π)` with `rem_euclid(360.0)` and converted to radians.

- `eci_to_ecef(vec: Vec3, gst: f64) -> Vec3`  
  rotates the vector around the z axis by `gst` so you get earth fixed coordinates.

observer and local horizon:

- `observer(lat_deg, lon_deg, alt_m) -> Vec3`  
  converts an observer position from geodetic lat, lon (degrees) and altitude (meters) into ecef, using standard wgs 84 formulas and computing the prime vertical radius `n`.

- `horizon_from_delta(delta: Vec3, lat_rad, lon_rad) -> (az, alt)`  
  builds three orthonormal basis vectors:
  - `east`, `north`, `up` from the observer lat and lon.
  - projects the `delta` vector (ecef satellite minus observer) into that basis.
  - computes azimuth with `atan2` and normalizes into `[0, 2π)`.
  - computes altitude using `asin(u / range)` with clamping to avoid numerical issues.

- `horizon_from_direction(direction: Vec3, lat_rad, lon_rad) -> (az, alt)`  
  same math as above but for a direction vector on the celestial sphere, used for stars instead of satellites.

### extern api from rust

all exported functions are `#[no_mangle]` and `extern "C"` so their symbols are stable and can be called from zig without rust name mangling.

#### `sat_pos`

```rust
pub extern "C" fn sat_pos(tle1: *const c_char, tle2: *const c_char, time: f64) -> Vec3
```

- checks for null pointers and returns `Vec3::nan()` if either is null.
- wraps the raw pointers in `CStr`, takes the bytes, and feeds them into `propagate_position`.
- returns the propagated `Vec3` in eci coordinates, or `Vec3::nan()` if propagation fails.

#### `sat_altaz`

```rust
pub extern "C" fn sat_altaz(
    tle1: *const c_char,
    tle2: *const c_char,
    lat: f64,
    lon: f64,
    alt: f64,
    time: f64,
) -> AzEl
```

- same null pointer check and `CStr` conversion for tle data.
- calls `propagate_position` to get the eci satellite position.
- converts `time` to a julian date, then to gst, then converts sat eci to ecef with `eci_to_ecef`.
- builds the observer ecef position with `observer`.
- computes the difference vector and feeds it into `horizon_from_delta` to get az and alt in radians, then converts to degrees.
- returns `AzEl::nan()` if any step fails.

#### `star_positions`

```rust
pub extern "C" fn star_positions(lat: f64, lon: f64, time: f64, out: *mut StarAzEl) -> usize
```

- if `out` is null, returns `0` and does nothing.
- computes gst from the unix time via julian date.
- converts the observer lat and lon into radians once.
- uses `slice::from_raw_parts_mut(out, STAR_CATALOG_LEN)` to treat the output pointer as a mutable slice of `StarAzEl`.
- for each `StarEntry` in `STAR_CATALOG`:
  - builds a direction vector from ra and dec, on the unit sphere.
  - rotates it with `eci_to_ecef` using gst, producing an earth fixed direction.
  - uses `horizon_from_direction` to get az and alt in radians, converts to degrees.
  - writes az, alt, and magnitude into the corresponding `StarAzEl` slot.
- returns `STAR_CATALOG_LEN` as the number of written stars.

### zig side and linking

`build.zig` wires everything together in zig’s build system:

- defines `runCargo` that spawns `cargo build --release` using `std.process.Child`, inheriting stdio.
- in `build`, it:
  - calls `runCargo` and panics with a friendly message if it fails.
  - obtains a target via `b.standardTargetOptions`.
  - adds a module rooted at `main.zig` named `main`.
  - creates an executable:
    - name: `solarmap`
    - root module: `main`
  - adds `target/release` as a library search path.
  - links a system library called `astronomy_engine` and `libc`.
  - on windows, also links `ws2_32` and `userenv` to satisfy rust’s runtime needs when statically linking.
- finally, it installs the exe as a build artifact so `zig build run` can execute it.

the zig `main.zig` file is not in this repo, but the setup expects it to:

- declare the rust symbols with `extern "c"` signatures matching the rust api.
- call `sat_pos`, `sat_altaz`, and `star_positions` to get raw data.
- render or log that data (text ui, img generation, whatever you want).

### execution flow summary

1. `zig build run`
2. `build.zig` calls `cargo build --release` to produce `target/release/libastronomy_engine.*`.
3. zig compiles `main.zig` into `solarmap`, linking it with the rust static library.
4. at runtime, `solarmap` calls the rust `extern "C"` functions:
   - satellite calls go through `sgp4` and the coordinate transform chain.
   - star calls use the constant catalog and rotation math.
5. everything crossing the boundary is simple c friendly types: pointers, `f64`, and `#[repr(C)]` structs.

