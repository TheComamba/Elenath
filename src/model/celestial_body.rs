use super::planet_data::PlanetData;
use astro_utils::{
    coordinates::{cartesian::CartesianCoordinates, direction::Direction},
    stellar_properties::StellarProperties,
    units::{length::Length, mass::Mass, time::Time},
};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub(crate) struct CelestialBody {
    data: CelestialBodyData,
    position: CartesianCoordinates,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CelestialBodyData {
    Star(StellarProperties),
    Planet(PlanetData),
}

impl PartialEq for CelestialBody {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for CelestialBody {}

impl Display for CelestialBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

impl CelestialBody {
    pub(crate) fn central_body(data: StellarProperties) -> Self {
        CelestialBody {
            data: CelestialBodyData::Star(data),
            position: CartesianCoordinates::ORIGIN,
        }
    }

    pub(crate) fn from_planet(data: PlanetData, central_body: &Self, time: Time) -> Self {
        let position =
            data.get_orbital_parameters()
                .calculate_position(data.get_mass(), &central_body, time);
        CelestialBody {
            data: CelestialBodyData::Planet(data),
            position,
        }
    }

    pub(crate) fn get_mass(&self) -> Mass {
        match &self.data {
            CelestialBodyData::Star(data) => data.get_mass(),
            CelestialBodyData::Planet(data) => data.get_mass(),
        }
    }

    pub(crate) fn get_position(&self) -> &CartesianCoordinates {
        &self.position
    }

    pub(crate) fn get_name(&self) -> &str {
        match &self.data {
            CelestialBodyData::Star(data) => data.get_name(),
            CelestialBodyData::Planet(data) => data.get_name(),
        }
    }

    pub(crate) fn get_rotation_axis(&self) -> &Direction {
        match &self.data {
            CelestialBodyData::Star(_) => &Direction::Z,
            CelestialBodyData::Planet(data) => data.get_rotation_axis(),
        }
    }

    pub(crate) fn get_sideral_rotation_period(&self) -> Time {
        match &self.data {
            CelestialBodyData::Star(_) => Time::from_seconds(0.),
            CelestialBodyData::Planet(data) => data.get_sideral_rotation_period(),
        }
    }

    pub(crate) fn get_radius(&self) -> Length {
        match &self.data {
            CelestialBodyData::Star(data) => data.get_radius(),
            CelestialBodyData::Planet(data) => data.get_radius(),
        }
    }
}
