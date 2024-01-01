use super::orbital_parameters::OrbitalParameters;
use astro_utils::{
    color::Color,
    coordinates::direction::Direction,
    units::{length::Length, mass::Mass, time::Time},
    Float,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PlanetData {
    name: String,
    mass: Mass,
    radius: Length,
    albedo: Float,
    color: Color,
    orbital_parameters: OrbitalParameters,
    sideral_rotation_period: Time,
    rotation_axis: Direction,
}

impl PartialEq for PlanetData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PlanetData {
    pub(crate) fn new(
        name: String,
        mass: Mass,
        orbital_parameters: OrbitalParameters,
        radius: Length,
        albedo: Float,
        color: Color,
        sideral_rotation_period: Time,
        rotation_axis: Direction,
    ) -> Self {
        PlanetData {
            name,
            mass,
            orbital_parameters,
            radius,
            albedo,
            color,
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

    pub(crate) fn get_color(&self) -> &Color {
        &self.color
    }

    pub(crate) fn get_orbital_parameters(&self) -> &OrbitalParameters {
        &self.orbital_parameters
    }

    pub(crate) fn get_sideral_rotation_period(&self) -> Time {
        self.sideral_rotation_period
    }

    pub(crate) fn get_rotation_axis(&self) -> &Direction {
        &self.rotation_axis
    }
}
