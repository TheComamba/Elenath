use crate::model::celestial_body::CelestialBody;
use iced::{
    widget::canvas::{self, Cache, Path},
    Color,
};

pub(super) struct TopViewState {
    topview_bodies_cache: Cache,
    celestial_bodies: Vec<CelestialBody>,
}

impl TopViewState {
    pub(super) fn new(celestial_bodies: Vec<CelestialBody>) -> Self {
        TopViewState {
            topview_bodies_cache: Cache::default(),
            celestial_bodies,
        }
    }
}

impl<GuiMessage> canvas::Program<GuiMessage> for TopViewState {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::theme::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        const LENGTH_SCALE: f32 = 100.0;

        let bodies = self
            .topview_bodies_cache
            .draw(renderer, bounds.size(), |frame| {
                let bodies = Path::new(|path_builder| {
                    for body in self.celestial_bodies.iter() {
                        let x = body.get_position().x().as_astronomical_units() * LENGTH_SCALE;
                        let y = body.get_position().y().as_astronomical_units() * LENGTH_SCALE;
                        let radius = 3.0;
                        let pos = frame.center() + iced::Vector::new(x as f32, y as f32);
                        path_builder.circle(pos, radius);
                    }
                });
                frame.fill(&bodies, Color::BLACK);
            });
        vec![bodies]
    }
}
