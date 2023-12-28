use astro_utils::{length::Length, mass::Mass, time::Time, Float};

use super::{
    coordinates::CartesianCoordinates, orbital_parameters::OrbitalParameters,
    rotation_parameters::RotationParameters,
};

#[derive(Debug, Clone)]
pub(crate) struct CelestialBodyData {
    name: String,
    mass: Mass,
    radius: Length,
    albedo: Float,
    orbital_parameters: OrbitalParameters,
    rotation_parameters: RotationParameters,
    orbiting_bodies: Vec<CelestialBodyData>,
}

pub(crate) struct CelestialBody {
    data: CelestialBodyData,
    position: CartesianCoordinates,
}

impl CelestialBodyData {
    pub(crate) fn new(
        name: String,
        mass: Mass,
        orbital_parameters: OrbitalParameters,
        rotation_parameters: RotationParameters,
        radius: Length,
        albedo: Float,
    ) -> Self {
        CelestialBodyData {
            name,
            mass,
            orbital_parameters,
            rotation_parameters,
            radius,
            albedo,
            orbiting_bodies: Vec::new(),
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

    pub(crate) fn add_orbiting_body(&mut self, body_data: CelestialBodyData) {
        self.orbiting_bodies.push(body_data);
    }
}

impl CelestialBody {
    pub(crate) fn new(data: CelestialBodyData, central_body: Option<Self>, time: Time) -> Self {
        let position = match central_body {
            Some(central_body) => data
                .orbital_parameters
                .calculate_position(&central_body, time),
            None => CartesianCoordinates::zero(),
        };
        CelestialBody { data, position }
    }

    pub(crate) fn get_data(&self) -> &CelestialBodyData {
        &self.data
    }

    pub(crate) fn get_orbital_parameters(&self) -> &OrbitalParameters {
        &self.data.get_orbital_parameters()
    }

    pub(crate) fn get_semi_major_axis(&self) -> Length {
        self.data.get_semi_major_axis()
    }

    pub(crate) fn get_mass(&self) -> Mass {
        self.data.get_mass()
    }
}
