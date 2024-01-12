pub const G: f64 = 6.67428 * 1e-11; // N * m2 * kg2
pub const AU: f64 = 1.496 * 1e11; // m
pub const SCALE: f64 = 2000.0 / AU;
pub const TIMESTEP: f32 = 60.0 * 60.0; // * 24.0; //seconds in day

// Mass, kg
pub const SUN_MASS: f64 = 1.98892 * 1e30;
pub const MERCURY_MASS: f64 = 3.3011 * 1e23;
pub const VENUS_MASS: f64 = 4.8675 * 1e24;
pub const EARTH_MASS: f64 = 5.9742 * 1e24;
pub const MOON_MASS: f64 = 7.342 * 1e22;
pub const MARS_MASS: f64 = 6.4171 * 1e23;

// Velocity, m/s
pub const MERCURY_VEL: f32 = 4.736 * 1e4;
pub const VENUS_VEL: f32 = 3.502 * 1e4;
pub const EARTH_VEL: f32 = 2.9783 * 1e4;
pub const MOON_VEL: f32 = 1.022 * 1e3;
pub const MARS_VEL: f32 = 2.4077 * 1e4;

// Rdius, m
pub const SUN_RADIUS: f64 = 6.957 * 1e8;
pub const MERCURY_RADIUS: f64 = 2.4397 * 1e6;
pub const VENUS_RADIUS: f64 = 6.0518 * 1e6;
pub const EARTH_RADIUS: f64 = 6.371 * 1e6;
pub const MOON_RADIUS: f64 = 1.7374 * 1e6;
pub const MARS_RADIUS: f64 = 3.3895 * 1e6;

// Distance, m
pub const MERCURY_DISTANCE: f64 = 0.387 * AU;
pub const VENUS_DISTANCE: f64 = 0.72 * AU;
pub const EARTH_DISTANCE: f64 = AU;
pub const MOON_DISTANCE: f64 = 3.84400 * 1e8;
pub const MARS_DISTANCE: f64 = 1.52 * AU;

// Inclination, radians
pub const MERCURY_INCLINATION: f32 = 0.05899212871740834;
pub const VENUS_INCLINATION: f32 = 0.059341194567807204;
pub const EARTH_INCLINATION: f32 = 0.12487830798019428;
pub const MOON_INCLINATION: f32 = 0.08979719001510825;
pub const MARS_INCLINATION: f32 = 0.0322885911618951;
