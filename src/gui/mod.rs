use self::gui_widget::GuiMode;
use self::surface_view::SurfaceViewState;
use self::top_view::TopViewState;
use crate::gui::table_col_data::TableColData;
use crate::model::{celestial_body::CelestialBody, celestial_system::CelestialSystem};
use astro_utils::units::time::Time;
use std::path::PathBuf;

mod gui_widget;
mod shared_widgets;
mod surface_view;
mod surface_view_canvas;
mod table_col_data;
mod table_view;
mod top_view;
mod top_view_canvas;

pub(crate) struct Gui {
    opened_file: Option<PathBuf>,
    mode: GuiMode,
    time_since_epoch: Time,
    time_step: Time,
    surface_view_state: SurfaceViewState,
    topview_state: TopViewState,
    celestial_system: CelestialSystem,
    celestial_bodies: Vec<CelestialBody>,
    selected_body: Option<CelestialBody>,
    table_col_data: Vec<TableColData>,
}

impl Gui {
    fn update_bodies(&mut self) {
        self.celestial_bodies = self
            .celestial_system
            .get_current_data(self.time_since_epoch);
        if let Some(focus) = &self.selected_body {
            self.selected_body = self
                .celestial_bodies
                .iter()
                .find(|body| body.get_name() == focus.get_name())
                .cloned();
        }
    }
}
