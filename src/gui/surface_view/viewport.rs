use astro_utils::coordinates::direction::Direction;

pub(super) struct Viewport {
    center_direction: Direction,
    width: f32,
}