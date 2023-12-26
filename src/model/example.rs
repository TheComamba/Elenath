use crate::model::{
    celestial_body::CelestialBodyData, orbital_parameters::OrbitalParameters,
    rotation_parameters::RotationParameters,
};

pub(crate) fn solar_system_example() -> Vec<CelestialBodyData> {
    vec![
        CelestialBodyData::new(
            String::from("Sun"),
            OrbitalParameters::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            RotationParameters::new(0.0, 0.0, 0.0),
            10.0,
            1.0,
        ),
        CelestialBodyData::new(
            String::from("Earth"),
            OrbitalParameters::new(1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            RotationParameters::new(0.0, 0.0, 0.0),
            1.0,
            1.0,
        ),
    ]
}
