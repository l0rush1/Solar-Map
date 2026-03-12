use chrono::{NaiveDateTime, TimeZone, Utc};
use core::{f64::consts::PI, slice};
use sgp4::{Constants, Elements};
use std::{ffi::CStr, os::raw::c_char};

#[cfg(all(target_os = "windows", target_env = "msvc"))]
#[allow(non_upper_case_globals)]
#[used]
#[link_section = ".rdata"]
#[export_name = "??_7type_info@@6B@"]
static RUSTC_TYPE_INFO_VFTABLE_STUB: [usize; 8] = [0; 8];

const TAU: f64 = PI * 2.0;
const EARTH_A: f64 = 6378.137;
const EARTH_F: f64 = 1.0 / 298.257223563;
const EARTH_E2: f64 = EARTH_F * (2.0 - EARTH_F);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    fn nan() -> Self {
        Self {
            x: f64::NAN,
            y: f64::NAN,
            z: f64::NAN,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AzEl {
    pub az: f64,
    pub alt: f64,
}

impl AzEl {
    fn nan() -> Self {
        Self {
            az: f64::NAN,
            alt: f64::NAN,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StarAzEl {
    pub az: f64,
    pub alt: f64,
    pub mag: f32,
}

#[derive(Copy, Clone)]
struct StarEntry {
    ra_deg: f64,
    dec_deg: f64,
    mag: f32,
}

const STAR_CATALOG: [StarEntry; 200] = [
    StarEntry {
        ra_deg: 101.287083,
        dec_deg: -16.716111,
        mag: -1.460,
    },
    StarEntry {
        ra_deg: 95.987917,
        dec_deg: -52.695833,
        mag: -0.720,
    },
    StarEntry {
        ra_deg: 213.915417,
        dec_deg: 19.182500,
        mag: -0.040,
    },
    StarEntry {
        ra_deg: 219.899583,
        dec_deg: -60.835278,
        mag: -0.010,
    },
    StarEntry {
        ra_deg: 279.234583,
        dec_deg: 38.783611,
        mag: 0.030,
    },
    StarEntry {
        ra_deg: 79.172500,
        dec_deg: 45.998056,
        mag: 0.080,
    },
    StarEntry {
        ra_deg: 78.634583,
        dec_deg: -8.201667,
        mag: 0.120,
    },
    StarEntry {
        ra_deg: 114.825417,
        dec_deg: 5.225000,
        mag: 0.380,
    },
    StarEntry {
        ra_deg: 24.428750,
        dec_deg: -57.236667,
        mag: 0.460,
    },
    StarEntry {
        ra_deg: 88.792917,
        dec_deg: 7.406944,
        mag: 0.500,
    },
    StarEntry {
        ra_deg: 210.955833,
        dec_deg: -60.373056,
        mag: 0.610,
    },
    StarEntry {
        ra_deg: 297.695833,
        dec_deg: 8.868333,
        mag: 0.770,
    },
    StarEntry {
        ra_deg: 68.980000,
        dec_deg: 16.509167,
        mag: 0.850,
    },
    StarEntry {
        ra_deg: 247.351667,
        dec_deg: -26.431944,
        mag: 0.960,
    },
    StarEntry {
        ra_deg: 201.298333,
        dec_deg: -11.161389,
        mag: 0.980,
    },
    StarEntry {
        ra_deg: 116.328750,
        dec_deg: 28.026111,
        mag: 1.140,
    },
    StarEntry {
        ra_deg: 344.412917,
        dec_deg: -29.622222,
        mag: 1.160,
    },
    StarEntry {
        ra_deg: 191.930000,
        dec_deg: -59.688611,
        mag: 1.250,
    },
    StarEntry {
        ra_deg: 310.357917,
        dec_deg: 45.280278,
        mag: 1.250,
    },
    StarEntry {
        ra_deg: 186.649583,
        dec_deg: -63.099167,
        mag: 1.330,
    },
    StarEntry {
        ra_deg: 219.900417,
        dec_deg: -60.835556,
        mag: 1.330,
    },
    StarEntry {
        ra_deg: 152.092917,
        dec_deg: 11.967222,
        mag: 1.350,
    },
    StarEntry {
        ra_deg: 104.656250,
        dec_deg: -28.972222,
        mag: 1.500,
    },
    StarEntry {
        ra_deg: 187.791250,
        dec_deg: -57.113333,
        mag: 1.630,
    },
    StarEntry {
        ra_deg: 263.402083,
        dec_deg: -37.103889,
        mag: 1.630,
    },
    StarEntry {
        ra_deg: 81.282917,
        dec_deg: 6.349722,
        mag: 1.640,
    },
    StarEntry {
        ra_deg: 81.572917,
        dec_deg: 28.607500,
        mag: 1.650,
    },
    StarEntry {
        ra_deg: 138.300000,
        dec_deg: -69.717222,
        mag: 1.680,
    },
    StarEntry {
        ra_deg: 84.053333,
        dec_deg: -1.201944,
        mag: 1.700,
    },
    StarEntry {
        ra_deg: 186.652083,
        dec_deg: -63.099444,
        mag: 1.730,
    },
    StarEntry {
        ra_deg: 332.058333,
        dec_deg: -46.961111,
        mag: 1.740,
    },
    StarEntry {
        ra_deg: 193.507083,
        dec_deg: 55.959722,
        mag: 1.770,
    },
    StarEntry {
        ra_deg: 122.383333,
        dec_deg: -47.336667,
        mag: 1.780,
    },
    StarEntry {
        ra_deg: 51.080833,
        dec_deg: 49.861111,
        mag: 1.790,
    },
    StarEntry {
        ra_deg: 165.932083,
        dec_deg: 61.750833,
        mag: 1.790,
    },
    StarEntry {
        ra_deg: 107.097917,
        dec_deg: -26.393333,
        mag: 1.840,
    },
    StarEntry {
        ra_deg: 276.042917,
        dec_deg: -34.384722,
        mag: 1.850,
    },
    StarEntry {
        ra_deg: 125.628333,
        dec_deg: -59.509722,
        mag: 1.860,
    },
    StarEntry {
        ra_deg: 206.885000,
        dec_deg: 49.313333,
        mag: 1.860,
    },
    StarEntry {
        ra_deg: 264.330000,
        dec_deg: -42.997778,
        mag: 1.870,
    },
    StarEntry {
        ra_deg: 89.882083,
        dec_deg: 44.947500,
        mag: 1.900,
    },
    StarEntry {
        ra_deg: 252.166250,
        dec_deg: -69.027778,
        mag: 1.920,
    },
    StarEntry {
        ra_deg: 99.427917,
        dec_deg: 16.399167,
        mag: 1.930,
    },
    StarEntry {
        ra_deg: 306.412083,
        dec_deg: -56.735000,
        mag: 1.940,
    },
    StarEntry {
        ra_deg: 131.175833,
        dec_deg: -54.708333,
        mag: 1.960,
    },
    StarEntry {
        ra_deg: 95.675000,
        dec_deg: -17.955833,
        mag: 1.980,
    },
    StarEntry {
        ra_deg: 113.650000,
        dec_deg: 31.888333,
        mag: 1.980,
    },
    StarEntry {
        ra_deg: 141.896667,
        dec_deg: -8.658611,
        mag: 1.980,
    },
    StarEntry {
        ra_deg: 31.793333,
        dec_deg: 23.462500,
        mag: 2.000,
    },
    StarEntry {
        ra_deg: 239.875833,
        dec_deg: 25.920278,
        mag: 2.000,
    },
    StarEntry {
        ra_deg: 37.952917,
        dec_deg: 89.264167,
        mag: 2.020,
    },
    StarEntry {
        ra_deg: 283.816250,
        dec_deg: -26.296667,
        mag: 2.020,
    },
    StarEntry {
        ra_deg: 10.897500,
        dec_deg: -17.986667,
        mag: 2.040,
    },
    StarEntry {
        ra_deg: 85.189583,
        dec_deg: -1.942778,
        mag: 2.050,
    },
    StarEntry {
        ra_deg: 2.097083,
        dec_deg: 29.090556,
        mag: 2.060,
    },
    StarEntry {
        ra_deg: 17.432917,
        dec_deg: 35.620556,
        mag: 2.060,
    },
    StarEntry {
        ra_deg: 86.939167,
        dec_deg: -9.669722,
        mag: 2.060,
    },
    StarEntry {
        ra_deg: 211.670833,
        dec_deg: -36.370000,
        mag: 2.060,
    },
    StarEntry {
        ra_deg: 222.676250,
        dec_deg: 74.155556,
        mag: 2.080,
    },
    StarEntry {
        ra_deg: 263.733750,
        dec_deg: 12.560000,
        mag: 2.080,
    },
    StarEntry {
        ra_deg: 340.667083,
        dec_deg: -46.884722,
        mag: 2.100,
    },
    StarEntry {
        ra_deg: 47.042083,
        dec_deg: 40.955556,
        mag: 2.120,
    },
    StarEntry {
        ra_deg: 177.265000,
        dec_deg: 14.571944,
        mag: 2.140,
    },
    StarEntry {
        ra_deg: 190.379167,
        dec_deg: -48.959722,
        mag: 2.170,
    },
    StarEntry {
        ra_deg: 305.557083,
        dec_deg: 40.256667,
        mag: 2.200,
    },
    StarEntry {
        ra_deg: 136.999167,
        dec_deg: -43.432500,
        mag: 2.210,
    },
    StarEntry {
        ra_deg: 10.127083,
        dec_deg: 56.537222,
        mag: 2.230,
    },
    StarEntry {
        ra_deg: 83.001667,
        dec_deg: -0.299167,
        mag: 2.230,
    },
    StarEntry {
        ra_deg: 233.672083,
        dec_deg: 26.714722,
        mag: 2.230,
    },
    StarEntry {
        ra_deg: 269.151667,
        dec_deg: 51.488889,
        mag: 2.230,
    },
    StarEntry {
        ra_deg: 120.896250,
        dec_deg: -40.003333,
        mag: 2.250,
    },
    StarEntry {
        ra_deg: 139.272500,
        dec_deg: -59.275278,
        mag: 2.250,
    },
    StarEntry {
        ra_deg: 30.975000,
        dec_deg: 42.329722,
        mag: 2.260,
    },
    StarEntry {
        ra_deg: 2.294583,
        dec_deg: 59.149722,
        mag: 2.270,
    },
    StarEntry {
        ra_deg: 200.981250,
        dec_deg: 54.925278,
        mag: 2.270,
    },
    StarEntry {
        ra_deg: 252.540833,
        dec_deg: -34.293333,
        mag: 2.290,
    },
    StarEntry {
        ra_deg: 204.971667,
        dec_deg: -53.466389,
        mag: 2.300,
    },
    StarEntry {
        ra_deg: 220.482500,
        dec_deg: -47.388333,
        mag: 2.300,
    },
    StarEntry {
        ra_deg: 218.876667,
        dec_deg: -42.157778,
        mag: 2.310,
    },
    StarEntry {
        ra_deg: 240.083333,
        dec_deg: -22.621667,
        mag: 2.320,
    },
    StarEntry {
        ra_deg: 165.460417,
        dec_deg: 56.382500,
        mag: 2.370,
    },
    StarEntry {
        ra_deg: 6.570833,
        dec_deg: -42.306111,
        mag: 2.390,
    },
    StarEntry {
        ra_deg: 326.046667,
        dec_deg: 9.875000,
        mag: 2.390,
    },
    StarEntry {
        ra_deg: 265.622083,
        dec_deg: -39.030000,
        mag: 2.410,
    },
    StarEntry {
        ra_deg: 345.943750,
        dec_deg: 28.082778,
        mag: 2.420,
    },
    StarEntry {
        ra_deg: 257.594583,
        dec_deg: -15.724722,
        mag: 2.430,
    },
    StarEntry {
        ra_deg: 178.457500,
        dec_deg: 53.694722,
        mag: 2.440,
    },
    StarEntry {
        ra_deg: 319.645000,
        dec_deg: 62.585556,
        mag: 2.440,
    },
    StarEntry {
        ra_deg: 111.023750,
        dec_deg: -29.303056,
        mag: 2.450,
    },
    StarEntry {
        ra_deg: 311.552917,
        dec_deg: 33.970278,
        mag: 2.460,
    },
    StarEntry {
        ra_deg: 14.177083,
        dec_deg: 60.716667,
        mag: 2.470,
    },
    StarEntry {
        ra_deg: 346.190417,
        dec_deg: 15.205278,
        mag: 2.490,
    },
    StarEntry {
        ra_deg: 140.528333,
        dec_deg: -55.010833,
        mag: 2.500,
    },
    StarEntry {
        ra_deg: 45.570000,
        dec_deg: 4.089722,
        mag: 2.530,
    },
    StarEntry {
        ra_deg: 208.885000,
        dec_deg: -47.288333,
        mag: 2.550,
    },
    StarEntry {
        ra_deg: 168.527083,
        dec_deg: 20.523611,
        mag: 2.560,
    },
    StarEntry {
        ra_deg: 249.289583,
        dec_deg: -10.567222,
        mag: 2.560,
    },
    StarEntry {
        ra_deg: 83.182500,
        dec_deg: -17.822222,
        mag: 2.580,
    },
    StarEntry {
        ra_deg: 183.951667,
        dec_deg: -17.541944,
        mag: 2.590,
    },
    StarEntry {
        ra_deg: 182.089583,
        dec_deg: -50.722500,
        mag: 2.600,
    },
    StarEntry {
        ra_deg: 285.652917,
        dec_deg: -29.880278,
        mag: 2.600,
    },
    StarEntry {
        ra_deg: 154.992917,
        dec_deg: 19.841667,
        mag: 2.610,
    },
    StarEntry {
        ra_deg: 229.251667,
        dec_deg: -9.383056,
        mag: 2.610,
    },
    StarEntry {
        ra_deg: 89.930417,
        dec_deg: 37.212500,
        mag: 2.620,
    },
    StarEntry {
        ra_deg: 241.359167,
        dec_deg: -19.805556,
        mag: 2.620,
    },
    StarEntry {
        ra_deg: 28.660000,
        dec_deg: 20.808056,
        mag: 2.640,
    },
    StarEntry {
        ra_deg: 84.912083,
        dec_deg: -34.074167,
        mag: 2.640,
    },
    StarEntry {
        ra_deg: 188.596667,
        dec_deg: -23.396667,
        mag: 2.650,
    },
    StarEntry {
        ra_deg: 236.067083,
        dec_deg: 6.425556,
        mag: 2.650,
    },
    StarEntry {
        ra_deg: 21.454167,
        dec_deg: 60.235278,
        mag: 2.680,
    },
    StarEntry {
        ra_deg: 208.671250,
        dec_deg: 18.397778,
        mag: 2.680,
    },
    StarEntry {
        ra_deg: 224.632917,
        dec_deg: -43.133889,
        mag: 2.680,
    },
    StarEntry {
        ra_deg: 74.248333,
        dec_deg: 33.166111,
        mag: 2.690,
    },
    StarEntry {
        ra_deg: 161.692500,
        dec_deg: -49.420000,
        mag: 2.690,
    },
    StarEntry {
        ra_deg: 189.295833,
        dec_deg: -69.135556,
        mag: 2.690,
    },
    StarEntry {
        ra_deg: 262.690833,
        dec_deg: -37.295833,
        mag: 2.690,
    },
    StarEntry {
        ra_deg: 109.285833,
        dec_deg: -37.097500,
        mag: 2.700,
    },
    StarEntry {
        ra_deg: 221.246667,
        dec_deg: 27.074167,
        mag: 2.700,
    },
    StarEntry {
        ra_deg: 275.248750,
        dec_deg: -29.828056,
        mag: 2.700,
    },
    StarEntry {
        ra_deg: 296.565000,
        dec_deg: 10.613333,
        mag: 2.720,
    },
    StarEntry {
        ra_deg: 243.586250,
        dec_deg: -3.694444,
        mag: 2.740,
    },
    StarEntry {
        ra_deg: 245.997917,
        dec_deg: 61.514167,
        mag: 2.740,
    },
    StarEntry {
        ra_deg: 200.149167,
        dec_deg: -36.712222,
        mag: 2.750,
    },
    StarEntry {
        ra_deg: 222.719583,
        dec_deg: -16.041667,
        mag: 2.750,
    },
    StarEntry {
        ra_deg: 160.739167,
        dec_deg: -64.394444,
        mag: 2.760,
    },
    StarEntry {
        ra_deg: 83.858333,
        dec_deg: -5.910000,
        mag: 2.770,
    },
    StarEntry {
        ra_deg: 247.555000,
        dec_deg: 21.489722,
        mag: 2.770,
    },
    StarEntry {
        ra_deg: 265.868333,
        dec_deg: 4.567222,
        mag: 2.770,
    },
    StarEntry {
        ra_deg: 233.785417,
        dec_deg: -41.166944,
        mag: 2.780,
    },
    StarEntry {
        ra_deg: 76.962500,
        dec_deg: -5.086389,
        mag: 2.790,
    },
    StarEntry {
        ra_deg: 262.608333,
        dec_deg: 52.301389,
        mag: 2.790,
    },
    StarEntry {
        ra_deg: 6.437917,
        dec_deg: -77.254167,
        mag: 2.800,
    },
    StarEntry {
        ra_deg: 183.786250,
        dec_deg: -58.748889,
        mag: 2.800,
    },
    StarEntry {
        ra_deg: 121.885833,
        dec_deg: -24.304167,
        mag: 2.810,
    },
    StarEntry {
        ra_deg: 250.321667,
        dec_deg: 31.603056,
        mag: 2.810,
    },
    StarEntry {
        ra_deg: 276.992500,
        dec_deg: -25.421667,
        mag: 2.810,
    },
    StarEntry {
        ra_deg: 248.970833,
        dec_deg: -28.216111,
        mag: 2.820,
    },
    StarEntry {
        ra_deg: 3.309167,
        dec_deg: 15.183611,
        mag: 2.830,
    },
    StarEntry {
        ra_deg: 195.544167,
        dec_deg: 10.959167,
        mag: 2.830,
    },
    StarEntry {
        ra_deg: 82.061250,
        dec_deg: -20.759444,
        mag: 2.840,
    },
    StarEntry {
        ra_deg: 58.532917,
        dec_deg: 31.883611,
        mag: 2.850,
    },
    StarEntry {
        ra_deg: 238.785417,
        dec_deg: -63.430556,
        mag: 2.850,
    },
    StarEntry {
        ra_deg: 261.325000,
        dec_deg: -55.530000,
        mag: 2.850,
    },
    StarEntry {
        ra_deg: 29.692500,
        dec_deg: -61.569722,
        mag: 2.860,
    },
    StarEntry {
        ra_deg: 334.625417,
        dec_deg: -60.259722,
        mag: 2.860,
    },
    StarEntry {
        ra_deg: 56.871250,
        dec_deg: 24.105000,
        mag: 2.870,
    },
    StarEntry {
        ra_deg: 296.243750,
        dec_deg: 45.130833,
        mag: 2.870,
    },
    StarEntry {
        ra_deg: 326.760000,
        dec_deg: -16.127222,
        mag: 2.870,
    },
    StarEntry {
        ra_deg: 95.740000,
        dec_deg: 22.513611,
        mag: 2.880,
    },
    StarEntry {
        ra_deg: 113.650000,
        dec_deg: 31.888611,
        mag: 2.880,
    },
    StarEntry {
        ra_deg: 59.463333,
        dec_deg: 40.010278,
        mag: 2.890,
    },
    StarEntry {
        ra_deg: 229.727500,
        dec_deg: -68.679444,
        mag: 2.890,
    },
    StarEntry {
        ra_deg: 239.712917,
        dec_deg: -26.114167,
        mag: 2.890,
    },
    StarEntry {
        ra_deg: 245.297083,
        dec_deg: -25.592778,
        mag: 2.890,
    },
    StarEntry {
        ra_deg: 287.440833,
        dec_deg: -21.023611,
        mag: 2.890,
    },
    StarEntry {
        ra_deg: 111.787500,
        dec_deg: 8.289444,
        mag: 2.900,
    },
    StarEntry {
        ra_deg: 194.007083,
        dec_deg: 38.318333,
        mag: 2.900,
    },
    StarEntry {
        ra_deg: 322.889583,
        dec_deg: -5.571111,
        mag: 2.910,
    },
    StarEntry {
        ra_deg: 46.199167,
        dec_deg: 53.506389,
        mag: 2.930,
    },
    StarEntry {
        ra_deg: 102.484167,
        dec_deg: -50.614722,
        mag: 2.930,
    },
    StarEntry {
        ra_deg: 340.750417,
        dec_deg: 30.221389,
        mag: 2.940,
    },
    StarEntry {
        ra_deg: 59.507500,
        dec_deg: -13.508611,
        mag: 2.950,
    },
    StarEntry {
        ra_deg: 187.466250,
        dec_deg: -16.515556,
        mag: 2.950,
    },
    StarEntry {
        ra_deg: 262.960417,
        dec_deg: -49.876111,
        mag: 2.950,
    },
    StarEntry {
        ra_deg: 331.445833,
        dec_deg: -0.319722,
        mag: 2.960,
    },
    StarEntry {
        ra_deg: 100.982917,
        dec_deg: 25.131111,
        mag: 2.980,
    },
    StarEntry {
        ra_deg: 146.462917,
        dec_deg: 23.774167,
        mag: 2.980,
    },
    StarEntry {
        ra_deg: 75.492083,
        dec_deg: 43.823333,
        mag: 2.990,
    },
    StarEntry {
        ra_deg: 271.452083,
        dec_deg: -30.424167,
        mag: 2.990,
    },
    StarEntry {
        ra_deg: 286.352500,
        dec_deg: 13.863333,
        mag: 2.990,
    },
    StarEntry {
        ra_deg: 32.385833,
        dec_deg: 34.987222,
        mag: 3.000,
    },
    StarEntry {
        ra_deg: 84.411250,
        dec_deg: 21.142500,
        mag: 3.000,
    },
    StarEntry {
        ra_deg: 182.531250,
        dec_deg: -22.619722,
        mag: 3.000,
    },
    StarEntry {
        ra_deg: 199.730417,
        dec_deg: -23.171667,
        mag: 3.000,
    },
    StarEntry {
        ra_deg: 55.731250,
        dec_deg: 47.787500,
        mag: 3.010,
    },
    StarEntry {
        ra_deg: 146.775417,
        dec_deg: -65.071944,
        mag: 3.010,
    },
    StarEntry {
        ra_deg: 167.415833,
        dec_deg: 44.498611,
        mag: 3.010,
    },
    StarEntry {
        ra_deg: 328.482083,
        dec_deg: -37.365000,
        mag: 3.010,
    },
    StarEntry {
        ra_deg: 95.078333,
        dec_deg: -30.063333,
        mag: 3.020,
    },
    StarEntry {
        ra_deg: 105.756250,
        dec_deg: -23.833333,
        mag: 3.020,
    },
    StarEntry {
        ra_deg: 218.019583,
        dec_deg: 38.308333,
        mag: 3.030,
    },
    StarEntry {
        ra_deg: 266.896250,
        dec_deg: -40.126944,
        mag: 3.030,
    },
    StarEntry {
        ra_deg: 34.836250,
        dec_deg: -2.977500,
        mag: 3.040,
    },
    StarEntry {
        ra_deg: 207.404167,
        dec_deg: -42.473889,
        mag: 3.040,
    },
    StarEntry {
        ra_deg: 155.582083,
        dec_deg: 41.499444,
        mag: 3.050,
    },
    StarEntry {
        ra_deg: 191.570417,
        dec_deg: -68.108056,
        mag: 3.050,
    },
    StarEntry {
        ra_deg: 230.182083,
        dec_deg: 71.833889,
        mag: 3.050,
    },
    StarEntry {
        ra_deg: 288.138750,
        dec_deg: 67.661667,
        mag: 3.070,
    },
    StarEntry {
        ra_deg: 252.967500,
        dec_deg: -38.047500,
        mag: 3.080,
    },
    StarEntry {
        ra_deg: 292.680417,
        dec_deg: 27.959722,
        mag: 3.080,
    },
    StarEntry {
        ra_deg: 305.252917,
        dec_deg: -14.781389,
        mag: 3.080,
    },
    StarEntry {
        ra_deg: 133.848333,
        dec_deg: 5.945556,
        mag: 3.110,
    },
    StarEntry {
        ra_deg: 162.406250,
        dec_deg: -16.193611,
        mag: 3.110,
    },
    StarEntry {
        ra_deg: 274.406667,
        dec_deg: -36.761667,
        mag: 3.110,
    },
    StarEntry {
        ra_deg: 309.391667,
        dec_deg: -47.291389,
        mag: 3.110,
    },
    StarEntry {
        ra_deg: 87.740000,
        dec_deg: -35.768333,
        mag: 3.120,
    },
    StarEntry {
        ra_deg: 140.263750,
        dec_deg: 34.392500,
        mag: 3.130,
    },
    StarEntry {
        ra_deg: 142.805417,
        dec_deg: -57.034444,
        mag: 3.130,
    },
    StarEntry {
        ra_deg: 173.945000,
        dec_deg: -63.019722,
        mag: 3.130,
    },
    StarEntry {
        ra_deg: 224.790417,
        dec_deg: -42.104167,
        mag: 3.130,
    },
];
const STAR_CATALOG_LEN: usize = 200;

fn dot(a: Vec3, b: Vec3) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn sub(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
}

fn to_naive(time: f64) -> Option<NaiveDateTime> {
    if !time.is_finite() {
        return None;
    }
    let base = time.floor();
    if base < i64::MIN as f64 || base > i64::MAX as f64 {
        return None;
    }
    let mut seconds = base as i64;
    let mut nanos = ((time - base) * 1e9).round() as i64;
    if nanos >= 1_000_000_000 {
        nanos -= 1_000_000_000;
        seconds = seconds.checked_add(1)?;
    }
    if nanos < 0 {
        nanos = 0;
    }
    Utc.timestamp_opt(seconds, nanos as u32)
        .single()
        .map(|dt| dt.naive_utc())
}

fn propagate_position(line1: &[u8], line2: &[u8], time: f64) -> Option<Vec3> {
    let elements = Elements::from_tle(line1, line2).ok()?;
    let datetime = to_naive(time)?;
    let minutes = elements.datetime_to_minutes_since_epoch(&datetime).ok()?;
    let constants = Constants::from_elements(&elements).ok()?;
    let prediction = constants.propagate(minutes).ok()?;
    let [x, y, z] = prediction.position;
    Some(Vec3 { x, y, z })
}

fn unix_to_jd(time: f64) -> f64 {
    time / 86400.0 + 2440587.5
}

fn gst_from_jd(jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    let mut gst = 280.46061837 + 360.98564736629 * (jd - 2451545.0) + 0.000387933 * t * t
        - t * t * t / 38710000.0;
    gst = gst.rem_euclid(360.0);
    gst.to_radians()
}

fn eci_to_ecef(vec: Vec3, gst: f64) -> Vec3 {
    let cos = gst.cos();
    let sin = gst.sin();
    Vec3 {
        x: vec.x * cos + vec.y * sin,
        y: -vec.x * sin + vec.y * cos,
        z: vec.z,
    }
}

fn observer(lat_deg: f64, lon_deg: f64, alt_m: f64) -> Vec3 {
    let lat = lat_deg.to_radians();
    let lon = lon_deg.to_radians();
    let sin_lat = lat.sin();
    let cos_lat = lat.cos();
    let cos_lon = lon.cos();
    let sin_lon = lon.sin();
    let alt = alt_m / 1000.0;
    let n = EARTH_A / (1.0 - EARTH_E2 * sin_lat * sin_lat).sqrt();
    Vec3 {
        x: (n + alt) * cos_lat * cos_lon,
        y: (n + alt) * cos_lat * sin_lon,
        z: (n * (1.0 - EARTH_E2) + alt) * sin_lat,
    }
}

fn horizon_from_delta(delta: Vec3, lat_rad: f64, lon_rad: f64) -> (f64, f64) {
    let sin_lat = lat_rad.sin();
    let cos_lat = lat_rad.cos();
    let sin_lon = lon_rad.sin();
    let cos_lon = lon_rad.cos();
    let east = Vec3 {
        x: -sin_lon,
        y: cos_lon,
        z: 0.0,
    };
    let north = Vec3 {
        x: -sin_lat * cos_lon,
        y: -sin_lat * sin_lon,
        z: cos_lat,
    };
    let up = Vec3 {
        x: cos_lat * cos_lon,
        y: cos_lat * sin_lon,
        z: sin_lat,
    };
    let e = dot(delta, east);
    let n = dot(delta, north);
    let u = dot(delta, up);
    let mut az = e.atan2(n);
    if az < 0.0 {
        az += TAU;
    }
    let range = (e * e + n * n + u * u).sqrt().max(1e-12);
    let alt = (u / range).clamp(-1.0, 1.0).asin();
    (az, alt)
}

fn horizon_from_direction(direction: Vec3, lat_rad: f64, lon_rad: f64) -> (f64, f64) {
    let sin_lat = lat_rad.sin();
    let cos_lat = lat_rad.cos();
    let sin_lon = lon_rad.sin();
    let cos_lon = lon_rad.cos();
    let east = Vec3 {
        x: -sin_lon,
        y: cos_lon,
        z: 0.0,
    };
    let north = Vec3 {
        x: -sin_lat * cos_lon,
        y: -sin_lat * sin_lon,
        z: cos_lat,
    };
    let up = Vec3 {
        x: cos_lat * cos_lon,
        y: cos_lat * sin_lon,
        z: sin_lat,
    };
    let e = dot(direction, east);
    let n = dot(direction, north);
    let u = dot(direction, up);
    let mut az = e.atan2(n);
    if az < 0.0 {
        az += TAU;
    }
    let range = (e * e + n * n + u * u).sqrt().max(1e-12);
    let alt = (u / range).clamp(-1.0, 1.0).asin();
    (az, alt)
}

#[no_mangle]
pub extern "C" fn sat_pos(tle1: *const c_char, tle2: *const c_char, time: f64) -> Vec3 {
    if tle1.is_null() || tle2.is_null() {
        return Vec3::nan();
    }
    let line1 = unsafe { CStr::from_ptr(tle1) };
    let line2 = unsafe { CStr::from_ptr(tle2) };
    propagate_position(line1.to_bytes(), line2.to_bytes(), time).unwrap_or(Vec3::nan())
}

#[no_mangle]
pub extern "C" fn sat_altaz(
    tle1: *const c_char,
    tle2: *const c_char,
    lat: f64,
    lon: f64,
    alt: f64,
    time: f64,
) -> AzEl {
    if tle1.is_null() || tle2.is_null() {
        return AzEl::nan();
    }
    let line1 = unsafe { CStr::from_ptr(tle1) };
    let line2 = unsafe { CStr::from_ptr(tle2) };
    if let Some(pos) = propagate_position(line1.to_bytes(), line2.to_bytes(), time) {
        let gst = gst_from_jd(unix_to_jd(time));
        let sat_ecef = eci_to_ecef(pos, gst);
        let obs = observer(lat, lon, alt);
        let (az_rad, alt_rad) =
            horizon_from_delta(sub(sat_ecef, obs), lat.to_radians(), lon.to_radians());
        AzEl {
            az: az_rad.to_degrees(),
            alt: alt_rad.to_degrees(),
        }
    } else {
        AzEl::nan()
    }
}

#[no_mangle]
pub extern "C" fn star_positions(lat: f64, lon: f64, time: f64, out: *mut StarAzEl) -> usize {
    if out.is_null() {
        return 0;
    }
    let gst = gst_from_jd(unix_to_jd(time));
    let lat_rad = lat.to_radians();
    let lon_rad = lon.to_radians();
    let buffer = unsafe { slice::from_raw_parts_mut(out, STAR_CATALOG_LEN) };
    for (slot, entry) in buffer.iter_mut().zip(STAR_CATALOG.iter()) {
        let ra = entry.ra_deg.to_radians();
        let dec = entry.dec_deg.to_radians();
        let direction = Vec3 {
            x: dec.cos() * ra.cos(),
            y: dec.cos() * ra.sin(),
            z: dec.sin(),
        };
        let rotated = eci_to_ecef(direction, gst);
        let (az_rad, alt_rad) = horizon_from_direction(rotated, lat_rad, lon_rad);
        slot.az = az_rad.to_degrees();
        slot.alt = alt_rad.to_degrees();
        slot.mag = entry.mag;
    }
    STAR_CATALOG_LEN
}
