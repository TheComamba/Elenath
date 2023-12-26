use astro_utils::{distance::Distance, time::Time, Float};

use super::coordinates::CartesianCoordinates;

#[derive(Debug, Clone)]
pub(crate) struct OrbitalParameters {
    semi_major_axis: Distance,
    eccentricity: Float,
    inclination: Float,
    longitude_of_ascending_node: Float,
    argument_of_periapsis: Float,
    true_anomaly: Float,
}

impl OrbitalParameters {
    pub(super) fn new(
        semi_major_axis: Distance,
        eccentricity: Float,
        inclination: Float,
        longitude_of_ascending_node: Float,
        argument_of_periapsis: Float,
        true_anomaly: Float,
    ) -> Self {
        OrbitalParameters {
            semi_major_axis,
            eccentricity,
            inclination,
            longitude_of_ascending_node,
            argument_of_periapsis,
            true_anomaly,
        }
    }

    pub(super) fn current_position(&self, time: Time) -> CartesianCoordinates {
        //TODO
        CartesianCoordinates {
            x: Distance::from_astronomical_units(0.0),
            y: Distance::from_astronomical_units(0.0),
            z: Distance::from_astronomical_units(0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_ACCURACY: Float = 1e-5;

    #[test]
    fn test_orbital_parameters() {
        let orbital_parameters = OrbitalParameters::new(
            Distance::from_astronomical_units(1.0),
            0.0,
            0.0,
            0.0,
            0.0,
            std::f32::consts::PI / 2.0,
        );
        let expected_position = CartesianCoordinates {
            x: Distance::from_astronomical_units(1.0),
            y: Distance::from_astronomical_units(0.0),
            z: Distance::from_astronomical_units(0.0),
        };

        let position = orbital_parameters.current_position(Time::from_days(0.0));
        assert!(position.eq_within(&expected_position, TEST_ACCURACY));
    }
}
