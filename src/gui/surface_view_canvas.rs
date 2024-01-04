use super::{
    shared_canvas_functionality::{draw_body_name, maximized_color},
    surface_view_widget::SurfaceViewState,
};
use crate::{
    gui::shared_canvas_functionality::draw_background,
    model::celestial_body::{CelestialBody, CelestialBodyData},
};
use astro_utils::{
    coordinates::{
        cartesian::CartesianCoordinates, direction::Direction, equatorial::EquatorialCoordinates,
        spherical::SphericalCoordinates,
    },
    planet_brightness::planet_brightness,
    stellar_properties::StellarProperties,
    surface_normal::{direction_relative_to_surface_normal, surface_normal_at_time},
    units::{angle::Angle, illuminance::Illuminance, time::Time},
    Float,
};
use iced::{
    widget::canvas::{self, Path},
    Color, Point,
};

const BRIGHTNESS_EXPONENT: f32 = 0.25;
const BRIGHTNESS_FACTOR: f32 = 8e1;
const GRADIENT_ALPHA: f32 = 1.;
const GRADIENT_STEPS: i32 = 100;
const GRADIENT_SHARPNESS_EXPONENT: i32 = 3;

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

    pub(super) fn canvas(
        &self,
        renderer: &iced::Renderer,
        bounds: iced::Rectangle,
        central_body: &StellarProperties,
        selected_body: &Option<CelestialBody>,
        time_since_epoch: Time,
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
                central_body,
                selected_body,
                time_since_epoch,
                bounds,
                celestial_bodies,
                frame,
                display_names,
            );
        });

        vec![background, bodies]
    }

    fn draw_bodies(
        &self,
        central_body: &StellarProperties,
        selected_body: &Option<CelestialBody>,
        time_since_epoch: Time,
        bounds: iced::Rectangle,
        celestial_bodies: &Vec<CelestialBody>,
        frame: &mut canvas::Frame,
        display_names: bool,
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

        for body in celestial_bodies.iter() {
            self.draw_body(
                central_body,
                body,
                &observer_position,
                &observer_normal,
                pixel_per_viewport_width,
                frame,
                display_names,
            );
        }
    }

    fn draw_body(
        &self,
        central_body: &StellarProperties,
        body: &CelestialBody,
        observer_position: &CartesianCoordinates,
        observer_normal: &Direction,
        pixel_per_viewport_width: f32,
        frame: &mut canvas::Frame,
        display_names: bool,
    ) {
        let relative_position = body.get_position() - observer_position;
        let pos = canvas_position(
            &relative_position,
            observer_normal,
            pixel_per_viewport_width,
        );
        if let Some(pos) = pos {
            let brightness = body_brightness(central_body, body, observer_position);
            let color = maximized_color(body);
            let apparent_radius =
                canvas_apparent_radius(body, &relative_position, pixel_per_viewport_width);
            let pos = frame.center() + pos;

            let solid_circle = Path::circle(pos, apparent_radius);
            frame.fill(&solid_circle, color);

            let brightness_radius = canvas_brightness_radius(&brightness);
            fake_gradient(color, brightness_radius, pos, frame);

            if display_names {
                draw_body_name(body, color, pos, apparent_radius, frame);
            }
        }
    }
}

// Radial gradients are not yet impelemented in iced.
fn fake_gradient(color: Color, brightness_radius: f32, pos: Point, frame: &mut canvas::Frame) {
    let mut gradient_color = color.clone();
    gradient_color.a = (GRADIENT_ALPHA as f32) / (GRADIENT_STEPS as f32);
    for i in 1..=GRADIENT_STEPS {
        let radius = brightness_radius
            * (i as f32 / (GRADIENT_STEPS as f32)).powi(GRADIENT_SHARPNESS_EXPONENT);
        let brightness_circle = Path::circle(pos, radius);
        frame.fill(&brightness_circle, gradient_color);
    }
}

fn canvas_position(
    relative_position: &CartesianCoordinates,
    observer_normal: &Direction,
    pixel_per_viewport_width: Float,
) -> Option<iced::Vector> {
    let direction = relative_position.to_direction();
    let direction = direction_relative_to_surface_normal(&direction, observer_normal);
    if direction.z() > 0.0 {
        let x = direction.x() * pixel_per_viewport_width;
        let y = -direction.y() * pixel_per_viewport_width; // y axis is inverted
        Some(iced::Vector::new(x as f32, y as f32))
    } else {
        None
    }
}

fn body_brightness(
    central_body: &StellarProperties,
    body: &CelestialBody,
    observer_position: &CartesianCoordinates,
) -> Illuminance {
    match body.get_data() {
        CelestialBodyData::CentralBody(data) => {
            let distance = body.get_position() - observer_position;
            data.get_absolute_magnitude()
                .to_illuminance(&distance.length())
        }
        CelestialBodyData::DistantStar(data) => {
            let distance = body.get_position() - observer_position;
            data.get_absolute_magnitude()
                .to_illuminance(&distance.length())
        }
        CelestialBodyData::Planet(data) => planet_brightness(
            central_body.get_absolute_magnitude(),
            &CartesianCoordinates::ORIGIN,
            body.get_position(),
            observer_position,
            data.get_radius(),
            data.get_geometric_albedo(),
        ),
    }
}

fn canvas_apparent_radius(
    body: &CelestialBody,
    relative_position: &CartesianCoordinates,
    pixel_per_viewport_width: f32,
) -> f32 {
    body.get_radius() / relative_position.length() * pixel_per_viewport_width
}

fn canvas_brightness_radius(brightness: &Illuminance) -> f32 {
    let size = brightness.as_lux().powf(BRIGHTNESS_EXPONENT) * BRIGHTNESS_FACTOR;
    if size > 1e4 {
        1e4
    } else {
        size
    }
}
