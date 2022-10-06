use nannou::color::named::*;
use nannou::color::rgb::Srgb;

#[derive(Clone, Copy)]
pub struct Planet {
    pub orbit_radius: f32,
    pub planet_radius: f32,
    pub year_in_days: f32,
    pub color: Srgb<u8>
}

static MERCURY: Planet = Planet {
    orbit_radius: 57909050.0, // 57 909 050
    planet_radius: 2439.7, // 2 439,7
    year_in_days: 87.969,
    color: GREY
};

static VENUS: Planet = Planet {
    orbit_radius: 108209500.0, // 108 209 500
    planet_radius: 6051.8, // 6 051,8
    year_in_days: 224.667,
    color: BEIGE
};

static EARTH: Planet = Planet {
    orbit_radius: 149597887.0, // 149 597 887
    planet_radius: 6378.137, // 6 378,137
    year_in_days: 365.256,
    color: STEELBLUE
};

static MARS: Planet = Planet {
    orbit_radius: 227944000.0, // 227 944 000
    planet_radius: 3396.2, //3 396,2
    year_in_days: 686.885,
    color: BROWN
};

static JUPITER: Planet = Planet {
    orbit_radius: 778340000.0, // 778 340 000
    planet_radius: 71492.0, // 71 492 
    year_in_days: 4332.01,
    color: DARKSALMON
};

static SATURN: Planet = Planet {
    orbit_radius: 1426700000.0, // 1 426 700 000
    planet_radius: 60268.0, // 60 268
    year_in_days:  10754.0,
    color: KHAKI
};

static URANUS: Planet = Planet {
    orbit_radius: 2870700000.0, // 2 870 700 000
    planet_radius: 25559.0, // 25 559
    year_in_days: 30698.0,
    color: LIGHTCYAN
};

static NEPTUNE: Planet = Planet {
    orbit_radius: 4498400000.0, // 4 498 400 000
    planet_radius: 24764.0, // 24 764
    year_in_days: 60216.8,
    color: CORNFLOWERBLUE
};



pub static PLANETS: [Planet; 8] = [NEPTUNE, URANUS, SATURN, JUPITER, MARS, EARTH, VENUS, MERCURY];
pub static MAX_ORBIT: f32 = PLANETS[0].orbit_radius;
pub static SMALLEST_PLANET: f32 = MERCURY.planet_radius;
