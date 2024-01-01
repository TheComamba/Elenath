use std::f32::consts::PI;

use crate::model::celestial_body::CelestialBody;

use super::{gui_widget::GuiMessage, Gui};
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
}
