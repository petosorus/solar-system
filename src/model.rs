use nannou::color::named::*;
use nannou::color::rgb::Srgb;

#[derive(Clone, Copy)]
pub struct Planet {
    pub orbit_radius: f32,
    pub planet_radius: f32,
    pub year_in_days: f32,
    pub color: Srgb<u8>
}

pub static MERCURY: Planet = Planet {
    orbit_radius: 80.0, // 57 909 050
    planet_radius: 10.0, // 2 439,7
    year_in_days: 87.969,
    color: GREY
};

pub static VENUS: Planet = Planet {
    orbit_radius: 200.0, // 108 209 500
    planet_radius: 25.0, // 6 051,8
    year_in_days: 224.667,
    color: BEIGE
};

pub static EARTH: Planet = Planet {
    orbit_radius: 300.0, // 149 597 887
    planet_radius: 26.0, // 6 378,137
    year_in_days: 365.256,
    color: STEELBLUE
};

pub static PLANETS: [Planet; 3] = [MERCURY, VENUS, EARTH];