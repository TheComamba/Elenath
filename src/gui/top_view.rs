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
}

impl TopViewState {
    pub(super) fn set_meter_per_pixel(&mut self, meter_per_pixel: Float) {
        self.meter_per_pixel = meter_per_pixel;
    }

    pub(super) fn get_meter_per_pixel(&self) -> Float {
        self.meter_per_pixel
    }

    pub(super) fn redraw(&mut self) {
        self.bodies_cache.clear();
        self.scale_cache.clear();
    }
}

impl Gui {
    pub(super) fn topview_control_field(&self) -> iced::Element<'_, GuiMessage> {
        let m_per_px = self.topview_state.get_meter_per_pixel();
        let length_scale_control_field = self.control_field(
            "Length per 100px:",
            format!("{}", Length::from_meters(100. * m_per_px)),
            GuiMessage::UpdateLengthScale(m_per_px / 2.),
            GuiMessage::UpdateLengthScale(m_per_px * 2.),
        );
        const VIEW_ANGLE_STEP: Angle = Angle::from_radians(10. * 2. * PI / 360.);
        let view_longitude = self.topview_state.view_ecliptic.get_longitude();
        let view_longitude_control_field = self.control_field(
            "View longitude:",
            format!("{}", view_longitude),
            GuiMessage::UpdateViewLongitude(view_longitude - VIEW_ANGLE_STEP),
            GuiMessage::UpdateViewLongitude(view_longitude + VIEW_ANGLE_STEP),
        );
        let view_latitude = self.topview_state.view_ecliptic.get_latitude();
        let view_latitude_control_field = self.control_field(
            "View latitude:",
            format!("{}", view_latitude),
            GuiMessage::UpdateViewLatitude(view_latitude - VIEW_ANGLE_STEP),
            GuiMessage::UpdateViewLatitude(view_latitude + VIEW_ANGLE_STEP),
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
