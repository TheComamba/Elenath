use astro_utils::{
    coordinates::direction::Direction, stellar_properties::StellarProperties, units::length::Length,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct DistantStar {
    stellar_properties: StellarProperties,
    direction: Direction,
    distance: Length,
}

impl DistantStar {
    pub(crate) fn new(
        stellar_properties: StellarProperties,
        direction: Direction,
        distance: Length,
    ) -> Self {
        DistantStar {
            stellar_properties,
            direction,
            distance,
        }
    }

    pub(crate) fn get_stellar_properties(&self) -> &StellarProperties {
        &self.stellar_properties
    }

    pub(crate) fn get_direction(&self) -> &Direction {
        &self.direction
    }

    pub(crate) fn get_distance(&self) -> Length {
        self.distance
    }
}
