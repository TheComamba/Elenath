use astro_utils::{distance::Distance, time::Time};

use crate::model::{
    celestial_body::CelestialBodyData, orbital_parameters::OrbitalParameters,
    rotation_parameters::RotationParameters,
};

pub(crate) fn solar_system_example() -> Vec<CelestialBodyData> {
    vec![
        CelestialBodyData::new(
            String::from("Sun"),
            OrbitalParameters::new(
                Distance::from_astronomical_units(0.0),
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ),
            RotationParameters::new(0.0, Time::from_days(0.0), 0.0),
            Distance::from_sun_radii(1.0),
            1.0,
        ),
        CelestialBodyData::new(
            String::from("Earth"),
            OrbitalParameters::new(
                Distance::from_astronomical_units(1.0),
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ),
            RotationParameters::new(0.0, Time::from_days(1.0), 0.0),
            Distance::from_earth_radii(1.0),
            1.0,
        ),
    ]
}
