use super::widget::SurfaceViewState;
use crate::{
    gui::shared_canvas_functionality::{display_info_text, draw_background},
    model::{celestial_system::CelestialSystem, planet::Planet},
};
use astro_utils::coordinates::{cartesian::CartesianCoordinates, direction::Direction};
use iced::widget::canvas::{self};
use simple_si_units::base::Time;

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
}
