use astro_utils::length::Length;

struct Star {
    name: Option<String>,
    polar_coordinates: PolarCoordinates,
    distance: Length,
    brightness: f32,
    temperature: f32,
}
