use crate::gui::shared_widgets::control_field;

use super::gui_widget::GuiMessage;
use astro_utils::units::angle::Angle;
use iced::{
    widget::{
        canvas::{self},
        Column,
    },
    Alignment,
};
use std::f32::consts::PI;

const HUMAN_EYE_OPENING_ANGLE: Angle = Angle::from_radians(120. / 360. * 2. * PI);

pub(super) struct SurfaceViewState {
    pub(super) background_cache: canvas::Cache,
    pub(super) bodies_cache: canvas::Cache,
    pub(super) surface_longitude: Angle,
    pub(super) surface_latitude: Angle,
    pub(super) viewport_horizontal_opening_angle: Angle,
}

#[derive(Debug, Clone)]
pub(crate) enum SurfaceViewMessage {
    UpdateSurfaceLongitude(Angle),
    UpdateSurfaceLatitude(Angle),
    UpdateViewportOpeningAngle(Angle),
}

impl Into<GuiMessage> for SurfaceViewMessage {
    fn into(self) -> GuiMessage {
        GuiMessage::UpdateSurfaceView(self)
    }
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

    pub(super) fn update(&mut self, message: SurfaceViewMessage) {
        match message {
            SurfaceViewMessage::UpdateSurfaceLongitude(mut longitude) => {
                longitude.normalize();
                self.surface_longitude = longitude;
            }
            SurfaceViewMessage::UpdateSurfaceLatitude(mut latitude) => {
                latitude.normalize();
                self.surface_latitude = latitude;
            }
            SurfaceViewMessage::UpdateViewportOpeningAngle(mut angle) => {
                if angle.as_degrees() < 10. {
                    angle = Angle::from_degrees(10.);
                } else if angle.as_degrees() > 170. {
                    angle = Angle::from_degrees(170.);
                }
                self.viewport_horizontal_opening_angle = angle;
            }
        }
    }

    pub(super) fn redraw(&mut self) {
        self.bodies_cache.clear();
    }

    pub(super) fn control_field(&self) -> iced::Element<'_, GuiMessage> {
        const ANGLE_STEP: Angle = Angle::from_radians(10. * 2. * PI / 360.);
        let longitude = self.surface_longitude;
        let surface_longitude_control_field = control_field(
            "Surface Longitude:",
            format!("{}", longitude),
            SurfaceViewMessage::UpdateSurfaceLongitude(longitude - ANGLE_STEP),
            SurfaceViewMessage::UpdateSurfaceLongitude(longitude + ANGLE_STEP),
        );
        let latitude = self.surface_latitude;
        let surface_latitude_control_field = control_field(
            "Surface Latitude:",
            format!("{}", latitude),
            SurfaceViewMessage::UpdateSurfaceLatitude(latitude - ANGLE_STEP),
            SurfaceViewMessage::UpdateSurfaceLatitude(latitude + ANGLE_STEP),
        );
        let viewport_angle = self.viewport_horizontal_opening_angle;
        let viewport_angle_control_field = control_field(
            "Horizontal Viewport Opening Angle:",
            format!("{}", viewport_angle),
            SurfaceViewMessage::UpdateViewportOpeningAngle(viewport_angle - ANGLE_STEP),
            SurfaceViewMessage::UpdateViewportOpeningAngle(viewport_angle + ANGLE_STEP),
        );
        Column::new()
            .push(surface_longitude_control_field)
            .push(surface_latitude_control_field)
            .push(viewport_angle_control_field)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}
