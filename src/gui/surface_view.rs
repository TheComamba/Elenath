use std::f32::consts::PI;

use crate::model::celestial_body::CelestialBody;

use super::{Gui, GuiMessage};
use astro_utils::{
    coordinates::{
        cartesian::{CartesianCoordinates, ORIGIN},
        direction::{Direction, X, Z},
        ecliptic::EclipticCoordinates,
        equatorial::EquatorialCoordinates,
    },
    surface_normal::apparent_celestial_position,
    units::angle::Angle,
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
    pub(super) surface_longitude: Angle,
    pub(super) surface_latitude: Angle,
}

impl SurfaceViewState {
    pub(super) fn new() -> Self {
        SurfaceViewState {
            background_cache: canvas::Cache::default(),
            bodies_cache: canvas::Cache::default(),
            surface_longitude: Angle::from_degrees(0.0),
            surface_latitude: Angle::from_degrees(0.0),
        }
    }

    pub(super) fn redraw(&mut self) {
        self.background_cache.clear();
        self.bodies_cache.clear();
    }
}

impl Gui {
    pub(super) fn surface_view_control_field(&self) -> iced::Element<'_, GuiMessage> {
        const SURFACE_ANGLE_STEP: Angle = Angle::from_radians(10. * 2. * PI / 360.);
        let longitude = self.surface_view_state.surface_longitude;
        let surface_longitude_control_field = self.control_field(
            "Surface Longitude:",
            format!("{}", longitude),
            GuiMessage::UpdateSurfaceLongitude(longitude - SURFACE_ANGLE_STEP),
            GuiMessage::UpdateSurfaceLongitude(longitude + SURFACE_ANGLE_STEP),
        );
        let latitude = self.surface_view_state.surface_latitude;
        let surface_latitude_control_field = self.control_field(
            "Surface Latitude:",
            format!("{}", latitude),
            GuiMessage::UpdateSurfaceLatitude(latitude - SURFACE_ANGLE_STEP),
            GuiMessage::UpdateSurfaceLatitude(latitude + SURFACE_ANGLE_STEP),
        );
        let planet_picker = self.planet_picker();
        Column::new()
            .push(self.time_control_fields())
            .push(surface_longitude_control_field)
            .push(surface_latitude_control_field)
            .push(planet_picker)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }

    fn observer_data(&self) -> (CartesianCoordinates, Direction) {
        let body = match self.selected_body.as_ref() {
            Some(body) => body,
            None => return (ORIGIN, Z),
        };

        let observer_normal = X; //TODO: Calculate observer normal

        let focus_radius = body.get_data().get_radius();
        let observer_position =
            body.get_position().clone() + observer_normal.to_cartesian(focus_radius);
        (observer_position, observer_normal)
    }

    fn surface_view_canvas_position(
        body: &CelestialBody,
        observer_position: &CartesianCoordinates,
        observer_normal: &Direction,
        canvas_radius: f32,
    ) -> iced::Vector {
        const PI_HALF: Float = PI / 2.;
        let relative_position = body.get_position() - observer_position;
        let ecliptic_position = EclipticCoordinates::from_cartesian(&relative_position);
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

        let (observer_position, observer_normal) = self.observer_data();

        let bodies = self
            .surface_view_state
            .bodies_cache
            .draw(renderer, bounds.size(), |frame| {
                let bodies_path = Path::new(|path_builder| {
                    for body in self.celestial_bodies.iter() {
                        let pos = frame.center()
                            + Self::surface_view_canvas_position(
                                body,
                                &observer_position,
                                &observer_normal,
                                canvas_radius,
                            );
                        path_builder.circle(pos, 3.0);

                        let mut name_widget = canvas::Text::default();
                        name_widget.color = Color::WHITE;
                        name_widget.content = body.get_name().to_string();
                        name_widget.position = pos;
                        frame.fill_text(name_widget);
                    }
                });
                frame.fill(&bodies_path, Color::WHITE);
            });
        vec![background, bodies]
    }
}
