use crate::gui::{
    gui_widget::{GuiMessage, BIG_COLUMN_WIDTH, PADDING},
    shared_widgets::control_field,
};
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

pub(crate) struct SurfaceViewState {
    pub(super) background_cache: canvas::Cache,
    pub(super) bodies_cache: canvas::Cache,
    pub(super) surface_longitude: Angle,
    pub(super) surface_latitude: Angle,
    pub(super) view_longitude: Angle,
    pub(super) view_latitude: Angle,
    pub(super) viewport_horizontal_opening_angle: Angle,
}

#[derive(Debug, Clone)]
pub(crate) enum SurfaceViewMessage {
    UpdateSurfaceLongitude(Angle),
    UpdateSurfaceLatitude(Angle),
    UpdateViewLongitude(Angle),
    UpdateViewLatitude(Angle),
    UpdateViewportOpeningAngle(Angle),
}

impl Into<GuiMessage> for SurfaceViewMessage {
    fn into(self) -> GuiMessage {
        GuiMessage::UpdateSurfaceView(self)
    }
}

impl SurfaceViewState {
    pub(crate) fn new() -> Self {
        SurfaceViewState {
            background_cache: canvas::Cache::default(),
            bodies_cache: canvas::Cache::default(),
            surface_longitude: Angle::ZERO,
            surface_latitude: Angle::ZERO,
            view_longitude: Angle::ZERO,
            view_latitude: Angle::from_degrees(90.),
            viewport_horizontal_opening_angle: HUMAN_EYE_OPENING_ANGLE,
        }
    }

    pub(crate) fn update(&mut self, message: SurfaceViewMessage) {
        match message {
            SurfaceViewMessage::UpdateSurfaceLongitude(mut longitude) => {
                longitude.normalize();
                self.surface_longitude = longitude;
            }
            SurfaceViewMessage::UpdateSurfaceLatitude(mut latitude) => {
                if latitude.as_degrees() < -90. {
                    latitude = Angle::from_degrees(-90.);
                } else if latitude.as_degrees() > 90. {
                    latitude = Angle::from_degrees(90.);
                }
                self.surface_latitude = latitude;
            }
            SurfaceViewMessage::UpdateViewLongitude(mut longitude) => {
                longitude.normalize();
                self.view_longitude = longitude;
            }
            SurfaceViewMessage::UpdateViewLatitude(mut latitude) => {
                if latitude.as_degrees() < 10. {
                    latitude = Angle::from_degrees(10.);
                } else if latitude.as_degrees() > 90. {
                    latitude = Angle::from_degrees(90.);
                }
                self.view_latitude = latitude;
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

    pub(crate) fn redraw(&mut self) {
        self.bodies_cache.clear();
    }

    pub(crate) fn control_field(&self) -> iced::Element<'_, GuiMessage> {
        const ANGLE_STEP: Angle = Angle::from_radians(10. * 2. * PI / 360.);
        let surface_long = self.surface_longitude;
        let surface_longitude_control_field = control_field(
            "Surface Longitude:",
            format!("{}", surface_long),
            SurfaceViewMessage::UpdateSurfaceLongitude(surface_long - ANGLE_STEP),
            SurfaceViewMessage::UpdateSurfaceLongitude(surface_long + ANGLE_STEP),
        );

        let surface_lat = self.surface_latitude;
        let surface_latitude_control_field = control_field(
            "Surface Latitude:",
            format!("{}", surface_lat),
            SurfaceViewMessage::UpdateSurfaceLatitude(surface_lat - ANGLE_STEP),
            SurfaceViewMessage::UpdateSurfaceLatitude(surface_lat + ANGLE_STEP),
        );

        let view_long = self.view_longitude;
        let view_longitude_control_field = control_field(
            "Observer Longitude:",
            format!("{}", view_long),
            SurfaceViewMessage::UpdateViewLongitude(view_long - ANGLE_STEP),
            SurfaceViewMessage::UpdateViewLongitude(view_long + ANGLE_STEP),
        );

        let view_lat = self.view_latitude;
        let view_latitude_control_field = control_field(
            "Observer Latitude:",
            format!("{}", view_lat),
            SurfaceViewMessage::UpdateViewLatitude(view_lat - ANGLE_STEP),
            SurfaceViewMessage::UpdateViewLatitude(view_lat + ANGLE_STEP),
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
            .push(view_longitude_control_field)
            .push(view_latitude_control_field)
            .push(viewport_angle_control_field)
            .width(iced::Length::Fixed(BIG_COLUMN_WIDTH))
            .align_items(Alignment::Center)
            .spacing(PADDING)
            .into()
    }
}
