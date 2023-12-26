use astro_utils::{distance::Distance, time::Time, Float};

use super::{celestial_body::CelestialBody, coordinates::CartesianCoordinates};

#[derive(Debug, Clone)]
pub(crate) struct OrbitalParameters {
    semi_major_axis: Distance, // The greatest distance to the central body
    eccentricity: Float,       // The shape of the orbit, deviation from a perfect circle
    inclination: Float,        // The angle between the orbital plane and the reference plane
    longitude_of_ascending_node: Float, // The angle between the reference plane and the ascending node
    argument_of_periapsis: Float,       // The angle between the ascending node and the periapsis
}

impl OrbitalParameters {
    pub(super) fn new(
        semi_major_axis: Distance,
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
            semi_major_axis: Distance::from_astronomical_units(0.0),
            eccentricity: 0.0,
            inclination: 0.0,
            longitude_of_ascending_node: 0.0,
            argument_of_periapsis: 0.0,
        }
    }

    fn orbital_period(central_body: &CelestialBody, semi_major_axis: Distance) -> Time {
        todo!("Calculate the orbital period");
    }

    fn eccentric_anomaly(&self, central_body: &CelestialBody, time: Time) -> Float {
        todo!("Calculate the eccentric anomaly");
    }

    fn mean_anomaly(&self, central_body: &CelestialBody, time: Time) -> Float {
        todo!("Calculate the mean anomaly");
    }

    fn true_anomaly(&self, central_body: &CelestialBody, time: Time) -> Float {
        todo!("Calculate the true anomaly");
    }
}
pub(super) fn calculate_position(
    orbital_parameters: &OrbitalParameters,
    central_body: &CelestialBody,
    time: Time,
) -> CartesianCoordinates {
    todo!("Calculate the current position");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        celestial_body::CelestialBodyData, rotation_parameters::RotationParameters,
    };
    use astro_utils::mass::Mass;

    static TEST_ACCURACY: Float = 1e-5;

    fn sun() -> CelestialBody {
        let sun_data: CelestialBodyData = CelestialBodyData::new(
            String::from("Sun"),
            Mass::from_solar_masses(1.0),
            OrbitalParameters::central(),
            RotationParameters::new(0.0, Time::from_days(0.0), 0.0),
            Distance::from_sun_radii(1.0),
            1.0,
        );

        CelestialBody::new(sun_data, None, Time::from_days(0.0))
    }

    #[test]
    fn circular_orbit() {
        let semi_major_axis = Distance::from_astronomical_units(1.0);
        let eccentricity = 0.0;
        let orbital_parameters =
            OrbitalParameters::new(semi_major_axis, eccentricity, 0.0, 0.0, 0.0);
        let expected_position = CartesianCoordinates {
            x: Distance::from_astronomical_units(1.0),
            y: Distance::from_astronomical_units(0.0),
            z: Distance::from_astronomical_units(0.0),
        };

        let position = calculate_position(&orbital_parameters, &sun(), Time::from_days(0.0));
        assert!(position.eq_within(&expected_position, TEST_ACCURACY));
    }
}
