use super::{celestial_system::CelestialSystem, distant_star::DistantStar};
use crate::model::{orbital_parameters::OrbitalParameters, planet_data::PlanetData};
use astro_utils::{
    coordinates::earth_equatorial::EarthEquatorialCoordinates,
    data::{planets::*, stars::*},
};

fn mercury() -> PlanetData {
    PlanetData::new(
        String::from("Mercury"),
        MERCURY_MASS,
        OrbitalParameters::new(
            MERCURY_SEMI_MAJOR_AXIS,
            MERCURY_ECCENTRICITY,
            MERCURY_ORBIT_ORIENTATION,
        ),
        MERCURY_RADIUS,
        MERCURY_GEOMETRIC_ALBEDO,
        MERCURY_COLOR,
        MERCURY_SIDERIAL_ROTATION_PERIOD,
        MERCURY_NORTH.to_direction(),
    )
}

fn venus() -> PlanetData {
    PlanetData::new(
        String::from("Venus"),
        VENUS_MASS,
        OrbitalParameters::new(
            VENUS_SEMI_MAJOR_AXIS,
            VENUS_ECCENTRICITY,
            VENUS_ORBIT_ORIENTATION,
        ),
        VENUS_RADIUS,
        VENUS_GEOMETRIC_ALBEDO,
        VENUS_COLOR,
        VENUS_SIDERIAL_ROTATION_PERIOD,
        VENUS_NORTH.to_direction(),
    )
}

fn earth() -> PlanetData {
    PlanetData::new(
        String::from("Earth"),
        EARTH_MASS,
        OrbitalParameters::new(
            EARTH_SEMI_MAJOR_AXIS,
            EARTH_ECCENTRICITY,
            EARTH_ORBIT_ORIENTATION,
        ),
        EARTH_RADIUS,
        EARTH_GEOMETRIC_ALBEDO,
        EARTH_COLOR,
        EARTH_SIDERIAL_ROTATION_PERIOD,
        EARTH_NORTH.to_direction(),
    )
}

fn mars() -> PlanetData {
    PlanetData::new(
        String::from("Mars"),
        MARS_MASS,
        OrbitalParameters::new(
            MARS_SEMI_MAJOR_AXIS,
            MARS_ECCENTRICITY,
            MARS_ORBIT_ORIENTATION,
        ),
        MARS_RADIUS,
        MARS_GEOMETRIC_ALBEDO,
        MARS_COLOR,
        MARS_SIDERIAL_ROTATION_PERIOD,
        MARS_NORTH.to_direction(),
    )
}

fn ceres() -> PlanetData {
    PlanetData::new(
        String::from("Ceres"),
        CERES_MASS,
        OrbitalParameters::new(
            CERES_SEMI_MAJOR_AXIS,
            CERES_ECCENTRICITY,
            CERES_ORBIT_ORIENTATION,
        ),
        CERES_RADIUS,
        CERES_GEOMETRIC_ALBEDO,
        CERES_COLOR,
        CERES_SIDERIAL_ROTATION_PERIOD,
        CERES_NORTH.to_direction(),
    )
}

fn jupiter() -> PlanetData {
    PlanetData::new(
        String::from("Jupiter"),
        JUPITER_MASS,
        OrbitalParameters::new(
            JUPITER_SEMI_MAJOR_AXIS,
            JUPITER_ECCENTRICITY,
            JUPITER_ORBIT_ORIENTATION,
        ),
        JUPITER_RADIUS,
        JUPITER_GEOMETRIC_ALBEDO,
        JUPITER_COLOR,
        JUPITER_SIDERIAL_ROTATION_PERIOD,
        JUPITER_NORTH.to_direction(),
    )
}

fn saturn() -> PlanetData {
    PlanetData::new(
        String::from("Saturn"),
        SATURN_MASS,
        OrbitalParameters::new(
            SATURN_SEMI_MAJOR_AXIS,
            SATURN_ECCENTRICITY,
            SATURN_ORBIT_ORIENTATION,
        ),
        SATURN_RADIUS,
        SATURN_GEOMETRIC_ALBEDO,
        SATURN_COLOR,
        SATURN_SIDERIAL_ROTATION_PERIOD,
        SATURN_NORTH.to_direction(),
    )
}

fn uranus() -> PlanetData {
    PlanetData::new(
        String::from("Uranus"),
        URANUS_MASS,
        OrbitalParameters::new(
            URANUS_SEMI_MAJOR_AXIS,
            URANUS_ECCENTRICITY,
            URANUS_ORBIT_ORIENTATION,
        ),
        URANUS_RADIUS,
        URANUS_GEOMETRIC_ALBEDO,
        URANUS_COLOR,
        URANUS_SIDERIAL_ROTATION_PERIOD,
        URANUS_NORTH.to_direction(),
    )
}

fn neptune() -> PlanetData {
    PlanetData::new(
        String::from("Neptune"),
        NEPTUNE_MASS,
        OrbitalParameters::new(
            NEPTUNE_SEMI_MAJOR_AXIS,
            NEPTUNE_ECCENTRICITY,
            NEPTUNE_ORBIT_ORIENTATION,
        ),
        NEPTUNE_RADIUS,
        NEPTUNE_GEOMETRIC_ALBEDO,
        NEPTUNE_COLOR,
        NEPTUNE_SIDERIAL_ROTATION_PERIOD,
        NEPTUNE_NORTH.to_direction(),
    )
}

fn pluto() -> PlanetData {
    PlanetData::new(
        String::from("Pluto"),
        PLUTO_MASS,
        OrbitalParameters::new(
            PLUTO_SEMI_MAJOR_AXIS,
            PLUTO_ECCENTRICITY,
            PLUTO_ORBIT_ORIENTATION,
        ),
        PLUTO_RADIUS,
        PLUTO_GEOMETRIC_ALBEDO,
        PLUTO_COLOR,
        PLUTO_SIDERIAL_ROTATION_PERIOD,
        PLUTO_NORTH.to_direction(),
    )
}

fn _moon() -> PlanetData {
    PlanetData::new(
        String::from("Moon"),
        MOON_MASS,
        OrbitalParameters::new(
            MOON_SEMI_MAJOR_AXIS,
            MOON_ECCENTRICITY,
            MOON_ORBIT_ORIENTATION,
        ),
        MOON_RADIUS,
        MOON_GEOMETRIC_ALBEDO,
        MOON_COLOR,
        MOON_SIDERIAL_ROTATION_PERIOD,
        MOON_NORTH.to_direction(),
    )
}

pub(crate) fn solar_system() -> CelestialSystem {
    let mut system = CelestialSystem::new(SUN_PROPERTIES);
    system.add_planet(mercury());
    system.add_planet(venus());
    system.add_planet(earth());
    system.add_planet(mars());
    system.add_planet(ceres());
    system.add_planet(jupiter());
    system.add_planet(saturn());
    system.add_planet(uranus());
    system.add_planet(neptune());
    system.add_planet(pluto());

    for (props, ra, dec, dist) in STARS_TO_TWO_POINT_FIVE_APPARENT_MAG {
        let ra = ra.to_angle();
        let dec = dec.to_angle();
        let pos = EarthEquatorialCoordinates::new(ra, dec);
        system.add_distant_star(DistantStar::new(props, pos.to_direction(), dist));
    }

    system
}
