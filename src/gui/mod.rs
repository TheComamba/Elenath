use self::dialog::Dialog;
use self::gui_widget::GuiMode;
use self::surface_view::surface_view_widget::SurfaceViewState;
use self::table_view::table_view_widget::TableViewState;
use self::top_view::top_view_widget::TopViewState;
use crate::model::celestial_system::CelestialSystem;
use crate::model::planet::Planet;
use astro_utils::planets::planet_data::PlanetData;
use astro_utils::units::time::Time;
use std::path::PathBuf;

mod dialog;
mod gui_widget;
mod shared_canvas_functionality;
mod shared_widgets;
mod surface_view;
mod table_view;
mod top_view;

pub(crate) struct Gui {
    opened_file: Option<PathBuf>,
    mode: GuiMode,
    surface_view_state: SurfaceViewState,
    top_view_state: TopViewState,
    table_view_state: TableViewState,
    time_since_epoch: Time,
    time_step: Time,
    celestial_system: CelestialSystem,
    selected_planet_name: String,
    display_names: bool,
    pub(crate) dialog: Option<Box<dyn Dialog>>,
}

impl Gui {
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

    pub(super) fn get_selected_planet(&self) -> Option<Planet> {
        self.get_selected_planet_data().map(|data| {
            Planet::new(
                (*data).clone(),
                self.celestial_system.get_central_body_data(),
                self.time_since_epoch,
            )
        })
    }

    pub(super) fn get_selected_planet_data(&self) -> Option<&PlanetData> {
        let planet_data = self
            .celestial_system
            .get_planet_data()
            .iter()
            .find(|p| p.get_name().eq(&self.selected_planet_name))
            .map(|p| *p);
        planet_data
    }
}
