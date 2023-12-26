// The "absolute" reference we use is heliocentric ecliptic coordinates:
// Longitude denotes the angle between the vernal equinox and the body, measured in the ecliptic plane.
// Latitude denotes the angle between the ecliptic plane and the body.
// https://en.wikipedia.org/wiki/Ecliptic_coordinate_system

pub(super) struct PolarCoordinates {
    longitude: f64,
    latitude: f64,
}
