use super::{celestial_body_data::CelestialBodyData, orbital_parameters::OrbitalParameters};
use astro_utils::{
    coordinates::cartesian::{CartesianCoordinates, ORIGIN},
    units::{length::Length, mass::Mass, time::Time},
};
use std::fmt::{Display, Formatter};

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
        write!(f, "{}", self.data.get_name())
    }
}

impl CelestialBody {
    pub(crate) fn new(data: CelestialBodyData, central_body: Option<&Self>, time: Time) -> Self {
        let position = match central_body {
            Some(central_body) => data.get_orbital_parameters().calculate_position(
                data.get_mass(),
                &central_body,
                time,
            ),
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
        &self.data.get_name()
    }
}
