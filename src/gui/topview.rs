use crate::model::celestial_body::CelestialBody;
use astro_utils::{units::length::Length, Float};
use iced::{
    alignment::Horizontal,
    widget::canvas::{self, Cache, Path, Style, Text},
    Color,
};

pub(super) struct TopViewState {
    background_cache: Cache,
    bodies_cache: Cache,
    scale_cache: Cache,
    celestial_bodies: Vec<CelestialBody>,
    meter_per_pixel: Float,
}

impl TopViewState {
    pub(super) fn new(celestial_bodies: Vec<CelestialBody>) -> Self {
        TopViewState {
            background_cache: Cache::default(),
            bodies_cache: Cache::default(),
            scale_cache: Cache::default(),
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
        self.bodies_cache.clear();
        self.scale_cache.clear();
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
            .background_cache
            .draw(renderer, bounds.size(), |frame| {
                let background = Path::rectangle(bounds.position(), bounds.size());
                frame.fill(&background, Color::BLACK);
            });
        let bodies = self.bodies_cache.draw(renderer, bounds.size(), |frame| {
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
        let scale = self.scale_cache.draw(renderer, bounds.size(), |frame| {
            const LENGTH: f32 = 200.0;
            let start_pos = bounds.position() + iced::Vector::new(50. as f32, 50. as f32);
            let middle_pos = start_pos + iced::Vector::new(LENGTH as f32 / 2., 0.0 as f32);
            let end_pos = start_pos + iced::Vector::new(LENGTH as f32, 0.0 as f32);
            let delimitor_vec = iced::Vector::new(0.0 as f32, 5. as f32);

            let scale = Path::new(|path_builder| {
                path_builder.move_to(start_pos + delimitor_vec);
                path_builder.line_to(start_pos - delimitor_vec);
                path_builder.move_to(start_pos);
                path_builder.line_to(end_pos);
                path_builder.move_to(end_pos + delimitor_vec);
                path_builder.line_to(end_pos - delimitor_vec);
            });
            let mut stroke = canvas::Stroke::default();
            stroke.style = Style::Solid(Color::WHITE);

            frame.stroke(&scale, stroke);

            let mut text = Text::default();
            text.color = Color::WHITE;
            text.content = format!("{}", Length::from_meters(LENGTH * self.meter_per_pixel));
            text.position = middle_pos;
            text.horizontal_alignment = Horizontal::Center;
            frame.fill_text(text);
        });
        vec![background, bodies, scale]
    }
}
