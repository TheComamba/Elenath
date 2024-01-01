use super::{shared_canvas_functionality::draw_body_name, top_view_widget::TopViewState};
use crate::{
    gui::shared_canvas_functionality::draw_background, model::celestial_body::CelestialBody,
};
use astro_utils::{
    coordinates::{direction::Direction, rotations::get_rotation_parameters},
    units::{angle::Angle, length::Length},
};
use iced::{
    alignment::Horizontal,
    widget::canvas::{self, Path, Style},
    Color, Point,
};

impl TopViewState {
    fn canvas_position(
        &self,
        body: &CelestialBody,
        view_angle: Angle,
        view_rotation_axis: &Direction,
    ) -> iced::Vector {
        let three_dim_position = body.get_position();
        let rotated_position = three_dim_position.rotated(-view_angle, view_rotation_axis); //passive transformation
        let x = rotated_position.x().as_meters() / self.meter_per_pixel;
        let y = -rotated_position.y().as_meters() / self.meter_per_pixel; // y axis is inverted
        iced::Vector::new(x as f32, y as f32)
    }

    pub(super) fn canvas(
        &self,
        renderer: &iced::Renderer,
        bounds: iced::Rectangle,
        selected_body: &Option<CelestialBody>,
        celestial_bodies: &Vec<CelestialBody>,
    ) -> Vec<canvas::Geometry> {
        let background = self
            .background_cache
            .draw(renderer, bounds.size(), |frame| {
                draw_background(bounds, frame);
            });

        let bodies = self.bodies_cache.draw(renderer, bounds.size(), |frame| {
            self.draw_bodies(selected_body, celestial_bodies, frame);
        });

        let scale = self.scale_cache.draw(renderer, bounds.size(), |frame| {
            self.draw_scale(bounds, frame);
        });

        vec![background, bodies, scale]
    }

    fn draw_bodies(
        &self,
        selected_body: &Option<CelestialBody>,
        celestial_bodies: &Vec<CelestialBody>,
        frame: &mut canvas::Frame,
    ) {
        let view_direction = Direction::from_spherical(&self.view_ecliptic.get_spherical());
        let (view_angle, view_rotation_axis) =
            get_rotation_parameters(&Direction::Z, &view_direction);

        let offset = match selected_body {
            Some(focus) => self.canvas_position(focus, view_angle, &view_rotation_axis),
            None => iced::Vector::new(0.0 as f32, 0.0 as f32),
        };
        let bodies = Path::new(|path_builder| {
            for body in celestial_bodies.iter() {
                self.draw_body(
                    frame,
                    body,
                    view_angle,
                    view_rotation_axis,
                    offset,
                    path_builder,
                );
            }
        });
        frame.fill(&bodies, Color::WHITE);
    }

    fn draw_body(
        &self,
        frame: &mut canvas::Frame,
        body: &CelestialBody,
        view_angle: Angle,
        view_rotation_axis: Direction,
        offset: iced::Vector,
        path_builder: &mut canvas::path::Builder,
    ) {
        let radius = 3.0;
        let pos =
            frame.center() + self.canvas_position(body, view_angle, &view_rotation_axis) - offset;
        path_builder.circle(pos, radius);

        draw_body_name(body, pos, frame);
    }

    fn draw_scale(&self, bounds: iced::Rectangle, frame: &mut canvas::Frame) {
        const LENGTH: f32 = 200.0;
        let start_pos = Point::ORIGIN + iced::Vector::new(50. as f32, bounds.height - 50. as f32);
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
        text.content = format!("{}", Length::from_meters(LENGTH * self.meter_per_pixel));
        text.position = middle_pos;
        text.horizontal_alignment = Horizontal::Center;
        frame.fill_text(text);
    }
}