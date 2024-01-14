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
pub const CERES_MASS: f64 = 9.38392 * 1e20;
pub const JUPITER_MASS: f64 = 1.8982 * 1e27;
pub const SATURN_MASS: f64 = 5.6834 * 1e26;
pub const URANUS_MASS: f64 = 8.6810 * 1e25;
pub const NEPTUNE_MASS: f64 = 1.02413 * 1e26;
pub const PLUTO_MASS: f64 = 1.303 * 1e22;

// Velocity, m/s
pub const MERCURY_VEL: f32 = 4.736 * 1e4;
pub const VENUS_VEL: f32 = 3.502 * 1e4;
pub const EARTH_VEL: f32 = 2.9783 * 1e4;
pub const MOON_VEL: f32 = 1.022 * 1e3;
pub const MARS_VEL: f32 = 2.4077 * 1e4;
pub const CERES_VEL: f32 = 1.79 * 1e4;
pub const JUPITER_VEL: f32 = 1.307 * 1e4;
pub const SATURN_VEL: f32 = 9.68 * 1e3;
pub const URANUS_VEL: f32 = 6.8 * 1e3;
pub const NEPTUNE_VEL: f32 = 5.43 * 1e3;
pub const PLUTO_VEL: f32 = 4.743 * 1e3;

// Rdius, m
pub const SUN_RADIUS: f64 = 6.957 * 1e8;
pub const MERCURY_RADIUS: f64 = 2.4397 * 1e6;
pub const VENUS_RADIUS: f64 = 6.0518 * 1e6;
pub const EARTH_RADIUS: f64 = 6.371 * 1e6;
pub const MOON_RADIUS: f64 = 1.7374 * 1e6;
pub const MARS_RADIUS: f64 = 3.3895 * 1e6;
pub const CERES_RADIUS: f64 = 4.697 * 1e5;
pub const JUPITER_RADIUS: f64 = 6.9911 * 1e7;
pub const SATURN_RADIUS: f64 = 5.8232 * 1e7;
pub const URANUS_RADIUS: f64 = 2.5362 * 1e7;
pub const NEPTUNE_RADIUS: f64 = 2.4622 * 1e7;
pub const PLUTO_RADIUS: f64 = 1.1883 * 1e6;

// Distance, m
pub const MERCURY_DISTANCE: f64 = 0.387 * AU;
pub const VENUS_DISTANCE: f64 = 0.72 * AU;
pub const EARTH_DISTANCE: f64 = AU;
pub const MOON_DISTANCE: f64 = 3.84400 * 1e8;
pub const MARS_DISTANCE: f64 = 1.52 * AU;
pub const CERES_DISTANCE: f64 = 2.8 * AU;
pub const JUPITER_DISTANCE: f64 = 5.2 * AU;
pub const SATURN_DISTANCE: f64 = 9.0 * AU;
pub const URANUS_DISTANCE: f64 = 20.0 * AU;
pub const NEPTUNE_DISTANCE: f64 = 30.1 * AU;
pub const PLUTO_DISTANCE: f64 = 35.0 * AU;

// Inclination, radians
pub const MERCURY_INCLINATION: f32 = 0.05899212871740834;
pub const VENUS_INCLINATION: f32 = 0.059341194567807204;
pub const EARTH_INCLINATION: f32 = 0.12487830798019428;
pub const MOON_INCLINATION: f32 = 0.08979719001510825;
pub const MARS_INCLINATION: f32 = 0.0322885911618951;
pub const CERES_INCLINATION: f32 = 0.1684242728174528;
pub const JUPITER_INCLINATION: f32 = 0.10629055144645466;
pub const SATURN_INCLINATION: f32 = 0.09616764178488756;
pub const URANUS_INCLINATION: f32 = 0.11309733552923257;
pub const NEPTUNE_INCLINATION: f32 = 0.11222467090323539;
pub const PLUTO_INCLINATION: f32 = 0.20734511513692636;
