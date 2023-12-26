use astro_utils::{distance::Distance, time::Time, Float};

use super::{
    coordinates::CartesianCoordinates, orbital_parameters::OrbitalParameters,
    rotation_parameters::RotationParameters,
};

#[derive(Debug, Clone)]
pub(crate) struct CelestialBodyData {
    name: String,
    orbital_parameters: OrbitalParameters,
    rotation_parameters: RotationParameters,
    radius: Distance,
    albedo: Float,
}

pub(crate) struct CelestialBody {
    data: CelestialBodyData,
    position: CartesianCoordinates,
}

impl CelestialBodyData {
    pub(crate) fn new(
        name: String,
        orbital_parameters: OrbitalParameters,
        rotation_parameters: RotationParameters,
        radius: Distance,
        albedo: Float,
    ) -> Self {
        CelestialBodyData {
            name,
            orbital_parameters,
            rotation_parameters,
            radius,
            albedo,
        }
    }
}

impl CelestialBody {
    pub(crate) fn new(data: CelestialBodyData, time: Time) -> Self {
        let position = data.orbital_parameters.current_position(time);
        CelestialBody { data, position }
    }
}
