use astro_utils::coordinates::direction::Direction;

pub(super) struct Viewport {
    pub(super) center_direction: Direction,
    pub(super) right_direction: Direction,
    pub(super) width: f32,
}

impl Viewport {}
