use crate::{
    gui::{
        gui_widget::GuiMessage,
        shared_widgets::{control_field, planet_picker, time_control_fields},
    },
    model::celestial_body::CelestialBody,
};
use astro_utils::{
    coordinates::ecliptic::EclipticCoordinates,
    units::{angle::Angle, length::Length, time::Time},
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
    pub(super) au_per_pixel: Float,
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
        TopViewState {
            background_cache: Cache::default(),
            bodies_cache: Cache::default(),
            scale_cache: Cache::default(),
            au_per_pixel: 0.01,
            view_ecliptic: EclipticCoordinates::Z_DIRECTION,
        }
    }

    pub(super) fn update(&mut self, message: TopViewMessage) {
        match message {
            TopViewMessage::UpdateLengthScale(au_per_pixel) => {
                self.au_per_pixel = au_per_pixel;
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
    }

    pub(super) fn redraw(&mut self) {
        self.bodies_cache.clear();
        self.scale_cache.clear();
    }

    pub(super) fn control_field<'a>(
        &'a self,
        time_since_epoch: &'a Time,
        time_step: &'a Time,
        celestial_bodies: &'a Vec<CelestialBody>,
        selected_body: &'a Option<CelestialBody>,
    ) -> iced::Element<'a, GuiMessage> {
        let time_control_fields = time_control_fields(time_since_epoch, time_step);
        let m_per_px = self.au_per_pixel;
        let length_scale_control_field = control_field(
            "Length per 100px:",
            format!("{}", Length::from_meters(100. * m_per_px)),
            TopViewMessage::UpdateLengthScale(m_per_px / 2.),
            TopViewMessage::UpdateLengthScale(m_per_px * 2.),
        );
        const VIEW_ANGLE_STEP: Angle = Angle::from_radians(10. * 2. * PI / 360.);
        let view_longitude = self.view_ecliptic.get_longitude();
        let view_longitude_control_field = control_field(
            "View longitude:",
            format!("{}", view_longitude),
            TopViewMessage::UpdateViewLongitude(view_longitude - VIEW_ANGLE_STEP),
            TopViewMessage::UpdateViewLongitude(view_longitude + VIEW_ANGLE_STEP),
        );
        let view_latitude = self.view_ecliptic.get_latitude();
        let view_latitude_control_field = control_field(
            "View latitude:",
            format!("{}", view_latitude),
            TopViewMessage::UpdateViewLatitude(view_latitude - VIEW_ANGLE_STEP),
            TopViewMessage::UpdateViewLatitude(view_latitude + VIEW_ANGLE_STEP),
        );
        let planet_picker = planet_picker(celestial_bodies, selected_body);
        Column::new()
            .push(time_control_fields)
            .push(length_scale_control_field)
            .push(view_longitude_control_field)
            .push(view_latitude_control_field)
            .push(planet_picker)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}
