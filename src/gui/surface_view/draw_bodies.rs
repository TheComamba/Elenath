use astro_utils::{
    coordinates::cartesian::CartesianCoordinates, stars::appearance::StarAppearance,
};
use iced::{
    widget::canvas::{self, Frame, Path},
    Color, Point, Rectangle,
};
use simple_si_units::base::{Distance, Time};

use crate::{
    gui::shared_canvas_functionality::{contains_workaround, draw_name},
    model::{celestial_system::CelestialSystem, planet::Planet},
};

use super::{canvas_appearance::CanvasAppearance, viewport::Viewport, widget::SurfaceViewState};

impl SurfaceViewState {
    pub(super) fn draw_bodies(
        &self,
        frame: &mut Frame,
        bounds: Rectangle,
        selected_planet: &Planet,
        celestial_system: &CelestialSystem,
        display_names: bool,
        viewport: &Viewport,
        observer_position: &CartesianCoordinates,
    ) {
        for distant_star in celestial_system.get_distant_star_appearances() {
            self.draw_star(
                frame,
                bounds,
                distant_star,
                &viewport,
                &observer_position,
                viewport.px_per_distance,
                display_names,
                celestial_system.get_time_since_epoch(),
            );
        }

        self.draw_central_body(
            frame,
            bounds,
            celestial_system,
            &viewport,
            &observer_position,
            viewport.px_per_distance,
            display_names,
        );

        for planet in celestial_system.get_planets() {
            if planet.get_data() == selected_planet.get_data() {
                continue;
            }
            self.draw_planet(
                frame,
                bounds,
                celestial_system,
                &planet,
                &viewport,
                &observer_position,
                viewport.px_per_distance,
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
        time_since_epoch: Time<f64>,
    ) {
        let canvas_appearance = CanvasAppearance::from_star_appearance(star, viewport);
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
        let canvas_appearance =
            CanvasAppearance::from_central_body(celestial_system, viewport, observer_position);
        let central_body_radius = celestial_system
            .get_central_body_data()
            .get_radius(celestial_system.get_time_since_epoch());
        self.draw_body(
            frame,
            bounds,
            &canvas_appearance,
            &central_body_radius,
            pixel_per_viewport_width,
            display_names,
            observer_position,
        );
    }

    fn draw_planet(
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
        let canvas_appearance =
            CanvasAppearance::from_planet(celestial_system, planet, viewport, observer_position);
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

    fn draw_body(
        &self,
        frame: &mut canvas::Frame,
        bounds: iced::Rectangle,
        canvas_appearance: &Option<CanvasAppearance>,
        radius: &Option<Distance<f64>>,
        pixel_per_viewport_width: f32,
        display_names: bool,
        observer_position: &CartesianCoordinates,
    ) {
        if let Some(canvas_appearance) = canvas_appearance {
            let pos = frame.center() + canvas_appearance.center_offset;
            let color = canvas_appearance.color;
            self.draw_hue(frame, canvas_appearance);

            if !contains_workaround(&bounds, pos) {
                return;
            }

            if let Some(radius) = radius {
                let relative_position = -observer_position;
                self.draw_disk(
                    frame,
                    pos,
                    radius,
                    &relative_position,
                    color,
                    pixel_per_viewport_width,
                );
            }

            if display_names {
                draw_name(&canvas_appearance.name, color, pos, frame);
            }
        }
    }

    fn draw_hue(&self, frame: &mut canvas::Frame, canvas_appearance: &CanvasAppearance) {
        // Radial gradients are not yet impelemented in iced.
        let mut step_width = CanvasAppearance::MIN_RADIUS;
        const MAX_STEPS: i32 = 100;
        let mut steps = (0.99 * canvas_appearance.radius / step_width).ceil() as i32;
        if steps > MAX_STEPS {
            steps = MAX_STEPS;
            step_width = canvas_appearance.radius / steps as f32;
        }
        let pos: Point = frame.center() + canvas_appearance.center_offset;
        let mut color = canvas_appearance.color;
        color.a /= steps as f32;
        for i in 0..steps {
            let mut radius = step_width * (i + 1) as f32;
            if radius > canvas_appearance.radius {
                radius = canvas_appearance.radius;
            }
            let circle = Path::circle(pos, radius);
            frame.fill(&circle, color);
        }
    }

    fn draw_disk(
        &self,
        frame: &mut canvas::Frame,
        pos: Point,
        radius: &Distance<f64>,
        relative_position: &CartesianCoordinates,
        color: Color,
        pixel_per_viewport_width: f32,
    ) {
        let apparent_radius =
            canvas_apparent_radius(radius, relative_position, pixel_per_viewport_width);

        let solid_circle = Path::circle(pos, apparent_radius);
        frame.fill(&solid_circle, color);
    }
}

fn canvas_apparent_radius(
    radius: &Distance<f64>,
    relative_position: &CartesianCoordinates,
    pixel_per_viewport_width: f32,
) -> f32 {
    (radius / &relative_position.length()) as f32 * pixel_per_viewport_width
}
