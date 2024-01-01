use super::surface_view_widget::SurfaceViewState;
use crate::model::celestial_body::CelestialBody;
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
    Color, Point,
};

impl SurfaceViewState {
    fn observer_data(
        &self,
        selected_body: &Option<CelestialBody>,
        time_since_epoch: Time,
    ) -> (CartesianCoordinates, Direction) {
        let body = match selected_body {
            Some(body) => body,
            None => return (CartesianCoordinates::ORIGIN, Direction::Z),
        };

        let observer_equatorial_position = EquatorialCoordinates::new(
            SphericalCoordinates::new(self.surface_longitude, self.surface_latitude),
            body.get_data().get_rotation_axis().clone(),
        );
        //TODO: Define Angle at Epoch
        let planet_angle_at_epoch = Angle::from_degrees(0.0);
        let observer_normal = surface_normal_at_time(
            observer_equatorial_position,
            planet_angle_at_epoch,
            time_since_epoch,
            body.get_data().get_sideral_rotation_period(),
        );

        let body_radius = body.get_data().get_radius();
        let observer_position =
            body.get_position().clone() + observer_normal.to_cartesian(body_radius);
        (observer_position, observer_normal)
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
                let background = Path::rectangle(Point::ORIGIN, bounds.size());
                frame.fill(&background, Color::BLACK);
            });

        let (observer_position, observer_normal) =
            self.observer_data(selected_body, time_since_epoch);

        let pixel_per_viewport_width = self.pixel_per_viewport_width(bounds.width);
        let bodies = self.bodies_cache.draw(renderer, bounds.size(), |frame| {
            let bodies_path = Path::new(|path_builder| {
                for body in celestial_bodies.iter() {
                    let pos = self.canvas_position(
                        body,
                        &observer_position,
                        &observer_normal,
                        pixel_per_viewport_width,
                    );
                    if let Some(pos) = pos {
                        let pos = frame.center() + pos;
                        path_builder.circle(pos, 3.0);

                        let mut name_widget = canvas::Text::default();
                        name_widget.color = Color::WHITE;
                        name_widget.content = body.get_name().to_string();
                        name_widget.position = pos;
                        frame.fill_text(name_widget);
                    }
                }
            });
            frame.fill(&bodies_path, Color::WHITE);
        });
        vec![background, bodies]
    }
}
