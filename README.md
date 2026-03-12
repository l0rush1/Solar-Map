## solar map

small side project that plots satellites and bright stars in the sky using zig and rust. special thanks to Lorisi and Ana for the idea (totally not pressured)

it is not a full product, just a playground for space nerd stuff and some low level ffi experiments. Currently 200 stars rlly small shits hard coded, maybe wil make it better soon or not who knows

### what it does

- **satellites**: takes tle data and figures out where a satellite is in earth centered coordinates.
- **sky view**: converts that into azimuth and altitude for a given location and time.
- **stars**: has a tiny built in catalog of bright stars and gives you their positions in the sky.

under the hood there is a rust static library called `astronomy_engine` that uses `sgp4` and `chrono`, and a zig executable called `solarmap` that links to it.

### requirements

- **rust + cargo**: to build the `astronomy_engine` library.
- **zig**: to build and run the `solarmap` binary.

### build and run

from the project root:

```bash
cargo build --release
zig build run
```

this will build the rust library, build the zig executable and then run `solarmap`.

if you only want the binary without running it:

```bash
zig build
```

you should then find the `solarmap` executable in zig's usual `zig-out/bin` folder.

creds: lorush1
       Ana & Loris