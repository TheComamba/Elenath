use super::widget::TopViewState;
use crate::{
    gui::shared_canvas_functionality::{
        canvas_contains, display_info_text, draw_background, draw_name,
    },
    model::{celestial_system::CelestialSystem, planet::Planet},
};
use astro_utils::{
    astro_display::AstroDisplay,
    color::srgb::sRGBColor,
    coordinates::{
        cartesian::CartesianCoordinates, direction::Direction,
        transformations::rotations::get_rotation_parameters,
    },
    units::distance::DISTANCE_ZERO,
};
use iced::{
    alignment::Horizontal,
    widget::canvas::{self, Path, Style},
    Color, Point, Rectangle, Renderer, Vector,
};
use simple_si_units::{base::Distance, geometry::Angle};

impl TopViewState {
    fn canvas_position(
        &self,
        pos: &CartesianCoordinates,
        view_angle: Angle<f64>,
        view_rotation_axis: &Direction,
    ) -> Vector {
        let rotated_position = pos.rotated(-view_angle, view_rotation_axis); //passive transformation
        let x = rotated_position.x() / self.length_per_pixel;
        let y = -rotated_position.y() / self.length_per_pixel; // y axis is inverted
        Vector::new(x as f32, y as f32)
    }

    pub(crate) fn canvas(
        &self,
        renderer: &Renderer,
        bounds: Rectangle,
        selected_planet: &Option<Planet>,
        celestial_system: &Option<CelestialSystem>,
        display_names: bool,
    ) -> Vec<canvas::Geometry> {
        let background = self
            .background_cache
            .draw(renderer, bounds.size(), |frame| {
                draw_background(bounds, frame);
            });

        let bodies = self.bodies_cache.draw(renderer, bounds.size(), |frame| {
            if let Some(celestial_system) = celestial_system {
                self.draw_bodies(
                    selected_planet,
                    celestial_system,
                    &bounds,
                    frame,
                    display_names,
                );
            } else {
                display_info_text(frame, "Please load or generate a celestial system.");
            }
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
        bounds: &Rectangle,
        frame: &mut canvas::Frame,
        display_names: bool,
    ) {
        let view_direction = &self.view_ecliptic.get_spherical().to_direction();
        let (view_angle, view_rotation_axis) =
            get_rotation_parameters(&Direction::Z, view_direction);

        let offset = match selected_planet {
            Some(focus) => {
                self.canvas_position(focus.get_position(), view_angle, &view_rotation_axis)
            }
            None => Vector::new(0.0, 0.0),
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

        for planet in celestial_system.get_planets().iter() {
            let data = planet.get_data();
            self.draw_body(
                frame,
                bounds,
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
        bounds: &Rectangle,
        view_angle: Angle<f64>,
        view_rotation_axis: &Direction,
        offset: Vector,
        display_names: bool,
    ) {
        let time = celestial_system.get_time_since_epoch();
        let data = celestial_system.get_central_body_data();
        let pos3d = CartesianCoordinates::ORIGIN;
        let color = sRGBColor::from_temperature(data.get_temperature(time));
        let radius = data.get_radius(time).unwrap_or(DISTANCE_ZERO);
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
        bounds: &Rectangle,
        name: &str,
        pos3d: &CartesianCoordinates,
        color: &sRGBColor,
        albedo: Option<f64>,
        radius: Distance<f64>,
        view_angle: Angle<f64>,
        view_rotation_axis: &Direction,
        offset: Vector,
        display_names: bool,
    ) {
        let radius = canvas_radius(&radius);
        let pos =
            frame.center() + self.canvas_position(pos3d, view_angle, view_rotation_axis) - offset;
        if canvas_contains(bounds, pos) {
            let circle = Path::circle(pos, radius);
            let color = canvas_color(color, albedo);
            frame.fill(&circle, color);

            if display_names {
                draw_name(name, color, pos, frame);
            }
        }
    }

    fn draw_scale(&self, bounds: Rectangle, frame: &mut canvas::Frame) {
        const LENGTH_IN_PX: f32 = 200.0;
        let start_pos = Point::ORIGIN + Vector::new(50., bounds.height - 50.);
        let middle_pos = start_pos + Vector::new(LENGTH_IN_PX / 2., 0.0);
        let end_pos = start_pos + Vector::new(LENGTH_IN_PX, 0.0);
        let delimitor_vec = Vector::new(0.0, 5.);

        let scale = Path::new(|path_builder| {
            path_builder.move_to(start_pos + delimitor_vec);
            path_builder.line_to(start_pos - delimitor_vec);
            path_builder.move_to(start_pos);
            path_builder.line_to(end_pos);
            path_builder.move_to(end_pos + delimitor_vec);
            path_builder.line_to(end_pos - delimitor_vec);
        });
        let stroke = canvas::Stroke {
            style: Style::Solid(Color::WHITE),
            ..Default::default()
        };
        frame.stroke(&scale, stroke);

        let text = canvas::Text {
            color: Color::WHITE,
            content: (LENGTH_IN_PX * self.length_per_pixel).astro_display(),
            position: middle_pos,
            horizontal_alignment: Horizontal::Center,
            ..Default::default()
        };
        frame.fill_text(text);
    }
}

fn canvas_radius(radius: &Distance<f64>) -> f32 {
    const SIZE_NUMBER: f32 = 0.3;
    (radius.to_km() as f32).powf(SIZE_NUMBER) * SIZE_NUMBER
}

fn canvas_color(color: &sRGBColor, albedo: Option<f64>) -> Color {
    let (r, g, b) = color.maximized_sRGB_tuple();
    let a = albedo.unwrap_or(1.) as f32;
    Color {
        r: r as f32,
        g: g as f32,
        b: b as f32,
        a,
    }
}
