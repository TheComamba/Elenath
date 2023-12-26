use super::{
    coordinates::CartesianCoordinates, orbital_parameters::OrbitalParameters,
    rotation_parameters::RotationParameters,
};

#[derive(Debug, Clone)]
pub(crate) struct CelestialBodyData {
    name: String,
    orbital_parameters: OrbitalParameters,
    rotation_parameters: RotationParameters,
    radius: f32,
    albedo: f32,
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
        radius: f32,
        albedo: f32,
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
    pub(crate) fn new(data: CelestialBodyData, time: f64) -> Self {
        let position = data.orbital_parameters.current_position(time);
        CelestialBody { data, position }
    }
}
