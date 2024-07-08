use astro_coords::{
    cartesian::CartesianCoordinates, direction::Direction, spherical::SphericalCoordinates,
};
use iced::{widget::canvas, Rectangle, Renderer};

use crate::{
    gui::shared_canvas_functionality::{display_info_text, draw_background},
    model::{celestial_system::CelestialSystem, planet::Planet},
};

use super::{
    viewport::{observer_normal, Viewport},
    widget::SurfaceViewState,
};

impl SurfaceViewState {
    pub(super) fn observer_position(
        &self,
        selected_planet: &Planet,
        observer_normal: &Direction,
    ) -> CartesianCoordinates {
        let body_radius = selected_planet.get_data().get_radius();
        selected_planet.get_position().clone() + observer_normal.to_cartesian(body_radius)
    }

    pub(crate) fn canvas(
        &self,
        renderer: &Renderer,
        bounds: Rectangle,
        selected_planet: &Option<Planet>,
        celestial_system: &Option<CelestialSystem>,
        display_names: bool,
        display_constellations: bool,
    ) -> Vec<canvas::Geometry> {
        let background = self
            .background_cache
            .draw(renderer, bounds.size(), |frame| {
                draw_background(bounds, frame);
            });

        let bodies = self.bodies_cache.draw(renderer, bounds.size(), |frame| {
            if let Some(celestial_system) = celestial_system {
                if let Some(selected_planet) = selected_planet {
                    self.draw_surface_view(
                        frame,
                        bounds,
                        selected_planet,
                        celestial_system,
                        display_names,
                        display_constellations,
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

    fn draw_surface_view(
        &self,
        frame: &mut canvas::Frame,
        bounds: Rectangle,
        selected_planet: &Planet,
        celestial_system: &CelestialSystem,
        display_names: bool,
        display_constellations: bool,
    ) {
        let surface_position =
            SphericalCoordinates::new(self.surface_longitude, self.surface_latitude);
        let observer_normal = observer_normal(
            selected_planet.get_data(),
            surface_position,
            celestial_system.get_time_since_epoch(),
        );
        let observer_position = self.observer_position(selected_planet, &observer_normal);
        let observer_view_direction =
            SphericalCoordinates::new(self.view_longitude, self.view_latitude);
        let viewport = Viewport::calculate(
            &observer_normal,
            &observer_view_direction,
            self.viewport_opening_angle,
            selected_planet.get_data().get_rotation_axis(),
            bounds,
        );

        self.draw_bodies(
            frame,
            bounds,
            selected_planet,
            celestial_system,
            display_names,
            &viewport,
            &observer_position,
        );

        if display_constellations {
            self.draw_constellations(frame, bounds, celestial_system, &viewport);
        }
    }
}
