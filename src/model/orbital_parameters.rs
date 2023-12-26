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
}

fn orbital_period(orbital_parameters: &OrbitalParameters, central_body: &CelestialBody) -> Time {
    todo!("Calculate the orbital period");
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

    fn earth() -> CelestialBody {
        let earth_data: CelestialBodyData = CelestialBodyData::new(
            String::from("Earth"),
            Mass::from_earth_masses(1.0),
            OrbitalParameters::new(Distance::from_astronomical_units(1.0), 0.0, 0.0, 0.0, 0.0),
            RotationParameters::new(0.0, Time::from_days(1.0), 0.0),
            Distance::from_earth_radii(1.0),
            1.0,
        );

        CelestialBody::new(earth_data, Some(sun()), Time::from_days(0.0))
    }

    fn jupiter() -> CelestialBody {
        let jupiter_data: CelestialBodyData = CelestialBodyData::new(
            String::from("Jupiter"),
            Mass::from_jupiter_masses(1.0),
            OrbitalParameters::new(
                Distance::from_astronomical_units(5.2),
                0.048,
                0.022,
                100.464,
                14.753,
            ),
            RotationParameters::new(0.0, Time::from_days(0.41354), 0.0),
            Distance::from_earth_radii(10.97),
            1.0,
        );

        CelestialBody::new(jupiter_data, Some(sun()), Time::from_days(0.0))
    }

    fn moon() -> CelestialBody {
        let moon_data: CelestialBodyData = CelestialBodyData::new(
            String::from("Moon"),
            Mass::from_earth_masses(0.0123),
            OrbitalParameters::new(Distance::from_kilometers(384399.0), 0.0, 0.0, 0.0, 0.0),
            RotationParameters::new(0.0, Time::from_days(27.321), 0.0),
            Distance::from_earth_radii(0.273),
            1.0,
        );

        CelestialBody::new(moon_data, Some(earth()), Time::from_days(0.0))
    }

    #[test]
    fn orbital_period_of_earth() {
        let expected_orbital_period = Time::from_days(365.256);
        let orbital_period = orbital_period(earth().get_orbital_parameters(), &sun());
        assert!(orbital_period.eq_within(expected_orbital_period, TEST_ACCURACY));
    }

    #[test]
    fn orbital_period_of_jupiter() {
        let expected_orbital_period = Time::from_days(4332.59);
        let orbital_period = orbital_period(jupiter().get_orbital_parameters(), &sun());
        assert!(orbital_period.eq_within(expected_orbital_period, TEST_ACCURACY));
    }

    #[test]
    fn orbital_period_of_moon() {
        let expected_orbital_period = Time::from_days(27.321);
        let orbital_period = orbital_period(moon().get_orbital_parameters(), &sun());
        assert!(orbital_period.eq_within(expected_orbital_period, TEST_ACCURACY));
    }
}
