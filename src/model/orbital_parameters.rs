use super::coordinates::CartesianCoordinates;

#[derive(Debug, Clone)]
pub(crate) struct OrbitalParameters {
    semi_major_axis: f64,
    eccentricity: f64,
    inclination: f64,
    longitude_of_ascending_node: f64,
    argument_of_periapsis: f64,
    true_anomaly: f64,
}

impl OrbitalParameters {
    pub(super) fn new(
        semi_major_axis: f64,
        eccentricity: f64,
        inclination: f64,
        longitude_of_ascending_node: f64,
        argument_of_periapsis: f64,
        true_anomaly: f64,
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

    pub(super) fn current_position(&self, time: f64) -> CartesianCoordinates {
        //TODO
        CartesianCoordinates {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbital_parameters() {
        let orbital_parameters = OrbitalParameters::new(
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            std::f64::consts::PI / 2.0,
        );
        let position = orbital_parameters.current_position(0.0);
        assert_eq!(position.x, 1.0);
        assert_eq!(position.y, 0.0);
        assert_eq!(position.z, 0.0);
    }
}