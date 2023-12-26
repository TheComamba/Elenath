use super::{orbital_parameters::OrbitalParameters, rotation_parameters::RotationParameters};

pub(super) struct CelestialBody {
    name: String,
    orbital_parameters: OrbitalParameters,
    rotation_parameters: RotationParameters,
    radius: f32,
    albedo: f32,
}
