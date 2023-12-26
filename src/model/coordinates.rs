use astro_utils::{distance::Distance, Float};

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
    pub(super) x: Distance,
    pub(super) y: Distance,
    pub(super) z: Distance,
}

impl CartesianCoordinates {
    pub(super) fn eq_within(&self, other: &CartesianCoordinates, accuracy: Float) -> bool {
        self.x.eq_within(other.x, accuracy)
            && self.y.eq_within(other.y, accuracy)
            && self.z.eq_within(other.z, accuracy)
    }
}
