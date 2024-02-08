use crate::gui::{
    gui_widget::{BIG_COLUMN_WIDTH, PADDING},
    message::GuiMessage,
    shared_widgets::control_field,
};
use astro_utils::{
    astro_display::AstroDisplay,
    units::angle::{normalized_angle, ANGLE_ZERO},
};
use iced::{
    widget::{
        canvas::{self},
        Column,
    },
    Alignment,
};
use simple_si_units::geometry::Angle;
use std::f64::consts::PI;

const HUMAN_EYE_OPENING_ANGLE: Angle<f64> = Angle {
    rad: 120. / 360. * 2. * PI,
};

pub(crate) struct SurfaceViewState {
    pub(super) background_cache: canvas::Cache,
    pub(super) bodies_cache: canvas::Cache,
    pub(super) surface_longitude: Angle<f64>,
    pub(super) surface_latitude: Angle<f64>,
    pub(super) view_longitude: Angle<f64>,
    pub(super) view_latitude: Angle<f64>,
    pub(super) viewport_vertical_opening_angle: Angle<f64>,
}

#[derive(Debug, Clone)]
pub(crate) enum SurfaceViewUpdate {
    SurfaceLongitude(Angle<f64>),
    SurfaceLatitude(Angle<f64>),
    ViewLongitude(Angle<f64>),
    ViewLatitude(Angle<f64>),
    ViewportOpeningAngle(Angle<f64>),
}

impl From<SurfaceViewUpdate> for GuiMessage {
    fn from(val: SurfaceViewUpdate) -> Self {
        GuiMessage::UpdateSurfaceView(val)
    }
}

impl SurfaceViewState {
    pub(crate) fn new() -> Self {
        SurfaceViewState {
            background_cache: canvas::Cache::default(),
            bodies_cache: canvas::Cache::default(),
            surface_longitude: ANGLE_ZERO,
            surface_latitude: ANGLE_ZERO,
            view_longitude: ANGLE_ZERO,
            view_latitude: Angle::from_degrees(90.),
            viewport_vertical_opening_angle: HUMAN_EYE_OPENING_ANGLE,
        }
    }

    pub(crate) fn update(&mut self, message: SurfaceViewUpdate) {
        match message {
            SurfaceViewUpdate::SurfaceLongitude(mut longitude) => {
                longitude = normalized_angle(longitude);
                self.surface_longitude = longitude;
            }
            SurfaceViewUpdate::SurfaceLatitude(mut latitude) => {
                if latitude.to_degrees() < -90. {
                    latitude = Angle::from_degrees(-90.);
                } else if latitude.to_degrees() > 90. {
                    latitude = Angle::from_degrees(90.);
                }
                self.surface_latitude = latitude;
            }
            SurfaceViewUpdate::ViewLongitude(mut longitude) => {
                longitude = normalized_angle(longitude);
                self.view_longitude = longitude;
            }
            SurfaceViewUpdate::ViewLatitude(mut latitude) => {
                if latitude.to_degrees() < 10. {
                    latitude = Angle::from_degrees(10.);
                } else if latitude.to_degrees() > 90. {
                    latitude = Angle::from_degrees(90.);
                }
                self.view_latitude = latitude;
            }
            SurfaceViewUpdate::ViewportOpeningAngle(mut angle) => {
                if angle.to_degrees() < 10. {
                    angle = Angle::from_degrees(10.);
                } else if angle.to_degrees() > 170. {
                    angle = Angle::from_degrees(170.);
                }
                self.viewport_vertical_opening_angle = angle;
            }
        }
    }

    pub(crate) fn redraw(&mut self) {
        self.bodies_cache.clear();
    }

    pub(crate) fn control_field(&self) -> iced::Element<'_, GuiMessage> {
        const ANGLE_STEP: Angle<f64> = Angle {
            rad: 10. * 2. * PI / 360.,
        };
        let surface_long = self.surface_longitude;
        let surface_longitude_control_field = control_field(
            "Surface Longitude:",
            surface_long.astro_display(),
            SurfaceViewUpdate::SurfaceLongitude(surface_long - ANGLE_STEP),
            SurfaceViewUpdate::SurfaceLongitude(surface_long + ANGLE_STEP),
        );

        let surface_lat = self.surface_latitude;
        let surface_latitude_control_field = control_field(
            "Surface Latitude:",
            surface_lat.astro_display(),
            SurfaceViewUpdate::SurfaceLatitude(surface_lat - ANGLE_STEP),
            SurfaceViewUpdate::SurfaceLatitude(surface_lat + ANGLE_STEP),
        );

        let view_long = self.view_longitude;
        let view_longitude_control_field = control_field(
            "Observer Longitude:",
            view_long.astro_display(),
            SurfaceViewUpdate::ViewLongitude(view_long - ANGLE_STEP),
            SurfaceViewUpdate::ViewLongitude(view_long + ANGLE_STEP),
        );

        let view_lat = self.view_latitude;
        let view_latitude_control_field = control_field(
            "Observer Latitude:",
            view_lat.astro_display(),
            SurfaceViewUpdate::ViewLatitude(view_lat - ANGLE_STEP),
            SurfaceViewUpdate::ViewLatitude(view_lat + ANGLE_STEP),
        );

        let viewport_angle = self.viewport_vertical_opening_angle;
        let viewport_angle_control_field = control_field(
            "Vertical Viewport Opening Angle:",
            viewport_angle.astro_display(),
            SurfaceViewUpdate::ViewportOpeningAngle(viewport_angle - ANGLE_STEP),
            SurfaceViewUpdate::ViewportOpeningAngle(viewport_angle + ANGLE_STEP),
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
