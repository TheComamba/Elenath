use crate::model::celestial_body::CelestialBody;
use iced::{
    widget::canvas::{self, Cache, Path},
    Color, Size,
};

pub(super) struct TopViewState {
    topview_background_cache: Cache,
    topview_bodies_cache: Cache,
    celestial_bodies: Vec<CelestialBody>,
}

impl TopViewState {
    pub(super) fn new(celestial_bodies: Vec<CelestialBody>) -> Self {
        TopViewState {
            topview_background_cache: Cache::default(),
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
        let background = self
            .topview_background_cache
            .draw(renderer, bounds.size(), |frame| {
                frame.fill_rectangle(frame.center(), Size::UNIT, Color::BLACK)
            });
        let bodies = self
            .topview_bodies_cache
            .draw(renderer, bounds.size(), |frame| {
                let bodies = Path::new(|path_builder| {
                    for body in self.celestial_bodies.iter() {
                        let radius = 3.0;
                        let pos = frame.center();
                        path_builder.circle(pos, radius);
                    }
                });
                frame.fill(&bodies, Color::BLACK);
            });
        vec![bodies]
    }
}
