use astro_utils::{units::length::Length, Float};
use iced::{
    alignment::Horizontal,
    widget::canvas::{self, Cache, Path, Style},
    Color,
};

use super::Gui;

pub(super) struct TopViewState {
    pub(super) background_cache: Cache,
    pub(super) bodies_cache: Cache,
    pub(super) scale_cache: Cache,
    pub(super) meter_per_pixel: Float,
}

impl TopViewState {
    pub(super) fn new() -> Self {
        let m_per_au = Length::from_astronomical_units(1.).as_meters();
        TopViewState {
            background_cache: Cache::default(),
            bodies_cache: Cache::default(),
            scale_cache: Cache::default(),
            meter_per_pixel: 0.01 * m_per_au,
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

    pub(super) fn redraw(&mut self) {
        self.bodies_cache.clear();
        self.scale_cache.clear();
    }
}

impl Gui {
    pub(super) fn topview_canvas(
        &self,
        renderer: &iced::Renderer,
        bounds: iced::Rectangle,
    ) -> Vec<canvas::Geometry> {
        let background =
            self.topview_state
                .background_cache
                .draw(renderer, bounds.size(), |frame| {
                    let background = Path::rectangle(bounds.position(), bounds.size());
                    frame.fill(&background, Color::BLACK);
                });
        let bodies = self
            .topview_state
            .bodies_cache
            .draw(renderer, bounds.size(), |frame| {
                let bodies = Path::new(|path_builder| {
                    for body in self.celestial_bodies.iter() {
                        let x = body.get_position().x().as_meters()
                            / self.topview_state.meter_per_pixel;
                        let y = -body.get_position().y().as_meters()
                            / self.topview_state.meter_per_pixel; // y axis is inverted
                        let radius = 3.0;
                        let pos = frame.center() + iced::Vector::new(x as f32, y as f32);
                        path_builder.circle(pos, radius);

                        let mut name_widget = canvas::Text::default();
                        name_widget.color = Color::WHITE;
                        name_widget.content = body.get_name().to_string();
                        name_widget.position = pos;
                        frame.fill_text(name_widget);
                    }
                });
                frame.fill(&bodies, Color::WHITE);
            });
        let scale = self
            .topview_state
            .scale_cache
            .draw(renderer, bounds.size(), |frame| {
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

                let mut text = canvas::Text::default();
                text.color = Color::WHITE;
                text.content = format!(
                    "{}",
                    Length::from_meters(LENGTH * self.topview_state.meter_per_pixel)
                );
                text.position = middle_pos;
                text.horizontal_alignment = Horizontal::Center;
                frame.fill_text(text);
            });
        vec![background, bodies, scale]
    }
}