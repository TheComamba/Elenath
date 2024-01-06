use astro_utils::{
    color::sRGBColor,
    coordinates::{cartesian::CartesianCoordinates, direction::Direction},
    planets::planet::Planet,
    stars::star::Star,
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
    CentralBody(Star),
    Star(Star),
    Planet(Planet),
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
    pub(crate) fn central_body(data: Star) -> Self {
        CelestialBody {
            data: CelestialBodyData::CentralBody(data),
            position: CartesianCoordinates::ORIGIN,
        }
    }

    pub(crate) fn from_distant_star(star: &Star) -> Self {
        CelestialBody {
            data: CelestialBodyData::Star(star.clone()),
            position: star.calculate_position(),
        }
    }

    pub(crate) fn from_planet(data: &Planet, central_body: &Star, time: Time) -> Self {
        let position =
            data.get_orbital_parameters()
                .calculate_position(data.get_mass(), &central_body, time);
        CelestialBody {
            data: CelestialBodyData::Planet(data.clone()),
            position,
        }
    }

    pub(crate) fn is_distant_star(&self) -> bool {
        match &self.data {
            CelestialBodyData::Star(_) => true,
            _ => false,
        }
    }

    pub(crate) fn get_data(&self) -> &CelestialBodyData {
        &self.data
    }

    pub(crate) fn get_color(&self) -> &sRGBColor {
        match &self.data {
            CelestialBodyData::CentralBody(data) => data.get_color(),
            CelestialBodyData::Star(data) => data.get_color(),
            CelestialBodyData::Planet(data) => data.get_color(),
        }
    }

    pub(crate) fn get_mass(&self) -> Mass {
        match &self.data {
            CelestialBodyData::CentralBody(data) => data.get_mass(),
            CelestialBodyData::Star(data) => data.get_mass(),
            CelestialBodyData::Planet(data) => data.get_mass(),
        }
    }

    pub(crate) fn get_position(&self) -> &CartesianCoordinates {
        &self.position
    }

    pub(crate) fn get_name(&self) -> &str {
        match &self.data {
            CelestialBodyData::CentralBody(data) => data.get_name(),
            CelestialBodyData::Star(data) => data.get_name(),
            CelestialBodyData::Planet(data) => data.get_name(),
        }
    }

    pub(crate) fn get_rotation_axis(&self) -> &Direction {
        match &self.data {
            CelestialBodyData::CentralBody(_) => &Direction::Z,
            CelestialBodyData::Star(_) => &Direction::Z,
            CelestialBodyData::Planet(data) => data.get_rotation_axis(),
        }
    }

    pub(crate) fn get_sideral_rotation_period(&self) -> Time {
        match &self.data {
            CelestialBodyData::CentralBody(_) => Time::from_seconds(0.),
            CelestialBodyData::Star(_) => Time::from_seconds(0.),
            CelestialBodyData::Planet(data) => data.get_sideral_rotation_period(),
        }
    }

    pub(crate) fn get_radius(&self) -> Length {
        match &self.data {
            CelestialBodyData::CentralBody(data) => data.get_radius(),
            CelestialBodyData::Star(data) => data.get_radius(),
            CelestialBodyData::Planet(data) => data.get_radius(),
        }
    }
}
