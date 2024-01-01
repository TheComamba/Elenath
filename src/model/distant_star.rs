use astro_utils::{coordinates::ecliptic::EclipticCoordinates, units::length::Length};

pub(crate) struct DistantStar {
    name: Option<String>,
    polar_coordinates: EclipticCoordinates,
    distance: Length,
    brightness: f32,
    temperature: f32,
}
