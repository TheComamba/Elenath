use super::{
    star_canvas_appearance::{brightness_radius, canvas_color},
    surface_view_widget::SurfaceViewState,
    viewport::observer_normal,
};
use crate::{
    gui::shared_canvas_functionality::{contains_workaround, draw_background, draw_name},
    model::{celestial_system::CelestialSystem, planet::Planet},
};
use astro_utils::{
    coordinates::{
        cartesian::CartesianCoordinates, direction::Direction, spherical::SphericalCoordinates,
    },
    planets::surface_normal::direction_relative_to_surface_normal,
    stars::star_appearance::StarAppearance,
    units::{length::Length, time::Time},
    Float,
};
use iced::{
    widget::canvas::{self, Path},
    Color, Point,
};

const GRADIENT_ALPHA: f32 = 1.;
const GRADIENT_STEPS: i32 = 10;
const GRADIENT_SHARPNESS_EXPONENT: i32 = 2;

impl SurfaceViewState {
    fn observer_position(
        &self,
        selected_planet: &Planet,
        observer_normal: &Direction,
    ) -> CartesianCoordinates {
        let body_radius = selected_planet.get_data().get_radius();
        selected_planet.get_position().clone() + observer_normal.to_cartesian(body_radius)
    }

    fn pixel_per_viewport_width(&self, canvas_width: f32) -> Float {
        let opening_angle = self.viewport_horizontal_opening_angle;
        let viewport_width = (opening_angle / 2.).sin() * 2.; //Viewport is at unit distance
        canvas_width / viewport_width
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

        let bodies = if let Some(selected_planet) = selected_planet {
            self.bodies_cache.draw(renderer, bounds.size(), |frame| {
                self.draw_bodies(
                    frame,
                    bounds,
                    selected_planet,
                    celestial_system,
                    time_since_epoch,
                    display_names,
                );
            })
        } else {
            self.bodies_cache.draw(renderer, bounds.size(), |frame| {
                display_planet_selection_text(frame);
            })
        };

        vec![background, bodies]
    }

    fn draw_bodies(
        &self,
        frame: &mut canvas::Frame,
        bounds: iced::Rectangle,
        selected_planet: &Planet,
        celestial_system: &CelestialSystem,
        time_since_epoch: Time,
        display_names: bool,
    ) {
        let surface_position =
            SphericalCoordinates::new(self.surface_longitude, self.surface_latitude);
        let observer_normal = observer_normal(
            selected_planet.get_data(),
            surface_position,
            time_since_epoch,
        );
        let observer_position = self.observer_position(&selected_planet, &observer_normal);
        let observer_view_direction =
            SphericalCoordinates::new(self.view_longitude, self.view_latitude).to_direction();
        let pixel_per_viewport_width = self.pixel_per_viewport_width(bounds.width);

        for distant_star in celestial_system.get_distant_star_appearances() {
            self.draw_star(
                frame,
                bounds,
                distant_star,
                &observer_position,
                &observer_view_direction,
                pixel_per_viewport_width,
                display_names,
            );
        }

        self.draw_central_body(
            frame,
            bounds,
            celestial_system,
            &observer_position,
            &observer_view_direction,
            pixel_per_viewport_width,
            display_names,
        );

        for planet in celestial_system.get_planets_at_time(time_since_epoch) {
            self.draw_planets(
                frame,
                bounds,
                celestial_system,
                &planet,
                &observer_position,
                &observer_view_direction,
                pixel_per_viewport_width,
                display_names,
            );
        }
    }

    fn draw_star(
        &self,
        frame: &mut canvas::Frame,
        bounds: iced::Rectangle,
        star: &StarAppearance,
        observer_position: &CartesianCoordinates,
        observer_view_direction: &Direction,
        pixel_per_viewport_width: f32,
        display_names: bool,
    ) {
        let pos = canvas_position(
            star.get_direction_in_ecliptic(),
            observer_view_direction,
            pixel_per_viewport_width,
        );
        self.draw_body(
            pos,
            frame,
            bounds,
            star,
            &None,
            observer_view_direction,
            pixel_per_viewport_width,
            display_names,
            observer_position,
        );
    }

    fn draw_planets(
        &self,
        frame: &mut canvas::Frame,
        bounds: iced::Rectangle,
        celestial_system: &CelestialSystem,
        planet: &Planet,
        observer_position: &CartesianCoordinates,
        observer_view_direction: &Direction,
        pixel_per_viewport_width: f32,
        display_names: bool,
    ) {
        let planet_appearance = planet.get_data().to_star_appearance(
            celestial_system.get_central_body_data(),
            &planet.get_position(),
            observer_position,
        );
        let pos = canvas_position(
            planet_appearance.get_direction_in_ecliptic(),
            observer_view_direction,
            pixel_per_viewport_width,
        );
        self.draw_body(
            pos,
            frame,
            bounds,
            &planet_appearance,
            &Some(planet.get_data().get_radius()),
            observer_view_direction,
            pixel_per_viewport_width,
            display_names,
            observer_position,
        );
    }

    fn draw_central_body(
        &self,
        frame: &mut canvas::Frame,
        bounds: iced::Rectangle,
        celestial_system: &CelestialSystem,
        observer_position: &CartesianCoordinates,
        observer_view_direction: &Direction,
        pixel_per_viewport_width: f32,
        display_names: bool,
    ) {
        let central_body = celestial_system.get_central_body();
        let central_body_pos = -observer_position;
        let central_body_dir = central_body_pos.to_direction();
        let mut central_body_appearance = central_body.get_appearance().clone();
        central_body_appearance.set_direction_in_ecliptic(central_body_dir);
        let pos = canvas_position(
            central_body_appearance.get_direction_in_ecliptic(),
            observer_view_direction,
            pixel_per_viewport_width,
        );
        self.draw_body(
            pos,
            frame,
            bounds,
            &central_body_appearance,
            &celestial_system.get_central_body_data().get_radius(),
            observer_view_direction,
            pixel_per_viewport_width,
            display_names,
            observer_position,
        );
    }

    fn draw_body(
        &self,
        pos: Option<iced::Vector>,
        frame: &mut canvas::Frame,
        bounds: iced::Rectangle,
        appearance: &StarAppearance,
        radius: &Option<Length>,
        observer_view_direction: &Direction,
        pixel_per_viewport_width: f32,
        display_names: bool,
        observer_position: &CartesianCoordinates,
    ) {
        if let Some(pos) = pos {
            let pos = frame.center() + pos;
            let color = canvas_color(&appearance);
            self.draw_hue(
                frame,
                &appearance,
                observer_view_direction,
                pixel_per_viewport_width,
            );

            if !contains_workaround(&bounds, pos) {
                return;
            }

            if let Some(radius) = radius {
                let relative_position = -observer_position;
                self.draw_disk(
                    frame,
                    pos,
                    &radius,
                    &relative_position,
                    color,
                    pixel_per_viewport_width,
                );
            }

            if display_names {
                draw_name(appearance.get_name(), color, pos, frame);
            }
        }
    }

    fn draw_hue(
        &self,
        frame: &mut canvas::Frame,
        appearance: &StarAppearance,
        observer_view_direction: &Direction,
        pixel_per_viewport_width: f32,
    ) {
        let pos = canvas_position(
            &appearance.get_direction_in_ecliptic(),
            observer_view_direction,
            pixel_per_viewport_width,
        );
        if let Some(pos) = pos {
            let pos: Point = frame.center() + pos;
            let color = canvas_color(appearance);
            let brightness = appearance.get_illuminance();
            let brightness_radius = brightness_radius(&brightness);
            fake_gradient(color, brightness_radius, pos, frame);
        }
    }

    fn draw_disk(
        &self,
        frame: &mut canvas::Frame,
        pos: Point,
        radius: &Length,
        relative_position: &CartesianCoordinates,
        color: Color,
        pixel_per_viewport_width: f32,
    ) {
        let apparent_radius =
            canvas_apparent_radius(radius, &relative_position, pixel_per_viewport_width);

        let solid_circle = Path::circle(pos, apparent_radius);
        frame.fill(&solid_circle, color);
    }
}

fn display_planet_selection_text(frame: &mut canvas::Frame) {
    let mut name_widget = canvas::Text::default();
    name_widget.size = 30.0;
    name_widget.color = Color::WHITE;
    name_widget.content = "Please select a planet.".to_string();
    name_widget.position = frame.center();
    frame.fill_text(name_widget)
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
    direction_in_ecliptic: &Direction,
    observer_view_direction: &Direction,
    pixel_per_viewport_width: Float,
) -> Option<iced::Vector> {
    let direction = direction_relative_to_surface_normal(
        &direction_in_ecliptic,
        observer_view_direction,
        &Direction::Z,
    );
    if direction.z() > 0.0 {
        let x = direction.x() * pixel_per_viewport_width;
        let y = -direction.y() * pixel_per_viewport_width; // y axis is inverted
        Some(iced::Vector::new(x as f32, y as f32))
    } else {
        None
    }
}

fn canvas_apparent_radius(
    radius: &Length,
    relative_position: &CartesianCoordinates,
    pixel_per_viewport_width: f32,
) -> f32 {
    radius / relative_position.length() * pixel_per_viewport_width
}
