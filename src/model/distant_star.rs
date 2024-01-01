use astro_utils::{
    color::Color,
    coordinates::{cartesian::CartesianCoordinates, direction::Direction},
    stellar_properties::StellarProperties,
    units::length::Length,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct DistantStar {
    stellar_properties: StellarProperties,
    color: Color,
    direction: Direction,
    distance: Length,
}

impl DistantStar {
    pub(crate) fn new(
        stellar_properties: StellarProperties,
        direction: Direction,
        distance: Length,
    ) -> Self {
        let color = Color::from_temperature(stellar_properties.get_temperature());
        DistantStar {
            stellar_properties,
            color,
            direction,
            distance,
        }
    }

    pub(crate) fn get_stellar_properties(&self) -> &StellarProperties {
        &self.stellar_properties
    }

    pub(crate) fn calculate_position(&self) -> CartesianCoordinates {
        self.direction.to_cartesian(self.distance)
    }

    pub(crate) fn get_color(&self) -> &Color {
        &self.color
    }
}
