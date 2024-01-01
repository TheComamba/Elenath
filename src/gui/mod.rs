use self::gui_widget::GuiMode;
use self::surface_view_widget::SurfaceViewState;
use self::top_view_widget::TopViewState;
use crate::model::{celestial_body::CelestialBody, celestial_system::CelestialSystem};
use astro_utils::units::time::Time;
use std::path::PathBuf;

mod gui_widget;
mod shared_canvas_functionality;
mod shared_widgets;
mod surface_view_canvas;
mod surface_view_widget;
mod table_col_data;
mod table_view;
mod top_view_canvas;
mod top_view_widget;

pub(crate) struct Gui {
    opened_file: Option<PathBuf>,
    mode: GuiMode,
    surface_view_state: SurfaceViewState,
    top_view_state: TopViewState,
    table_view_state: table_view::TableViewState,
    time_since_epoch: Time,
    time_step: Time,
    celestial_system: CelestialSystem,
    celestial_bodies: Vec<CelestialBody>,
    selected_body: Option<CelestialBody>,
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

    pub(super) fn redraw(&mut self) {
        match self.mode {
            GuiMode::SurfaceView => {
                self.surface_view_state.redraw();
            }
            GuiMode::TopView => {
                self.top_view_state.redraw();
            }
            _ => (),
        };
    }
}
