use super::Gui;
use crate::gui::gui_widget::GuiMessage;
use astro_utils::{
    coordinates::ecliptic::EclipticCoordinates,
    units::{angle::Angle, length::Length},
    Float,
};
use iced::{
    widget::{canvas::Cache, Column},
    Alignment,
};
use std::f32::consts::PI;

pub(super) struct TopViewState {
    pub(super) background_cache: Cache,
    pub(super) bodies_cache: Cache,
    pub(super) scale_cache: Cache,
    pub(super) meter_per_pixel: Float,
    pub(super) view_ecliptic: EclipticCoordinates,
}

#[derive(Debug, Clone)]
pub(crate) enum TopViewMessage {
    UpdateLengthScale(Float),
    UpdateViewLongitude(Angle),
    UpdateViewLatitude(Angle),
}

impl Into<GuiMessage> for TopViewMessage {
    fn into(self) -> GuiMessage {
        GuiMessage::UpdateTopView(self)
    }
}

impl TopViewState {
    pub(super) fn new() -> Self {
        let m_per_au = Length::from_astronomical_units(1.).as_meters();
        TopViewState {
            background_cache: Cache::default(),
            bodies_cache: Cache::default(),
            scale_cache: Cache::default(),
            meter_per_pixel: 0.01 * m_per_au,
            view_ecliptic: EclipticCoordinates::Z_DIRECTION,
        }
    }

    pub(super) fn update(&mut self, message: TopViewMessage) {
        match message {
            TopViewMessage::UpdateLengthScale(meter_per_pixel) => {
                self.meter_per_pixel = meter_per_pixel;
            }
            TopViewMessage::UpdateViewLongitude(mut longitude) => {
                longitude.normalize();
                self.view_ecliptic.set_longitude(longitude);
            }
            TopViewMessage::UpdateViewLatitude(mut latitude) => {
                latitude.normalize();
                self.view_ecliptic.set_latitude(latitude);
            }
        }
        self.redraw();
    }

    fn redraw(&mut self) {
        self.bodies_cache.clear();
        self.scale_cache.clear();
    }
}

impl Gui {
    pub(super) fn topview_control_field(&self) -> iced::Element<'_, GuiMessage> {
        let m_per_px = self.topview_state.meter_per_pixel;
        let length_scale_control_field = Gui::control_field(
            "Length per 100px:",
            format!("{}", Length::from_meters(100. * m_per_px)),
            TopViewMessage::UpdateLengthScale(m_per_px / 2.),
            TopViewMessage::UpdateLengthScale(m_per_px * 2.),
        );
        const VIEW_ANGLE_STEP: Angle = Angle::from_radians(10. * 2. * PI / 360.);
        let view_longitude = self.topview_state.view_ecliptic.get_longitude();
        let view_longitude_control_field = Gui::control_field(
            "View longitude:",
            format!("{}", view_longitude),
            TopViewMessage::UpdateViewLongitude(view_longitude - VIEW_ANGLE_STEP),
            TopViewMessage::UpdateViewLongitude(view_longitude + VIEW_ANGLE_STEP),
        );
        let view_latitude = self.topview_state.view_ecliptic.get_latitude();
        let view_latitude_control_field = Gui::control_field(
            "View latitude:",
            format!("{}", view_latitude),
            TopViewMessage::UpdateViewLatitude(view_latitude - VIEW_ANGLE_STEP),
            TopViewMessage::UpdateViewLatitude(view_latitude + VIEW_ANGLE_STEP),
        );
        let planet_picker = self.planet_picker();
        Column::new()
            .push(self.time_control_fields())
            .push(length_scale_control_field)
            .push(view_longitude_control_field)
            .push(view_latitude_control_field)
            .push(planet_picker)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}
