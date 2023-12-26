use super::polar_coordinates::PolarCoordinates;

struct Star {
    name: Option<String>,
    polar_coordinates: PolarCoordinates,
    distance: f32,
    brightness: f32,
    temperature: f32,
}
