use astro_utils::coordinates::direction::Direction;

pub(super) struct Viewport {
    pub(super) center_direction: Direction,
    pub(super) top_direction: Direction,
    pub(super) height: f32,
}

impl Viewport {}
