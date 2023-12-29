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
        central_body_position + position
    }
}
