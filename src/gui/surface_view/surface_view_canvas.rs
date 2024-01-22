use super::{
    star_canvas_appearance::StarCanvasAppearance,
    surface_view_widget::SurfaceViewState,
    viewport::{observer_normal, Viewport},
};
use crate::{
    gui::shared_canvas_functionality::{contains_workaround, draw_background, draw_name},
    model::{celestial_system::CelestialSystem, planet::Planet},
};
use astro_utils::{
    coordinates::{
        cartesian::CartesianCoordinates, direction::Direction, spherical::SphericalCoordinates,
    },
    stars::star_appearance::StarAppearance,
    units::{length::Length, time::Time},
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
            SphericalCoordinates::new(self.view_longitude, self.view_latitude);
        let viewport = Viewport::calculate(
            &observer_normal,
            &observer_view_direction,
            self.viewport_vertical_opening_angle,
            selected_planet.get_data().get_rotation_axis(),
            bounds.height,
        );

        for distant_star in celestial_system.get_distant_star_appearances() {
            self.draw_star(
                frame,
                bounds,
                distant_star,
                &viewport,
                &observer_position,
                viewport.px_per_unit_height,
                display_names,
            );
        }

        self.draw_central_body(
            frame,
            bounds,
            celestial_system,
            &viewport,
            &observer_position,
            viewport.px_per_unit_height,
            display_names,
        );

        for planet in celestial_system.get_planets_at_time(time_since_epoch) {
            self.draw_planets(
                frame,
                bounds,
                celestial_system,
                &planet,
                &viewport,
                &observer_position,
                viewport.px_per_unit_height,
                display_names,
            );
        }
    }

    fn draw_star(
        &self,
        frame: &mut canvas::Frame,
        bounds: iced::Rectangle,
        star: &StarAppearance,
        viewport: &Viewport,
        observer_position: &CartesianCoordinates,
        pixel_per_viewport_width: f32,
        display_names: bool,
    ) {
        let canvas_appearance = StarCanvasAppearance::from_star_appearance(star, viewport);
        self.draw_body(
            frame,
            bounds,
            &canvas_appearance,
            &None,
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
        viewport: &Viewport,
        observer_position: &CartesianCoordinates,
        pixel_per_viewport_width: f32,
        display_names: bool,
    ) {
        let planet_appearance = planet.get_data().to_star_appearance(
            celestial_system.get_central_body_data(),
            &planet.get_position(),
            observer_position,
        );

        let canvas_appearance =
            StarCanvasAppearance::from_star_appearance(&planet_appearance, viewport);
        self.draw_body(
            frame,
            bounds,
            &canvas_appearance,
            &Some(planet.get_data().get_radius()),
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
        viewport: &Viewport,
        observer_position: &CartesianCoordinates,
        pixel_per_viewport_width: f32,
        display_names: bool,
    ) {
        let central_body = celestial_system.get_central_body();
        let central_body_pos = -observer_position;
        let central_body_dir = central_body_pos.to_direction();
        let mut central_body_appearance = central_body.get_appearance().clone();
        central_body_appearance.set_direction_in_ecliptic(central_body_dir);
        let canvas_appearance =
            StarCanvasAppearance::from_star_appearance(&central_body_appearance, viewport);
        self.draw_body(
            frame,
            bounds,
            &canvas_appearance,
            &celestial_system.get_central_body_data().get_radius(),
            pixel_per_viewport_width,
            display_names,
            observer_position,
        );
    }

    fn draw_body(
        &self,
        frame: &mut canvas::Frame,
        bounds: iced::Rectangle,
        canvas_appearance: &Option<StarCanvasAppearance>,
        radius: &Option<Length>,
        pixel_per_viewport_width: f32,
        display_names: bool,
        observer_position: &CartesianCoordinates,
    ) {
        if let Some(canvas_appearance) = canvas_appearance {
            let pos = frame.center() + canvas_appearance.center_offset;
            let color = canvas_appearance.color;
            self.draw_hue(frame, &canvas_appearance);

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
                draw_name(canvas_appearance.name, color, pos, frame);
            }
        }
    }

    fn draw_hue(&self, frame: &mut canvas::Frame, canvas_appearance: &StarCanvasAppearance) {
        // Radial gradients are not yet impelemented in iced.
        let pos: Point = frame.center() + canvas_appearance.center_offset;
        let mut gradient_color = canvas_appearance.color.clone();
        gradient_color.a = (GRADIENT_ALPHA as f32) / (GRADIENT_STEPS as f32);
        for i in 1..=GRADIENT_STEPS {
            let radius = canvas_appearance.radius
                * (i as f32 / (GRADIENT_STEPS as f32)).powi(GRADIENT_SHARPNESS_EXPONENT);
            let brightness_circle = Path::circle(pos, radius);
            frame.fill(&brightness_circle, gradient_color);
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

fn canvas_apparent_radius(
    radius: &Length,
    relative_position: &CartesianCoordinates,
    pixel_per_viewport_width: f32,
) -> f32 {
    radius / relative_position.length() * pixel_per_viewport_width
}
