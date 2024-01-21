use super::top_view_widget::TopViewState;
use crate::{
    gui::shared_canvas_functionality::{contains_workaround, draw_background, draw_name},
    model::{celestial_system::CelestialSystem, planet::Planet},
};
use astro_utils::{
    color::sRGBColor,
    coordinates::{
        cartesian::CartesianCoordinates, direction::Direction, rotations::get_rotation_parameters,
    },
    units::{angle::Angle, length::Length, time::Time},
    Float,
};
use iced::{
    alignment::Horizontal,
    widget::canvas::{self, Path, Style},
    Color, Point,
};

impl TopViewState {
    fn canvas_position(
        &self,
        pos: &CartesianCoordinates,
        view_angle: Angle,
        view_rotation_axis: &Direction,
    ) -> iced::Vector {
        let rotated_position = pos.rotated(-view_angle, view_rotation_axis); //passive transformation
        let x = rotated_position.x() / self.length_per_pixel;
        let y = -rotated_position.y() / self.length_per_pixel; // y axis is inverted
        iced::Vector::new(x as f32, y as f32)
    }

    pub(crate) fn canvas(
        &self,
        renderer: &iced::Renderer,
        bounds: iced::Rectangle,
        selected_planet: &Option<Planet>,
        celestial_system: &CelestialSystem,
        time_since_epoch: Time,
        display_names: bool,
    ) -> Vec<canvas::Geometry> {
        let background = self
            .background_cache
            .draw(renderer, bounds.size(), |frame| {
                draw_background(bounds, frame);
            });

        let bodies = self.bodies_cache.draw(renderer, bounds.size(), |frame| {
            self.draw_bodies(
                selected_planet,
                celestial_system,
                time_since_epoch,
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
        selected_planet: &Option<Planet>,
        celestial_system: &CelestialSystem,
        time_since_epoch: Time,
        bounds: &iced::Rectangle,
        frame: &mut canvas::Frame,
        display_names: bool,
    ) {
        let view_direction = &self.view_ecliptic.get_spherical().to_direction();
        let (view_angle, view_rotation_axis) =
            get_rotation_parameters(&Direction::Z, &view_direction);

        let offset = match selected_planet {
            Some(focus) => {
                self.canvas_position(focus.get_position(), view_angle, &view_rotation_axis)
            }
            None => iced::Vector::new(0.0 as f32, 0.0 as f32),
        };

        self.draw_central_body(
            celestial_system,
            frame,
            bounds,
            view_angle,
            &view_rotation_axis,
            offset,
            display_names,
        );

        for planet in celestial_system
            .get_planets_at_time(time_since_epoch)
            .iter()
        {
            let data = planet.get_data();
            self.draw_body(
                frame,
                &bounds,
                data.get_name(),
                planet.get_position(),
                data.get_color(),
                Some(data.get_geometric_albedo()),
                data.get_radius(),
                view_angle,
                &view_rotation_axis,
                offset,
                display_names,
            );
        }
    }

    fn draw_central_body(
        &self,
        celestial_system: &CelestialSystem,
        frame: &mut canvas::Frame,
        bounds: &iced::Rectangle,
        view_angle: Angle,
        view_rotation_axis: &Direction,
        offset: iced::Vector,
        display_names: bool,
    ) {
        //TODO: draw central body
        let data = celestial_system.get_central_body_data();
        let pos3d = CartesianCoordinates::ORIGIN;
        let color = match data.get_temperature() {
            Some(temperature) => sRGBColor::from_temperature(*temperature),
            None => sRGBColor::from_sRGB(1., 1., 1.),
        };
        let radius = data.get_radius().unwrap_or(Length::ZERO);
        self.draw_body(
            frame,
            bounds,
            data.get_name(),
            &pos3d,
            &color,
            None,
            radius,
            view_angle,
            view_rotation_axis,
            offset,
            display_names,
        );
    }

    fn draw_body(
        &self,
        frame: &mut canvas::Frame,
        bounds: &iced::Rectangle,
        name: &str,
        pos3d: &CartesianCoordinates,
        color: &sRGBColor,
        albedo: Option<Float>,
        radius: Length,
        view_angle: Angle,
        view_rotation_axis: &Direction,
        offset: iced::Vector,
        display_names: bool,
    ) {
        let radius = canvas_radius(&radius);
        let pos =
            frame.center() + self.canvas_position(pos3d, view_angle, &view_rotation_axis) - offset;
        if contains_workaround(bounds, pos) {
            let circle = Path::circle(pos, radius);
            let color = canvas_color(color, albedo);
            frame.fill(&circle, color);

            if display_names {
                draw_name(name, color, pos, frame);
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

fn canvas_radius(radius: &Length) -> f32 {
    const SIZE_NUMBER: f32 = 0.3;
    radius.as_kilometers().powf(SIZE_NUMBER) * SIZE_NUMBER
}

fn canvas_color(color: &sRGBColor, albedo: Option<Float>) -> Color {
    let (r, g, b) = color.maximized_sRGB_tuple();
    let a = albedo.unwrap_or(1.) as f32;
    Color { r, g, b, a }
}
