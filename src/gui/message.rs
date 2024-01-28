use super::gui_widget::GuiMode;
use super::{dialog::error::Error, Gui};
use super::{
    dialog::new_system::NewSystemDialog, surface_view::surface_view_widget::SurfaceViewMessage,
    top_view::top_view_widget::TopViewMessage,
};
use crate::{file_dialog, model::celestial_system::CelestialSystem};
use astro_utils::units::time::Time;

#[derive(Debug, Clone)]
pub(crate) enum GuiMessage {
    UpdateSurfaceView(SurfaceViewMessage),
    UpdateTopView(TopViewMessage),
    NewSystemDialog,
    NewSystemDialogSubmit(CelestialSystem),
    SaveToFile,
    SaveToNewFile,
    OpenFile,
    ModeSelected(GuiMode),
    AddPlanet,
    AddStar,
    UpdateTime(Time),
    UpdateTimeStep(Time),
    PlanetSelected(String),
    SetShowNames(bool),
    DialogClosed,
}

impl Gui {
    pub(crate) fn handle_message(&mut self, message: GuiMessage) -> Result<(), Error> {
        match message {
            GuiMessage::UpdateSurfaceView(message) => {
                self.surface_view_state.update(message);
            }
            GuiMessage::UpdateTopView(message) => {
                self.top_view_state.update(message);
            }
            GuiMessage::NewSystemDialog => {
                self.dialog = Some(Box::new(NewSystemDialog::new()));
            }
            GuiMessage::AddPlanet => {
                todo!("Implement adding planets.");
            }
            GuiMessage::AddStar => {
                todo!("Implement adding stars.");
            }
            GuiMessage::NewSystemDialogSubmit(celestial_system) => {
                self.celestial_system = celestial_system;
                self.dialog = None;
            }
            GuiMessage::SaveToFile => {
                if self.opened_file.is_none() {
                    self.opened_file = file_dialog::new();
                }
                if let Some(path) = &self.opened_file {
                    self.celestial_system
                        .write_to_file(path.clone())
                        .expect("Failed to write to file");
                }
            }
            GuiMessage::SaveToNewFile => {
                self.opened_file = file_dialog::new();
                if let Some(path) = &self.opened_file {
                    self.celestial_system
                        .write_to_file(path.clone())
                        .expect("Failed to write to file");
                }
            }
            GuiMessage::OpenFile => {
                self.opened_file = file_dialog::open();
                if let Some(path) = &self.opened_file {
                    self.celestial_system = CelestialSystem::read_from_file(path.clone())
                        .expect("Failed to read from file");
                }
            }
            GuiMessage::ModeSelected(mode) => {
                self.mode = mode;
            }
            GuiMessage::UpdateTime(time) => {
                self.time_since_epoch = time;
            }
            GuiMessage::UpdateTimeStep(time_step) => {
                self.time_step = time_step;
            }
            GuiMessage::PlanetSelected(name) => {
                self.selected_planet_name = name;
            }
            GuiMessage::SetShowNames(display_names) => {
                self.display_names = display_names;
            }
            GuiMessage::DialogClosed => {
                self.dialog = None;
            }
        }
        self.redraw();
        Ok(())
    }
}
