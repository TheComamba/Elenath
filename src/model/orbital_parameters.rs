use astro_utils::{distance::Distance, mass::Mass, time::Time, Float};

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

    pub(super) fn get_semi_major_axis(&self) -> Distance {
        self.semi_major_axis
    }
}

/*
 * The orbital period is the time it takes for a given object to make one full orbit around another object.
 * https://en.wikipedia.org/wiki/Orbital_period
 */
fn orbital_period(semi_major_axis: Distance, mass1: Mass, mass2: Mass) -> Time {
    const G: Float = 6.67430e-11;
    const PI: Float = std::f32::consts::PI;

    let semi_major_axis_cubed = semi_major_axis.as_meters().powi(3);
    let total_mass = mass1.as_kilograms() + mass2.as_kilograms();
    let orbital_period = 2.0 * PI * (semi_major_axis_cubed / total_mass / G).sqrt();
    Time::from_seconds(orbital_period)
}

fn eccentric_anomaly(
    orbital_parameters: &OrbitalParameters,
    central_body: &CelestialBody,
    time: Time,
) -> Float {
    todo!("Calculate the eccentric anomaly");
}

fn mean_anomaly(
    orbital_parameters: &OrbitalParameters,
    central_body: &CelestialBody,
    time: Time,
) -> Float {
    todo!("Calculate the mean anomaly");
}

fn true_anomaly(
    orbital_parameters: &OrbitalParameters,
    central_body: &CelestialBody,
    time: Time,
) -> Float {
    todo!("Calculate the true anomaly");
}

pub(super) fn calculate_position(
    orbital_parameters: &OrbitalParameters,
    central_body: &CelestialBody,
    time: Time,
) -> CartesianCoordinates {
    //todo!("Calculate the current position");
    CartesianCoordinates::zero()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::example::*;

    static TEST_ACCURACY: Float = 1e-2;

    #[test]
    fn orbital_period_of_earth() {
        let expected_orbital_period = Time::from_days(365.256);
        let orbital_period = orbital_period(
            earth().get_semi_major_axis(),
            earth().get_mass(),
            sun().get_mass(),
        );
        println!("Expected orbital period: {}", expected_orbital_period);
        println!("Calculated orbital period: {}", orbital_period);
        assert!(orbital_period.eq_within(expected_orbital_period, TEST_ACCURACY));
    }

    #[test]
    fn orbital_period_of_jupiter() {
        let expected_orbital_period = Time::from_days(4332.59);
        let orbital_period = orbital_period(
            jupiter().get_semi_major_axis(),
            jupiter().get_mass(),
            sun().get_mass(),
        );
        println!("Expected orbital period: {}", expected_orbital_period);
        println!("Calculated orbital period: {}", orbital_period);
        assert!(orbital_period.eq_within(expected_orbital_period, TEST_ACCURACY));
    }

    #[test]
    fn orbital_period_of_moon() {
        let expected_orbital_period = Time::from_days(27.321);
        let orbital_period = orbital_period(
            moon().get_semi_major_axis(),
            moon().get_mass(),
            earth().get_mass(),
        );
        println!("Expected orbital period: {}", expected_orbital_period);
        println!("Calculated orbital period: {}", orbital_period);
        assert!(orbital_period.eq_within(expected_orbital_period, TEST_ACCURACY));
    }
}
