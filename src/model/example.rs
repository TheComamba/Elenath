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
            MERCURY_ORBIT_ORIENTATION,
        ),
        MERCURY_RADIUS,
        MERCURY_BOND_ALBEDO,
    )
}

fn venus() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Venus"),
        VENUS_MASS,
        OrbitalParameters::new(
            VENUS_SEMI_MAJOR_AXIS,
            VENUS_ECCENTRICITY,
            VENUS_ORBIT_ORIENTATION,
        ),
        VENUS_RADIUS,
        VENUS_BOND_ALBEDO,
    )
}

fn earth() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Earth"),
        EARTH_MASS,
        OrbitalParameters::new(
            EARTH_SEMI_MAJOR_AXIS,
            EARTH_ECCENTRICITY,
            EARTH_ORBIT_ORIENTATION,
        ),
        EARTH_RADIUS,
        EARTH_BOND_ALBEDO,
    )
}

fn mars() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Mars"),
        MARS_MASS,
        OrbitalParameters::new(
            MARS_SEMI_MAJOR_AXIS,
            MARS_ECCENTRICITY,
            MARS_ORBIT_ORIENTATION,
        ),
        MARS_RADIUS,
        MARS_BOND_ALBEDO,
    )
}

fn ceres() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Ceres"),
        CERES_MASS,
        OrbitalParameters::new(
            CERES_SEMI_MAJOR_AXIS,
            CERES_ECCENTRICITY,
            CERES_ORBIT_ORIENTATION,
        ),
        CERES_RADIUS,
        CERES_BOND_ALBEDO,
    )
}

fn jupiter() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Jupiter"),
        JUPITER_MASS,
        OrbitalParameters::new(
            JUPITER_SEMI_MAJOR_AXIS,
            JUPITER_ECCENTRICITY,
            JUPITER_ORBIT_ORIENTATION,
        ),
        JUPITER_RADIUS,
        JUPITER_BOND_ALBEDO,
    )
}

fn saturn() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Saturn"),
        SATURN_MASS,
        OrbitalParameters::new(
            SATURN_SEMI_MAJOR_AXIS,
            SATURN_ECCENTRICITY,
            SATURN_ORBIT_ORIENTATION,
        ),
        SATURN_RADIUS,
        SATURN_BOND_ALBEDO,
    )
}

fn uranus() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Uranus"),
        URANUS_MASS,
        OrbitalParameters::new(
            URANUS_SEMI_MAJOR_AXIS,
            URANUS_ECCENTRICITY,
            URANUS_ORBIT_ORIENTATION,
        ),
        URANUS_RADIUS,
        URANUS_BOND_ALBEDO,
    )
}

fn neptune() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Neptune"),
        NEPTUNE_MASS,
        OrbitalParameters::new(
            NEPTUNE_SEMI_MAJOR_AXIS,
            NEPTUNE_ECCENTRICITY,
            NEPTUNE_ORBIT_ORIENTATION,
        ),
        NEPTUNE_RADIUS,
        NEPTUNE_BOND_ALBEDO,
    )
}

fn pluto() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Pluto"),
        PLUTO_MASS,
        OrbitalParameters::new(
            PLUTO_SEMI_MAJOR_AXIS,
            PLUTO_ECCENTRICITY,
            PLUTO_ORBIT_ORIENTATION,
        ),
        PLUTO_RADIUS,
        PLUTO_BOND_ALBEDO,
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
