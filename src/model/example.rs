use astro_utils::{distance::Distance, mass::Mass, time::Time};

use crate::model::{
    celestial_body::CelestialBodyData, orbital_parameters::OrbitalParameters,
    rotation_parameters::RotationParameters,
};

pub(crate) fn solar_system_example() -> Vec<CelestialBodyData> {
    let sun = CelestialBodyData::new(
        String::from("Sun"),
        Mass::from_solar_masses(1.0),
        OrbitalParameters::central(),
        RotationParameters::new(0.0, Time::from_days(0.0), 0.0),
        Distance::from_sun_radii(1.0),
        1.0,
    );
    let earth = CelestialBodyData::new(
        String::from("Earth"),
        Mass::from_earth_masses(1.0),
        OrbitalParameters::new(Distance::from_astronomical_units(1.0), 0.0, 0.0, 0.0, 0.0),
        RotationParameters::new(0.0, Time::from_days(1.0), 0.0),
        Distance::from_earth_radii(1.0),
        1.0,
    );
    vec![sun, earth]
}
