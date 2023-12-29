use super::celestial_system::CelestialSystem;
use crate::model::{celestial_body_data::CelestialBodyData, orbital_parameters::OrbitalParameters};
use astro_utils::{orbit_orientation::OrbitOrientation, solar_system_data::*, units::angle::Angle};

fn sun() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Sun"),
        SUN_MASS,
        OrbitalParameters::central(),
        SUN_RADIUS,
        1.0,
    )
}

fn mercury() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Mercury"),
        MERCURY_MASS,
        OrbitalParameters::new(
            MERCURY_SEMI_MAJOR_AXIS,
            MERCURY_ECCENTRICITY,
            OrbitOrientation::new(
                MERCURY_INCLINATION,
                MERCURY_LONGITUDE_OF_ASCENDING_NODE,
                MERCURY_ARGUMENT_OF_PERIAPSIS,
            ),
        ),
        MERCURY_RADIUS,
        MERCURY_ALBEDO,
    )
}

fn venus() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Venus"),
        VENUS_MASS,
        OrbitalParameters::new(
            VENUS_SEMI_MAJOR_AXIS,
            VENUS_ECCENTRICITY,
            OrbitOrientation::new(
                VENUS_INCLINATION,
                VENUS_LONGITUDE_OF_ASCENDING_NODE,
                VENUS_ARGUMENT_OF_PERIAPSIS,
            ),
        ),
        VENUS_RADIUS,
        VENUS_ALBEDO,
    )
}

fn earth() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Earth"),
        EARTH_MASS,
        OrbitalParameters::new(
            EARTH_SEMI_MAJOR_AXIS,
            EARTH_ECCENTRICITY,
            OrbitOrientation::new(
                EARTH_INCLINATION,
                EARTH_LONGITUDE_OF_ASCENDING_NODE,
                EARTH_ARGUMENT_OF_PERIAPSIS,
            ),
        ),
        EARTH_RADIUS,
        EARTH_ALBEDO,
    )
}

fn mars() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Mars"),
        MARS_MASS,
        OrbitalParameters::new(
            MARS_SEMI_MAJOR_AXIS,
            MARS_ECCENTRICITY,
            OrbitOrientation::new(
                MARS_INCLINATION,
                MARS_LONGITUDE_OF_ASCENDING_NODE,
                MARS_ARGUMENT_OF_PERIAPSIS,
            ),
        ),
        MARS_RADIUS,
        MARS_ALBEDO,
    )
}

fn ceres() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Ceres"),
        CERES_MASS,
        OrbitalParameters::new(
            CERES_SEMI_MAJOR_AXIS,
            CERES_ECCENTRICITY,
            OrbitOrientation::new(
                CERES_INCLINATION,
                CERES_LONGITUDE_OF_ASCENDING_NODE,
                CERES_ARGUMENT_OF_PERIAPSIS,
            ),
        ),
        CERES_RADIUS,
        CERES_ALBEDO,
    )
}

fn jupiter() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Jupiter"),
        JUPITER_MASS,
        OrbitalParameters::new(
            JUPITER_SEMI_MAJOR_AXIS,
            JUPITER_ECCENTRICITY,
            OrbitOrientation::new(
                JUPITER_INCLINATION,
                JUPITER_LONGITUDE_OF_ASCENDING_NODE,
                JUPITER_ARGUMENT_OF_PERIAPSIS,
            ),
        ),
        JUPITER_RADIUS,
        JUPITER_ALBEDO,
    )
}

fn saturn() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Saturn"),
        SATURN_MASS,
        OrbitalParameters::new(
            SATURN_SEMI_MAJOR_AXIS,
            SATURN_ECCENTRICITY,
            OrbitOrientation::new(
                SATURN_INCLINATION,
                SATURN_LONGITUDE_OF_ASCENDING_NODE,
                SATURN_ARGUMENT_OF_PERIAPSIS,
            ),
        ),
        SATURN_RADIUS,
        SATURN_ALBEDO,
    )
}

fn uranus() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Uranus"),
        URANUS_MASS,
        OrbitalParameters::new(
            URANUS_SEMI_MAJOR_AXIS,
            URANUS_ECCENTRICITY,
            OrbitOrientation::new(
                URANUS_INCLINATION,
                URANUS_LONGITUDE_OF_ASCENDING_NODE,
                URANUS_ARGUMENT_OF_PERIAPSIS,
            ),
        ),
        URANUS_RADIUS,
        URANUS_ALBEDO,
    )
}

fn neptune() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Neptune"),
        NEPTUNE_MASS,
        OrbitalParameters::new(
            NEPTUNE_SEMI_MAJOR_AXIS,
            NEPTUNE_ECCENTRICITY,
            OrbitOrientation::new(
                NEPTUNE_INCLINATION,
                NEPTUNE_LONGITUDE_OF_ASCENDING_NODE,
                NEPTUNE_ARGUMENT_OF_PERIAPSIS,
            ),
        ),
        NEPTUNE_RADIUS,
        NEPTUNE_ALBEDO,
    )
}

fn pluto() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Pluto"),
        PLUTO_MASS,
        OrbitalParameters::new(
            PLUTO_SEMI_MAJOR_AXIS,
            PLUTO_ECCENTRICITY,
            OrbitOrientation::new(
                PLUTO_INCLINATION,
                PLUTO_LONGITUDE_OF_ASCENDING_NODE,
                PLUTO_ARGUMENT_OF_PERIAPSIS,
            ),
        ),
        PLUTO_RADIUS,
        PLUTO_ALBEDO,
    )
}

fn _moon() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Moon"),
        MOON_MASS,
        OrbitalParameters::new(
            MOON_SEMI_MAJOR_AXIS,
            0.0549,
            OrbitOrientation::new(
                Angle::from_degrees(5.145),
                Angle::from_degrees(0.0),
                Angle::from_degrees(0.0),
            ),
        ),
        MOON_RADIUS,
        1.0,
    )
}
pub(crate) fn solar_system() -> CelestialSystem {
    let mut system = CelestialSystem::new(sun());
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
    system
}
