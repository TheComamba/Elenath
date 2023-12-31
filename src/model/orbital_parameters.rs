use super::celestial_body::CelestialBody;
use astro_utils::{
    coordinates::cartesian::CartesianCoordinates,
    kepler_orbit::{
        eccentric_anomaly, mean_anomaly, orbital_period, position_relative_to_central_body,
        true_anomaly,
    },
    orbit_orientation::OrbitOrientation,
    units::{angle::Angle, length::Length, mass::Mass, time::Time},
    Float,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub(crate) fn get_semi_major_axis(&self) -> Length {
        self.semi_major_axis
    }

    pub(crate) fn get_eccentricity(&self) -> Float {
        self.eccentricity
    }

    pub(crate) fn get_inclination(&self) -> Angle {
        self.orientation.inclination()
    }

    pub(crate) fn get_longitude_of_ascending_node(&self) -> Angle {
        self.orientation.longitude_of_ascending_node()
    }

    pub(crate) fn get_argument_of_periapsis(&self) -> Angle {
        self.orientation.argument_of_periapsis()
    }

    pub(super) fn calculate_position(
        &self,
        body_mass: Mass,
        central_body: &CelestialBody,
        time: Time,
    ) -> CartesianCoordinates {
        let central_body_position = central_body.get_position();

        let orbital_period =
            orbital_period(self.semi_major_axis, body_mass, central_body.get_mass());
        let mean_anomaly = mean_anomaly(orbital_period, time);
        let eccentric_anomaly = eccentric_anomaly(mean_anomaly, self.eccentricity);
        let true_anomaly = true_anomaly(eccentric_anomaly, self.eccentricity);
        let position = position_relative_to_central_body(
            self.semi_major_axis,
            self.eccentricity,
            true_anomaly,
            &self.orientation,
        );
        central_body_position + &position
    }
}
