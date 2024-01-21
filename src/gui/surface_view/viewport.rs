use astro_utils::coordinates::direction::Direction;

pub(super) struct Viewport {
    pub(super) center_direction: Direction,
    pub(super) right_direction: Direction,
}

impl Viewport {
    pub(super) fn new(center_direction: Direction, right_direction: Direction) -> Self {
        Self {
            center_direction,
            right_direction,
        }
    }
}
