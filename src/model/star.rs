use super::coordinates::PolarCoordinates;
use astro_utils::distance::Distance;

struct Star {
    name: Option<String>,
    polar_coordinates: PolarCoordinates,
    distance: Distance,
    brightness: f32,
    temperature: f32,
}
