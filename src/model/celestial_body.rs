use std::fmt::{Display, Formatter};

use astro_utils::{
    coordinates::cartesian::{CartesianCoordinates, ORIGIN},
    units::{length::Length, mass::Mass, time::Time},
    Float,
};

use super::orbital_parameters::OrbitalParameters;

#[derive(Debug, Clone)]
pub(crate) struct CelestialBodyData {
    name: String,
    mass: Mass,
    radius: Length,
    albedo: Float,
    orbital_parameters: OrbitalParameters,
    orbiting_bodies: Vec<CelestialBodyData>,
}

impl PartialEq for CelestialBodyData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CelestialBody {
    data: CelestialBodyData,
    position: CartesianCoordinates,
}

impl PartialEq for CelestialBody {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for CelestialBody {}

impl Display for CelestialBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data.name)
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

    pub(crate) fn system(&self, time: Time) -> Vec<CelestialBody> {
        let mut system = Vec::new();
        let central_body = CelestialBody::new(self.clone(), None, time);
        for body_data in &self.orbiting_bodies {
            system.push(CelestialBody::new(
                body_data.clone(),
                Some(&central_body),
                time,
            ));
        }
        system.push(central_body);
        system
    }
}

impl CelestialBody {
    pub(crate) fn new(data: CelestialBodyData, central_body: Option<&Self>, time: Time) -> Self {
        let position = match central_body {
            Some(central_body) => {
                data.orbital_parameters
                    .calculate_position(data.get_mass(), &central_body, time)
            }
            None => ORIGIN,
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

    pub(crate) fn get_position(&self) -> CartesianCoordinates {
        self.position
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.data.name
    }
}
