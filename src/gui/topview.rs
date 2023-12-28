use crate::model::celestial_body::CelestialBody;
use astro_utils::Float;
use iced::{
    widget::canvas::{self, Cache, Path, Text},
    Color,
};

pub(super) struct TopViewState {
    topview_bodies_cache: Cache,
    celestial_bodies: Vec<CelestialBody>,
    meter_per_pixel: Float,
}

impl TopViewState {
    pub(super) fn new(celestial_bodies: Vec<CelestialBody>) -> Self {
        TopViewState {
            topview_bodies_cache: Cache::default(),
            celestial_bodies,
            meter_per_pixel: 1e10,
        }
    }
}

impl TopViewState {
    pub(super) fn set_meter_per_pixel(&mut self, meter_per_pixel: Float) {
        self.meter_per_pixel = meter_per_pixel;
    }

    pub(super) fn get_meter_per_pixel(&self) -> Float {
        self.meter_per_pixel
    }

    pub(super) fn set_celestial_bodies(&mut self, celestial_bodies: Vec<CelestialBody>) {
        self.celestial_bodies = celestial_bodies;
    }

    pub(super) fn redraw(&mut self) {
        self.topview_bodies_cache.clear();
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
        let bodies = self
            .topview_bodies_cache
            .draw(renderer, bounds.size(), |frame| {
                let bodies = Path::new(|path_builder| {
                    for body in self.celestial_bodies.iter() {
                        let x = body.get_position().x().as_meters() / self.meter_per_pixel;
                        let y = body.get_position().y().as_meters() / self.meter_per_pixel;
                        let radius = 3.0;
                        let pos = frame.center() + iced::Vector::new(x as f32, y as f32);
                        path_builder.circle(pos, radius);

                        let mut name_widget = Text::default();
                        name_widget.color = Color::WHITE;
                        name_widget.content = body.get_name().to_string();
                        name_widget.position = pos;
                        frame.fill_text(name_widget);
                    }
                });
                frame.fill(&bodies, Color::WHITE);
            });
        vec![bodies]
    }
}
