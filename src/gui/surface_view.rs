use std::f32::consts::PI;

use crate::model::celestial_body::CelestialBody;

use super::{Gui, GuiMessage};
use astro_utils::{
    coordinates::{
        direction::{Direction, X, Z},
        ecliptic::EclipticCoordinates,
    },
    surface_normal::apparent_celestial_position,
    Float,
};
use iced::{
    widget::{
        canvas::{self, Path},
        Column,
    },
    Alignment, Color,
};

pub(super) struct SurfaceViewState {
    pub(super) background_cache: canvas::Cache,
    pub(super) bodies_cache: canvas::Cache,
}

impl SurfaceViewState {
    pub(super) fn new() -> Self {
        SurfaceViewState {
            background_cache: canvas::Cache::default(),
            bodies_cache: canvas::Cache::default(),
        }
    }

    pub(super) fn redraw(&mut self) {
        self.background_cache.clear();
        self.bodies_cache.clear();
    }
}

impl Gui {
    pub(super) fn surface_view_control_field(&self) -> iced::Element<'_, GuiMessage> {
        let planet_picker = self.planet_picker();
        Column::new()
            .push(self.time_control_fields())
            .push(planet_picker)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }

    fn surface_view_canvas_position(
        &self,
        body: &CelestialBody,
        observer_normal: &Direction,
        canvas_radius: f32,
    ) -> iced::Vector {
        const PI_HALF: Float = PI / 2.;
        let ecliptic_position = EclipticCoordinates::from_cartesian(body.get_position());
        let surface_position = apparent_celestial_position(&ecliptic_position, observer_normal);
        let x = surface_position.get_longitude().as_radians() / PI_HALF * canvas_radius;
        let y = -surface_position.get_latitude().as_radians() / PI_HALF * canvas_radius; // y axis is inverted
        iced::Vector::new(x as f32, y as f32)
    }

    pub(super) fn surface_view_canvas(
        &self,
        renderer: &iced::Renderer,
        bounds: iced::Rectangle,
    ) -> Vec<canvas::Geometry> {
        let canvas_radius = bounds.size().width.min(bounds.size().height) / 2.;
        let background =
            self.surface_view_state
                .background_cache
                .draw(renderer, bounds.size(), |frame| {
                    let background = Path::circle(frame.center(), canvas_radius);
                    frame.fill(&background, Color::BLACK);
                });

        //TODO: Calculate observer normal
        let observer_normal = X;

        let bodies = self
            .surface_view_state
            .bodies_cache
            .draw(renderer, bounds.size(), |frame| {
                let bodies_path = Path::new(|path_builder| {
                    for body in self.celestial_bodies.iter() {
                        let pos = frame.center()
                            + self.surface_view_canvas_position(
                                body,
                                &observer_normal,
                                canvas_radius,
                            );
                        path_builder.circle(pos, 3.0);
                    }
                });
                frame.fill(&bodies_path, Color::WHITE);
            });
        vec![background, bodies]
    }
}
