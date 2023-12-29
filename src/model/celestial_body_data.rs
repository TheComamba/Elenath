use super::orbital_parameters::OrbitalParameters;
use astro_utils::{
    units::{length::Length, mass::Mass},
    Float,
};

#[derive(Debug, Clone)]
pub(crate) struct CelestialBodyData {
    name: String,
    mass: Mass,
    radius: Length,
    albedo: Float,
    orbital_parameters: OrbitalParameters,
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
    ) -> Self {
        CelestialBodyData {
            name,
            mass,
            orbital_parameters,
            radius,
            albedo,
        }
    }

    pub(crate) fn get_orbital_parameters(&self) -> &OrbitalParameters {
        &self.orbital_parameters
    }

    pub(crate) fn get_mass(&self) -> Mass {
        self.mass
    }

    pub(crate) fn get_semi_major_axis(&self) -> Length {
        self.orbital_parameters.get_semi_major_axis()
    }

    pub(crate) fn get_name(&self) -> &String {
        &self.name
    }
}
