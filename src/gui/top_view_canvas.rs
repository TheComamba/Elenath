use super::{
    shared_canvas_functionality::{contains_workaround, draw_body_name, maximized_color},
    top_view_widget::TopViewState,
};
use crate::gui::shared_canvas_functionality::draw_background;
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
        let x = rotated_position.x() / self.length_per_pixel;
        let y = -rotated_position.y() / self.length_per_pixel; // y axis is inverted
        iced::Vector::new(x as f32, y as f32)
    }

    pub(super) fn canvas(
        &self,
        renderer: &iced::Renderer,
        bounds: iced::Rectangle,
        selected_body: &Option<CelestialBody>,
        celestial_bodies: &Vec<CelestialBody>,
        display_names: bool,
    ) -> Vec<canvas::Geometry> {
        let background = self
            .background_cache
            .draw(renderer, bounds.size(), |frame| {
                draw_background(bounds, frame);
            });

        let bodies = self.bodies_cache.draw(renderer, bounds.size(), |frame| {
            self.draw_bodies(
                selected_body,
                celestial_bodies,
                &bounds,
                frame,
                display_names,
            );
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
        bounds: &iced::Rectangle,
        frame: &mut canvas::Frame,
        display_names: bool,
    ) {
        let view_direction = &self.view_ecliptic.get_spherical().to_direction();
        let (view_angle, view_rotation_axis) =
            get_rotation_parameters(&Direction::Z, &view_direction);

        let offset = match selected_body {
            Some(focus) => self.canvas_position(focus, view_angle, &view_rotation_axis),
            None => iced::Vector::new(0.0 as f32, 0.0 as f32),
        };
        for body in celestial_bodies.iter() {
            if !body.is_distant_star() {
                self.draw_body(
                    frame,
                    &bounds,
                    body,
                    view_angle,
                    &view_rotation_axis,
                    offset,
                    display_names,
                );
            }
        }
    }

    fn draw_body(
        &self,
        frame: &mut canvas::Frame,
        bounds: &iced::Rectangle,
        body: &CelestialBody,
        view_angle: Angle,
        view_rotation_axis: &Direction,
        offset: iced::Vector,
        display_names: bool,
    ) {
        let radius = body_radius(body);
        let pos =
            frame.center() + self.canvas_position(body, view_angle, &view_rotation_axis) - offset;
        if contains_workaround(bounds, pos) {
            let circle = Path::circle(pos, radius);
            let color = body_color(body);
            frame.fill(&circle, color);

            if display_names {
                draw_body_name(body, color, pos, radius, frame);
            }
        }
    }

    fn draw_scale(&self, bounds: iced::Rectangle, frame: &mut canvas::Frame) {
        const LENGTH_IN_PX: f32 = 200.0;
        let start_pos = Point::ORIGIN + iced::Vector::new(50. as f32, bounds.height - 50. as f32);
        let middle_pos = start_pos + iced::Vector::new(LENGTH_IN_PX as f32 / 2., 0.0 as f32);
        let end_pos = start_pos + iced::Vector::new(LENGTH_IN_PX as f32, 0.0 as f32);
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
        text.content = format!("{}", LENGTH_IN_PX * self.length_per_pixel);
        text.position = middle_pos;
        text.horizontal_alignment = Horizontal::Center;
        frame.fill_text(text);
    }
}

fn body_radius(body: &CelestialBody) -> f32 {
    const SIZE_NUMBER: f32 = 0.3;
    let radius = body.get_radius().unwrap_or(Length::ZERO);
    radius.as_kilometers().powf(SIZE_NUMBER) * SIZE_NUMBER
}

fn body_color(body: &CelestialBody) -> Color {
    let mut color = maximized_color(body);
    let albedo = match body.get_data() {
        CelestialBodyData::Planet(data) => data.get_geometric_albedo(),
        _ => 1.,
    };
    color.a = albedo as f32;
    color
}
