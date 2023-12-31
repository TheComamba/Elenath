use super::orbital_parameters::OrbitalParameters;
use astro_utils::{
    coordinates::direction::Direction,
    units::{length::Length, mass::Mass, time::Time},
    Float,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CelestialBodyData {
    name: String,
    mass: Mass,
    radius: Length,
    albedo: Float,
    orbital_parameters: OrbitalParameters,
    sideral_rotation_period: Time,
    rotation_axis: Direction,
}

impl PartialEq for CelestialBodyData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl CelestialBodyData {
    pub(crate) fn new(
        name: String,
        mass: Mass,
        orbital_parameters: OrbitalParameters,
        radius: Length,
        albedo: Float,
        sideral_rotation_period: Time,
        rotation_axis: Direction,
    ) -> Self {
        CelestialBodyData {
            name,
            mass,
            orbital_parameters,
            radius,
            albedo,
            sideral_rotation_period,
            rotation_axis,
        }
    }

    pub(crate) fn get_name(&self) -> &String {
        &self.name
    }

    pub(crate) fn get_mass(&self) -> Mass {
        self.mass
    }

    pub(crate) fn get_radius(&self) -> Length {
        self.radius
    }

    pub(crate) fn get_albedo(&self) -> Float {
        self.albedo
    }

    pub(crate) fn get_orbital_parameters(&self) -> &OrbitalParameters {
        &self.orbital_parameters
    }
}
