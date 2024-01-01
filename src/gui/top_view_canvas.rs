use super::Gui;
use crate::model::celestial_body::CelestialBody;
use astro_utils::{
    coordinates::{direction::Direction, rotations::get_rotation_parameters},
    units::{angle::Angle, length::Length},
};
use iced::{
    alignment::Horizontal,
    widget::canvas::{self, Path, Style},
    Color, Point,
};

impl Gui {
    fn canvas_position(
        &self,
        body: &CelestialBody,
        view_angle: Angle,
        view_rotation_axis: &Direction,
    ) -> iced::Vector {
        let three_dim_position = body.get_position();
        let rotated_position = three_dim_position.rotated(-view_angle, view_rotation_axis); //passive transformation
        let x = rotated_position.x().as_meters() / self.topview_state.meter_per_pixel;
        let y = -rotated_position.y().as_meters() / self.topview_state.meter_per_pixel; // y axis is inverted
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
                    let background = Path::rectangle(Point::ORIGIN, bounds.size());
                    frame.fill(&background, Color::BLACK);
                });
        let view_direction =
            Direction::from_spherical(&self.topview_state.view_ecliptic.get_spherical());
        let (view_angle, view_rotation_axis) =
            get_rotation_parameters(&Direction::Z, &view_direction);
        let bodies = self
            .topview_state
            .bodies_cache
            .draw(renderer, bounds.size(), |frame| {
                let offset = match &self.selected_body {
                    Some(focus) => self.canvas_position(focus, view_angle, &view_rotation_axis),
                    None => iced::Vector::new(0.0 as f32, 0.0 as f32),
                };
                let bodies = Path::new(|path_builder| {
                    for body in self.celestial_bodies.iter() {
                        let radius = 3.0;
                        let pos = frame.center()
                            + self.canvas_position(body, view_angle, &view_rotation_axis)
                            - offset;
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
                let start_pos =
                    Point::ORIGIN + iced::Vector::new(50. as f32, bounds.height - 50. as f32);
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
