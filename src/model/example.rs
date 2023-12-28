use super::celestial_body::CelestialBody;
use crate::model::{
    celestial_body::CelestialBodyData, orbital_parameters::OrbitalParameters,
    rotation_parameters::RotationParameters,
};
use astro_utils::{length::Length, mass::Mass, time::Time};

pub(crate) fn sun() -> CelestialBody {
    let sun_data: CelestialBodyData = CelestialBodyData::new(
        String::from("Sun"),
        Mass::from_solar_masses(1.0),
        OrbitalParameters::central(),
        RotationParameters::new(0.0, Time::from_days(0.0), 0.0),
        Length::from_sun_radii(1.0),
        1.0,
    );

    CelestialBody::new(sun_data, None, Time::from_days(0.0))
}

pub(crate) fn earth() -> CelestialBody {
    let earth_data: CelestialBodyData = CelestialBodyData::new(
        String::from("Earth"),
        Mass::from_earth_masses(1.0),
        OrbitalParameters::new(
            Length::from_kilometers(149598023.),
            0.0167086,
            0.0,
            -11.26064, /*degrees */
            114.20783, /*degrees */
        ),
        RotationParameters::new(0.0, Time::from_days(1.0), 0.0),
        Length::from_earth_radii(1.0),
        1.0,
    );

    CelestialBody::new(earth_data, Some(sun()), Time::from_days(0.0))
}

pub(crate) fn jupiter() -> CelestialBody {
    let jupiter_data: CelestialBodyData = CelestialBodyData::new(
        String::from("Jupiter"),
        Mass::from_jupiter_masses(1.0),
        OrbitalParameters::new(
            Length::from_astronomical_units(5.2038),
            0.0489,
            1.303,   /*Degrees */
            100.464, /*degrees */
            273.867, /*degrees */
        ),
        RotationParameters::new(0.0, Time::from_days(0.41354), 0.0),
        Length::from_earth_radii(10.97),
        1.0,
    );

    CelestialBody::new(jupiter_data, Some(sun()), Time::from_days(0.0))
}

pub(crate) fn moon() -> CelestialBody {
    let moon_data: CelestialBodyData = CelestialBodyData::new(
        String::from("Moon"),
        Mass::from_earth_masses(0.0123),
        OrbitalParameters::new(
            Length::from_kilometers(384399.0),
            0.0549,
            5.145, /*degrees */
            0.0,
            0.0,
        ),
        RotationParameters::new(0.0, Time::from_days(27.321), 0.0),
        Length::from_earth_radii(0.273),
        1.0,
    );

    CelestialBody::new(moon_data, Some(earth()), Time::from_days(0.0))
}

pub(crate) fn solar_system_example() -> Vec<CelestialBodyData> {
    vec![]
}
