use astro_utils::{length::Length, time::Time, Float};

use super::{celestial_body::CelestialBody, coordinates::CartesianCoordinates};

#[derive(Debug, Clone)]
pub(crate) struct OrbitalParameters {
    semi_major_axis: Length, // The greatest distance to the central body
    eccentricity: Float,     // The shape of the orbit, deviation from a perfect circle
    inclination: Float,      // The angle between the orbital plane and the reference plane
    longitude_of_ascending_node: Float, // The angle between the reference plane and the ascending node
    argument_of_periapsis: Float,       // The angle between the ascending node and the periapsis
}

impl OrbitalParameters {
    pub(super) fn new(
        semi_major_axis: Length,
        eccentricity: Float,
        inclination: Float,
        longitude_of_ascending_node: Float,
        argument_of_periapsis: Float,
    ) -> Self {
        OrbitalParameters {
            semi_major_axis,
            eccentricity,
            inclination,
            longitude_of_ascending_node,
            argument_of_periapsis,
        }
    }

    pub(super) fn central() -> Self {
        OrbitalParameters {
            semi_major_axis: Length::from_astronomical_units(0.0),
            eccentricity: 0.0,
            inclination: 0.0,
            longitude_of_ascending_node: 0.0,
            argument_of_periapsis: 0.0,
        }
    }

    pub(super) fn get_semi_major_axis(&self) -> Length {
        self.semi_major_axis
    }

    pub(super) fn calculate_position(
        &self,
        central_body: &CelestialBody,
        time: Time,
    ) -> CartesianCoordinates {
        //todo!("Calculate the current position");
        CartesianCoordinates::zero()
    }
}
