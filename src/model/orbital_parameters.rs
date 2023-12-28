use astro_utils::{
    coordinates::cartesian::{CartesianCoordinates, ORIGIN},
    orbit_orientation::OrbitOrientation,
    units::{angle::Angle, length::Length, time::Time},
    Float,
};

use super::celestial_body::CelestialBody;

#[derive(Debug, Clone)]
pub(crate) struct OrbitalParameters {
    semi_major_axis: Length,
    eccentricity: Float,
    orientation: OrbitOrientation,
}

impl OrbitalParameters {
    pub(super) fn new(
        semi_major_axis: Length,
        eccentricity: Float,
        orientation: OrbitOrientation,
    ) -> Self {
        OrbitalParameters {
            semi_major_axis,
            eccentricity,
            orientation,
        }
    }

    pub(super) fn central() -> Self {
        OrbitalParameters {
            semi_major_axis: Length::from_astronomical_units(0.0),
            eccentricity: 0.0,
            orientation: OrbitOrientation::new(
                Angle::from_radians(0.0),
                Angle::from_radians(0.0),
                Angle::from_radians(0.0),
            ),
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
        ORIGIN
    }
}
