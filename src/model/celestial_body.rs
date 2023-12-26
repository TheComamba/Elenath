use super::{orbital_parameters::OrbitalParameters, rotation_parameters::RotationParameters};

pub(crate) struct CelestialBody {
    name: String,
    orbital_parameters: OrbitalParameters,
    rotation_parameters: RotationParameters,
    radius: f32,
    albedo: f32,
}

impl CelestialBody {
    pub(crate) fn new(
        name: String,
        orbital_parameters: OrbitalParameters,
        rotation_parameters: RotationParameters,
        radius: f32,
        albedo: f32,
    ) -> Self {
        CelestialBody {
            name,
            orbital_parameters,
            rotation_parameters,
            radius,
            albedo,
        }
    }
}
