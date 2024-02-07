use crate::gui::{
    gui_widget::{BIG_COLUMN_WIDTH, PADDING},
    message::GuiMessage,
    shared_widgets::control_field,
};
use astro_utils::{
    astro_display::AstroDisplay, coordinates::ecliptic::EclipticCoordinates,
    units::angle::normalized_angle,
};
use iced::{
    widget::{canvas::Cache, Column},
    Alignment,
};
use simple_si_units::{base::Distance, geometry::Angle};
use std::f64::consts::PI;

pub(crate) struct TopViewState {
    pub(super) background_cache: Cache,
    pub(super) bodies_cache: Cache,
    pub(super) scale_cache: Cache,
    pub(super) length_per_pixel: Distance<f64>,
    pub(super) view_ecliptic: EclipticCoordinates,
}

#[derive(Debug, Clone)]
pub(crate) enum TopViewMessage {
    UpdateLengthScale(Distance<f64>),
    UpdateViewLongitude(Angle<f64>),
    UpdateViewLatitude(Angle<f64>),
}

impl Into<GuiMessage> for TopViewMessage {
    fn into(self) -> GuiMessage {
        GuiMessage::UpdateTopView(self)
    }
}

impl TopViewState {
    pub(crate) fn new() -> Self {
        TopViewState {
            background_cache: Cache::default(),
            bodies_cache: Cache::default(),
            scale_cache: Cache::default(),
            length_per_pixel: Distance::from_au(0.01),
            view_ecliptic: EclipticCoordinates::Z_DIRECTION,
        }
    }

    pub(crate) fn update(&mut self, message: TopViewMessage) {
        match message {
            TopViewMessage::UpdateLengthScale(length_per_pixel) => {
                self.length_per_pixel = length_per_pixel;
            }
            TopViewMessage::UpdateViewLongitude(mut longitude) => {
                longitude = normalized_angle(longitude);
                self.view_ecliptic.set_longitude(longitude);
            }
            TopViewMessage::UpdateViewLatitude(mut latitude) => {
                if latitude.to_degrees() < -90. {
                    latitude = Angle::from_degrees(-90.);
                } else if latitude.to_degrees() > 90. {
                    latitude = Angle::from_degrees(90.);
                }
                self.view_ecliptic.set_latitude(latitude);
            }
        }
    }

    pub(crate) fn redraw(&mut self) {
        self.bodies_cache.clear();
        self.scale_cache.clear();
    }

    pub(crate) fn control_field(&self) -> iced::Element<'_, GuiMessage> {
        let length_scale_control_field = control_field(
            "Length per 100px:",
            format!("{}", self.length_per_pixel * 100.),
            TopViewMessage::UpdateLengthScale(self.length_per_pixel / 2.),
            TopViewMessage::UpdateLengthScale(self.length_per_pixel * 2.),
        );
        const VIEW_ANGLE_STEP: Angle<f64> = Angle {
            rad: 10. * 2. * PI / 360.,
        };
        let view_longitude = self.view_ecliptic.get_longitude();
        let view_longitude_control_field = control_field(
            "View longitude:",
            view_longitude.astro_display(),
            TopViewMessage::UpdateViewLongitude(view_longitude - VIEW_ANGLE_STEP),
            TopViewMessage::UpdateViewLongitude(view_longitude + VIEW_ANGLE_STEP),
        );
        let view_latitude = self.view_ecliptic.get_latitude();
        let view_latitude_control_field = control_field(
            "View latitude:",
            view_latitude.astro_display(),
            TopViewMessage::UpdateViewLatitude(view_latitude - VIEW_ANGLE_STEP),
            TopViewMessage::UpdateViewLatitude(view_latitude + VIEW_ANGLE_STEP),
        );
        Column::new()
            .push(length_scale_control_field)
            .push(view_longitude_control_field)
            .push(view_latitude_control_field)
            .width(iced::Length::Fixed(BIG_COLUMN_WIDTH))
            .align_items(Alignment::Center)
            .spacing(PADDING)
            .into()
    }
}
