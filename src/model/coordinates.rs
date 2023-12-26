/*  The "absolute" reference we use for polar coordiantes is heliocentric ecliptic coordinates:
 * Longitude denotes the angle between the vernal equinox and the body, measured in the ecliptic plane.
 * Latitude denotes the angle between the ecliptic plane and the body.
 * https://en.wikipedia.org/wiki/Ecliptic_coordinate_system
 */
pub(super) struct PolarCoordinates {
    pub(super) longitude: f64,
    pub(super) latitude: f64,
}

pub(super) struct CartesianCoordinates {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) z: f64,
}
