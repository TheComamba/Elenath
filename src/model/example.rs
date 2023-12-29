use super::celestial_system::CelestialSystem;
use crate::model::{celestial_body_data::CelestialBodyData, orbital_parameters::OrbitalParameters};
use astro_utils::{
    orbit_orientation::OrbitOrientation,
    solar_system_data::{
        EARTH_MASS, EARTH_RADIUS, EARTH_SEMI_MAJOR_AXIS, JUPITER_SEMI_MAJOR_AXIS, MOON_MASS,
        MOON_RADIUS, MOON_SEMI_MAJOR_AXIS, SUN_MASS, SUN_RADIUS,
    },
    units::{angle::Angle, length::Length, mass::Mass},
};

fn sun() -> CelestialBodyData {
    let mut sun_data = CelestialBodyData::new(
        String::from("Sun"),
        SUN_MASS,
        OrbitalParameters::central(),
        SUN_RADIUS,
        1.0,
    );
    sun_data.add_orbiting_body(&"Earth".to_string());
    sun_data.add_orbiting_body(&"Jupiter".to_string());
    sun_data
}

fn earth() -> CelestialBodyData {
    let mut earth_data = CelestialBodyData::new(
        String::from("Earth"),
        EARTH_MASS,
        OrbitalParameters::new(
            EARTH_SEMI_MAJOR_AXIS,
            0.0167086,
            OrbitOrientation::new(
                Angle::from_degrees(0.0),
                Angle::from_degrees(-11.26064),
                Angle::from_degrees(114.20783),
            ),
        ),
        EARTH_RADIUS,
        1.0,
    );
    earth_data.add_orbiting_body(&"Moon".to_string());
    earth_data
}

fn jupiter() -> CelestialBodyData {
    CelestialBodyData::new(
        String::from("Jupiter"),
        Mass::from_jupiter_masses(1.0),
        OrbitalParameters::new(
            JUPITER_SEMI_MAJOR_AXIS,
            0.0489,
            OrbitOrientation::new(
                Angle::from_degrees(1.303),
                Angle::from_degrees(100.464),
                Angle::from_degrees(273.867),
            ),
        ),
        Length::from_earth_radii(10.97),
        1.0,
    )
}

fn moon() -> CelestialBodyData {
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
    CelestialSystem::new(sun())
    //vec![sun(), earth(), moon(), jupiter()]
}
