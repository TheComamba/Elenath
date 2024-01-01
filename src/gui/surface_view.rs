use std::f32::consts::PI;

use crate::model::celestial_body::CelestialBody;

use super::{Gui, GuiMessage};
use astro_utils::{
    coordinates::{
        cartesian::CartesianCoordinates, direction::Direction, equatorial::EquatorialCoordinates,
        spherical::SphericalCoordinates,
    },
    surface_normal::{direction_relative_to_surface_normal, surface_normal_at_time},
    units::angle::Angle,
    Float,
};
use iced::{
    widget::{
        canvas::{self, Path},
        Column,
    },
    Alignment, Color, Point,
};

const HUMAN_EYE_OPENING_ANGLE: Angle = Angle::from_radians(120. / 360. * 2. * PI);

pub(super) struct SurfaceViewState {
    pub(super) background_cache: canvas::Cache,
    pub(super) bodies_cache: canvas::Cache,
    pub(super) surface_longitude: Angle,
    pub(super) surface_latitude: Angle,
    pub(super) viewport_horizontal_opening_angle: Angle,
}

impl SurfaceViewState {
    pub(super) fn new() -> Self {
        SurfaceViewState {
            background_cache: canvas::Cache::default(),
            bodies_cache: canvas::Cache::default(),
            surface_longitude: Angle::from_degrees(0.0),
            surface_latitude: Angle::from_degrees(0.0),
            viewport_horizontal_opening_angle: HUMAN_EYE_OPENING_ANGLE,
        }
    }

    pub(super) fn redraw(&mut self) {
        self.background_cache.clear();
        self.bodies_cache.clear();
    }
}

impl Gui {
    pub(super) fn surface_view_control_field(&self) -> iced::Element<'_, GuiMessage> {
        const ANGLE_STEP: Angle = Angle::from_radians(10. * 2. * PI / 360.);
        let longitude = self.surface_view_state.surface_longitude;
        let surface_longitude_control_field = self.control_field(
            "Surface Longitude:",
            format!("{}", longitude),
            GuiMessage::UpdateSurfaceLongitude(longitude - ANGLE_STEP),
            GuiMessage::UpdateSurfaceLongitude(longitude + ANGLE_STEP),
        );
        let latitude = self.surface_view_state.surface_latitude;
        let surface_latitude_control_field = self.control_field(
            "Surface Latitude:",
            format!("{}", latitude),
            GuiMessage::UpdateSurfaceLatitude(latitude - ANGLE_STEP),
            GuiMessage::UpdateSurfaceLatitude(latitude + ANGLE_STEP),
        );
        let viewport_angle = self.surface_view_state.viewport_horizontal_opening_angle;
        let viewport_angle_control_field = self.control_field(
            "Horizontal Viewport Opening Angle:",
            format!("{}", viewport_angle),
            GuiMessage::UpdateViewportOpeningAngle(viewport_angle - ANGLE_STEP),
            GuiMessage::UpdateViewportOpeningAngle(viewport_angle + ANGLE_STEP),
        );
        let planet_picker = self.planet_picker();
        Column::new()
            .push(self.time_control_fields())
            .push(surface_longitude_control_field)
            .push(surface_latitude_control_field)
            .push(viewport_angle_control_field)
            .push(planet_picker)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }

    fn observer_data(&self) -> (CartesianCoordinates, Direction) {
        let body = match self.selected_body.as_ref() {
            Some(body) => body,
            None => return (CartesianCoordinates::ORIGIN, Direction::Z),
        };

        let observer_equatorial_position = EquatorialCoordinates::new(
            SphericalCoordinates::new(
                self.surface_view_state.surface_longitude,
                self.surface_view_state.surface_latitude,
            ),
            body.get_data().get_rotation_axis().clone(),
        );
        //TODO: Define Angle at Epoch
        let planet_angle_at_epoch = Angle::from_degrees(0.0);
        let observer_normal = surface_normal_at_time(
            observer_equatorial_position,
            planet_angle_at_epoch,
            self.time_since_epoch,
            body.get_data().get_sideral_rotation_period(),
        );

        let body_radius = body.get_data().get_radius();
        let observer_position =
            body.get_position().clone() + observer_normal.to_cartesian(body_radius);
        (observer_position, observer_normal)
    }

    fn pixel_per_viewport_width(&self, canvas_width: f32) -> Float {
        let opening_angle = self.surface_view_state.viewport_horizontal_opening_angle;
        let viewport_width = (opening_angle / 2.).sin() * 2.; //Viewport is at unit distance
        canvas_width / viewport_width
    }

    fn surface_view_canvas_position(
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

    pub(super) fn surface_view_canvas(
        &self,
        renderer: &iced::Renderer,
        bounds: iced::Rectangle,
    ) -> Vec<canvas::Geometry> {
        let background =
            self.surface_view_state
                .background_cache
                .draw(renderer, bounds.size(), |frame| {
                    let background = Path::rectangle(Point::ORIGIN, bounds.size());
                    frame.fill(&background, Color::BLACK);
                });

        let (observer_position, observer_normal) = self.observer_data();

        let pixel_per_viewport_width = self.pixel_per_viewport_width(bounds.width);
        let bodies = self
            .surface_view_state
            .bodies_cache
            .draw(renderer, bounds.size(), |frame| {
                let bodies_path = Path::new(|path_builder| {
                    for body in self.celestial_bodies.iter() {
                        let pos = self.surface_view_canvas_position(
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
