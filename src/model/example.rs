use crate::model::{
    celestial_body::CelestialBodyData, orbital_parameters::OrbitalParameters,
    rotation_parameters::RotationParameters,
};
use astro_utils::{
    orbit_orientation::OrbitOrientation,
    solar_system_data::{
        EARTH_MASS, EARTH_RADIUS, EARTH_SEMI_MAJOR_AXIS, JUPITER_SEMI_MAJOR_AXIS, MOON_MASS,
        MOON_RADIUS, MOON_SEMI_MAJOR_AXIS, SUN_MASS, SUN_RADIUS,
    },
    units::{angle::Angle, length::Length, mass::Mass, time::Time},
};

pub(crate) fn sun() -> CelestialBodyData {
    let mut sun_data = CelestialBodyData::new(
        String::from("Sun"),
        SUN_MASS,
        OrbitalParameters::central(),
        RotationParameters::new(
            Angle::from_degrees(0.0),
            Time::from_days(0.0),
            Angle::from_degrees(0.0),
        ),
        SUN_RADIUS,
        1.0,
    );
    sun_data.add_orbiting_body(earth());
    sun_data.add_orbiting_body(jupiter());
    sun_data
}

pub(crate) fn earth() -> CelestialBodyData {
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
        RotationParameters::new(
            Angle::from_degrees(0.0),
            Time::from_days(1.0),
            Angle::from_degrees(0.0),
        ),
        EARTH_RADIUS,
        1.0,
    );
    earth_data.add_orbiting_body(moon());
    earth_data
}

pub(crate) fn jupiter() -> CelestialBodyData {
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
        RotationParameters::new(
            Angle::from_degrees(0.0),
            Time::from_days(0.41354),
            Angle::from_degrees(0.0),
        ),
        Length::from_earth_radii(10.97),
        1.0,
    )
}

pub(crate) fn moon() -> CelestialBodyData {
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
        RotationParameters::new(
            Angle::from_degrees(0.0),
            Time::from_days(27.321),
            Angle::from_degrees(0.0),
        ),
        MOON_RADIUS,
        1.0,
    )
}
