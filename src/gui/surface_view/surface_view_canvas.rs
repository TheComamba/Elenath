use super::{
    star_canvas_appearance::StarCanvasAppearance,
    surface_view_widget::SurfaceViewState,
    viewport::{observer_normal, Viewport},
};
use crate::{
    gui::shared_canvas_functionality::{
        contains_workaround, display_info_text, draw_background, draw_name,
    },
    model::{celestial_system::CelestialSystem, planet::Planet},
};
use astro_utils::{
    coordinates::{
        cartesian::CartesianCoordinates, direction::Direction, spherical::SphericalCoordinates,
    },
    stars::star_appearance::StarAppearance,
};
use iced::{
    widget::canvas::{self, Path},
    Color, Point,
};
use simple_si_units::base::{Distance, Time};

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
        celestial_system: &Option<CelestialSystem>,
        time_since_epoch: Time<f64>,
        display_names: bool,
    ) -> Vec<canvas::Geometry> {
        let background = self
            .background_cache
            .draw(renderer, bounds.size(), |frame| {
                draw_background(bounds, frame);
            });

        let bodies = self.bodies_cache.draw(renderer, bounds.size(), |frame| {
            if let Some(celestial_system) = celestial_system {
                if let Some(selected_planet) = selected_planet {
                    self.draw_bodies(
                        frame,
                        bounds,
                        selected_planet,
                        celestial_system,
                        time_since_epoch,
                        display_names,
                    );
                } else {
                    display_info_text(frame, "Please select a planet.");
                }
            } else {
                display_info_text(frame, "Please load or generate a celestial system.");
            }
        });

        vec![background, bodies]
    }

    fn draw_bodies(
        &self,
        frame: &mut canvas::Frame,
        bounds: iced::Rectangle,
        selected_planet: &Planet,
        celestial_system: &CelestialSystem,
        time_since_epoch: Time<f64>,
        display_names: bool,
    ) {
        let surface_position =
            SphericalCoordinates::new(self.surface_longitude, self.surface_latitude);
        let observer_normal = observer_normal(
            selected_planet.get_data(),
            surface_position,
            time_since_epoch,
        );
        let observer_position = self.observer_position(selected_planet, &observer_normal);
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
            StarCanvasAppearance::from_central_body(celestial_system, viewport, observer_position);
        self.draw_body(
            frame,
            bounds,
            &canvas_appearance,
            celestial_system.get_central_body_data().get_radius(),
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
        let canvas_appearance = StarCanvasAppearance::from_planet(
            celestial_system,
            planet,
            viewport,
            observer_position,
        );
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
        canvas_appearance: &Option<StarCanvasAppearance>,
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

    fn draw_hue(&self, frame: &mut canvas::Frame, canvas_appearance: &StarCanvasAppearance) {
        // Radial gradients are not yet impelemented in iced.
        let mut step_width = StarCanvasAppearance::MIN_RADIUS;
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
