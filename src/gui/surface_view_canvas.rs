use super::{shared_canvas_functionality::draw_body_name, surface_view_widget::SurfaceViewState};
use crate::{
    gui::shared_canvas_functionality::draw_background, model::celestial_body::CelestialBody,
};
use astro_utils::{
    coordinates::{
        cartesian::CartesianCoordinates, direction::Direction, equatorial::EquatorialCoordinates,
        spherical::SphericalCoordinates,
    },
    surface_normal::{direction_relative_to_surface_normal, surface_normal_at_time},
    units::{angle::Angle, time::Time},
    Float,
};
use iced::{
    widget::canvas::{self, Path},
    Color,
};

impl SurfaceViewState {
    fn observer_normal(&self, selected_body: &CelestialBody, time_since_epoch: Time) -> Direction {
        let observer_equatorial_position = EquatorialCoordinates::new(
            SphericalCoordinates::new(self.surface_longitude, self.surface_latitude),
            selected_body.get_rotation_axis().clone(),
        );
        //TODO: Define Angle at Epoch
        let planet_angle_at_epoch = Angle::from_degrees(0.0);
        surface_normal_at_time(
            observer_equatorial_position,
            planet_angle_at_epoch,
            time_since_epoch,
            selected_body.get_sideral_rotation_period(),
        )
    }

    fn observer_position(
        &self,
        selected_body: &CelestialBody,
        observer_normal: &Direction,
    ) -> CartesianCoordinates {
        let body_radius = selected_body.get_radius();
        selected_body.get_position().clone() + observer_normal.to_cartesian(body_radius)
    }

    fn pixel_per_viewport_width(&self, canvas_width: f32) -> Float {
        let opening_angle = self.viewport_horizontal_opening_angle;
        let viewport_width = (opening_angle / 2.).sin() * 2.; //Viewport is at unit distance
        canvas_width / viewport_width
    }

    fn canvas_position(
        &self,
        body: &CelestialBody,
        observer_position: &CartesianCoordinates,
        observer_normal: &Direction,
        pixel_per_viewport_width: Float,
    ) -> Option<iced::Vector> {
        let relative_position = body.get_position() - observer_position;
        let direction = Direction::from_cartesian(&relative_position);
        let direction = direction_relative_to_surface_normal(&direction, observer_normal);
        if direction.z() > 0.0 {
            let x = direction.x() * pixel_per_viewport_width;
            let y = -direction.y() * pixel_per_viewport_width; // y axis is inverted
            Some(iced::Vector::new(x as f32, y as f32))
        } else {
            None
        }
    }

    pub(super) fn canvas(
        &self,
        renderer: &iced::Renderer,
        bounds: iced::Rectangle,
        selected_body: &Option<CelestialBody>,
        time_since_epoch: Time,
        celestial_bodies: &Vec<CelestialBody>,
    ) -> Vec<canvas::Geometry> {
        let background = self
            .background_cache
            .draw(renderer, bounds.size(), |frame| {
                draw_background(bounds, frame);
            });

        let bodies = self.bodies_cache.draw(renderer, bounds.size(), |frame| {
            self.draw_bodies(
                selected_body,
                time_since_epoch,
                bounds,
                celestial_bodies,
                frame,
            );
        });

        vec![background, bodies]
    }

    fn draw_bodies(
        &self,
        selected_body: &Option<CelestialBody>,
        time_since_epoch: Time,
        bounds: iced::Rectangle,
        celestial_bodies: &Vec<CelestialBody>,
        frame: &mut canvas::Frame,
    ) {
        let observer_normal = match selected_body {
            Some(body) => self.observer_normal(body, time_since_epoch),
            None => Direction::Z,
        };
        let observer_position = match selected_body {
            Some(body) => self.observer_position(body, &observer_normal),
            None => CartesianCoordinates::ORIGIN,
        };
        let pixel_per_viewport_width = self.pixel_per_viewport_width(bounds.width);

        let bodies_path = Path::new(|path_builder| {
            for body in celestial_bodies.iter() {
                self.draw_body(
                    body,
                    &observer_position,
                    &observer_normal,
                    pixel_per_viewport_width,
                    frame,
                    path_builder,
                );
            }
        });

        frame.fill(&bodies_path, Color::WHITE);
    }

    fn draw_body(
        &self,
        body: &CelestialBody,
        observer_position: &CartesianCoordinates,
        observer_normal: &Direction,
        pixel_per_viewport_width: f32,
        frame: &mut canvas::Frame,
        path_builder: &mut canvas::path::Builder,
    ) {
        let pos = self.canvas_position(
            body,
            observer_position,
            observer_normal,
            pixel_per_viewport_width,
        );
        if let Some(pos) = pos {
            let pos = frame.center() + pos;
            path_builder.circle(pos, 3.0);

            draw_body_name(body, pos, frame);
        }
    }
}
