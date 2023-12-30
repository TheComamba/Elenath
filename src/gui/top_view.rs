use std::f32::consts::PI;

use super::{Gui, GuiMessage};
use crate::model::celestial_body::CelestialBody;
use astro_utils::{
    coordinates::ecliptic::{EclipticCoordinates, Z_DIRECTION},
    units::{angle::Angle, length::Length},
    Float,
};
use iced::{
    alignment::Horizontal,
    widget::{
        canvas::{self, Cache, Path, Style},
        Column,
    },
    Alignment, Color,
};

pub(super) struct TopViewState {
    pub(super) background_cache: Cache,
    pub(super) bodies_cache: Cache,
    pub(super) scale_cache: Cache,
    pub(super) meter_per_pixel: Float,
    pub(super) view_angle: EclipticCoordinates,
}

impl TopViewState {
    pub(super) fn new() -> Self {
        let m_per_au = Length::from_astronomical_units(1.).as_meters();
        TopViewState {
            background_cache: Cache::default(),
            bodies_cache: Cache::default(),
            scale_cache: Cache::default(),
            meter_per_pixel: 0.01 * m_per_au,
            view_angle: Z_DIRECTION,
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
    pub(super) fn topview_control_field(&self) -> iced::Element<'_, GuiMessage> {
        let m_per_px = self.topview_state.get_meter_per_pixel();
        let length_scale_control_field = self.control_field(
            "Length per 100px:",
            format!("{}", Length::from_meters(100. * m_per_px)),
            GuiMessage::UpdateLengthScale(m_per_px / 2.),
            GuiMessage::UpdateLengthScale(m_per_px * 2.),
        );
        const VIEW_ANGLE_STEP: Angle = Angle::from_radians(10. * 2. * PI / 360.);
        let view_longitude = self.topview_state.view_angle.get_longitude();
        let view_longitude_control_field = self.control_field(
            "View longitude:",
            format!("{}", view_longitude),
            GuiMessage::UpdateViewLongitude(view_longitude - VIEW_ANGLE_STEP),
            GuiMessage::UpdateViewLongitude(view_longitude + VIEW_ANGLE_STEP),
        );
        let view_latitude = self.topview_state.view_angle.get_latitude();
        let view_latitude_control_field = self.control_field(
            "View latitude:",
            format!("{}", view_latitude),
            GuiMessage::UpdateViewLatitude(view_latitude - VIEW_ANGLE_STEP),
            GuiMessage::UpdateViewLatitude(view_latitude + VIEW_ANGLE_STEP),
        );
        let planet_picker = self.planet_picker();
        Column::new()
            .push(self.time_control_fields())
            .push(length_scale_control_field)
            .push(view_longitude_control_field)
            .push(view_latitude_control_field)
            .push(planet_picker)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }

    fn canvas_position(&self, body: &CelestialBody) -> iced::Vector {
        let x = body.get_position().x().as_meters() / self.topview_state.meter_per_pixel;
        let y = -body.get_position().y().as_meters() / self.topview_state.meter_per_pixel; // y axis is inverted
        iced::Vector::new(x as f32, y as f32)
    }

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
                let offset = match &self.selected_focus {
                    Some(focus) => self.canvas_position(focus),
                    None => iced::Vector::new(0.0 as f32, 0.0 as f32),
                };
                let bodies = Path::new(|path_builder| {
                    for body in self.celestial_bodies.iter() {
                        let radius = 3.0;
                        let pos = frame.center() + self.canvas_position(body) - offset;
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
