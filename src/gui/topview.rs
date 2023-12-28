use astro_utils::{units::length::Length, Float};
use iced::widget::canvas::Cache;

pub(super) struct TopViewState {
    pub(super) background_cache: Cache,
    pub(super) bodies_cache: Cache,
    pub(super) scale_cache: Cache,
    pub(super) meter_per_pixel: Float,
}

impl TopViewState {
    pub(super) fn new() -> Self {
        let m_per_au = Length::from_astronomical_units(1.).as_meters();
        TopViewState {
            background_cache: Cache::default(),
            bodies_cache: Cache::default(),
            scale_cache: Cache::default(),
            meter_per_pixel: 0.01 * m_per_au,
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
