use astro_utils::{length::Length, Float};

/*  The "absolute" reference we use for polar coordiantes is heliocentric ecliptic coordinates:
 * Longitude denotes the angle between the vernal equinox and the body, measured in the ecliptic plane.
 * Latitude denotes the angle between the ecliptic plane and the body.
 * https://en.wikipedia.org/wiki/Ecliptic_coordinate_system
 */
pub(super) struct PolarCoordinates {
    pub(super) longitude: Float,
    pub(super) latitude: Float,
}

pub(super) struct CartesianCoordinates {
    pub(super) x: Length,
    pub(super) y: Length,
    pub(super) z: Length,
}

impl CartesianCoordinates {
    pub(super) fn zero() -> Self {
        CartesianCoordinates {
            x: Length::from_astronomical_units(0.0),
            y: Length::from_astronomical_units(0.0),
            z: Length::from_astronomical_units(0.0),
        }
    }

    pub(super) fn eq_within(&self, other: &CartesianCoordinates, accuracy: Float) -> bool {
        self.x.eq_within(other.x, accuracy)
            && self.y.eq_within(other.y, accuracy)
            && self.z.eq_within(other.z, accuracy)
    }
}
