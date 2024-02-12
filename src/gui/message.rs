use super::dialog::planet::PlanetDialog;
use super::dialog::star::StarDialog;
use super::gui_widget::GuiViewMode;
use super::Gui;
use super::{
    dialog::new_system::NewSystemDialog, surface_view::surface_view_widget::SurfaceViewUpdate,
    top_view::top_view_widget::TopViewUpdate,
};
use crate::error::ElenathError;
use crate::{file_dialog, model::celestial_system::CelestialSystem};
use astro_utils::planets::derived_data::DerivedPlanetData;
use astro_utils::planets::planet_data::PlanetData;
use astro_utils::stars::star_data::StarData;
use simple_si_units::base::Time;

#[derive(Debug, Clone)]
pub(crate) enum GuiMessage {
    UpdateSurfaceView(SurfaceViewUpdate),
    UpdateTopView(TopViewUpdate),
    NewSystemDialog,
    NewSystemDialogSubmit(Result<CelestialSystem, ElenathError>),
    SaveToFile,
    SaveToNewFile,
    OpenFile,
    ModeSelected(GuiViewMode),
    NewPlanetDialog,
    EditPlanetDialog(usize),
    NewPlanet(PlanetData),
    PlanetEdited(usize, PlanetData),
    NewStarDialog,
    EditStarDialog(Option<usize>),
    NewStar(StarData),
    StarEdited(Option<usize>, StarData),
    UpdateTime(Time<f64>),
    UpdateTimeStep(Time<f64>),
    PlanetSelected(String),
    SetDisplayNames(bool),
    SetDisplayConstellations(bool),
    DialogClosed,
    ErrorEncountered(ElenathError),
}

impl Gui {
    pub(crate) fn handle_message(&mut self, message: GuiMessage) -> Result<(), ElenathError> {
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
            GuiMessage::NewPlanetDialog => {
                let celestial_system = &self
                    .celestial_system
                    .as_ref()
                    .ok_or(ElenathError::NoCelestialSystem)?;
                let central_body = celestial_system.get_central_body_data().clone();
                self.dialog = Some(Box::new(PlanetDialog::new(central_body)));
            }
            GuiMessage::EditPlanetDialog(index) => {
                let celestial_system = &self
                    .celestial_system
                    .as_ref()
                    .ok_or(ElenathError::NoCelestialSystem)?;
                let central_body = celestial_system.get_central_body_data();
                let planet = celestial_system
                    .get_planet_data(index)
                    .ok_or(ElenathError::BodyNotFound)?;
                let previous_planet = celestial_system
                    .get_planet_data(index - 1)
                    .map(|p| DerivedPlanetData::new(p, central_body, None));
                self.dialog = Some(Box::new(PlanetDialog::edit(
                    planet.clone(),
                    index,
                    previous_planet,
                    central_body.clone(),
                )));
            }
            GuiMessage::NewPlanet(planet) => {
                self.celestial_system
                    .as_mut()
                    .ok_or(ElenathError::NoCelestialSystem)?
                    .add_planet_data(planet);
                self.dialog = None;
            }
            GuiMessage::PlanetEdited(index, planet_data) => {
                self.celestial_system
                    .as_mut()
                    .ok_or(ElenathError::NoCelestialSystem)?
                    .overwrite_planet_data(index, planet_data);
                self.dialog = None;
            }
            GuiMessage::NewStarDialog => {
                self.dialog = Some(Box::new(StarDialog::new()));
            }
            GuiMessage::EditStarDialog(index) => {
                let star = self
                    .celestial_system
                    .as_ref()
                    .ok_or(ElenathError::NoCelestialSystem)?
                    .get_star_data(index)
                    .ok_or(ElenathError::BodyNotFound)?;
                self.dialog = Some(Box::new(StarDialog::edit(star.clone(), index)));
            }
            GuiMessage::NewStar(star) => {
                self.celestial_system
                    .as_mut()
                    .ok_or(ElenathError::NoCelestialSystem)?
                    .add_star_from_data(star);
                self.dialog = None;
            }
            GuiMessage::StarEdited(index, star_data) => {
                self.celestial_system
                    .as_mut()
                    .ok_or(ElenathError::NoCelestialSystem)?
                    .overwrite_star_data(index, star_data);
                self.dialog = None;
            }
            GuiMessage::NewSystemDialogSubmit(celestial_system) => {
                self.celestial_system = Some(celestial_system?);
                self.dialog = None;
            }
            GuiMessage::SaveToFile => {
                if self.opened_file.is_none() {
                    self.opened_file = file_dialog::new();
                }
                if let Some(path) = &self.opened_file {
                    self.celestial_system
                        .as_ref()
                        .ok_or(ElenathError::NoCelestialSystem)?
                        .write_to_file(path.clone())?;
                }
            }
            GuiMessage::SaveToNewFile => {
                self.opened_file = file_dialog::new();
                if let Some(path) = &self.opened_file {
                    self.celestial_system
                        .as_ref()
                        .ok_or(ElenathError::NoCelestialSystem)?
                        .write_to_file(path.clone())?;
                }
            }
            GuiMessage::OpenFile => {
                self.opened_file = file_dialog::open();
                if let Some(path) = &self.opened_file {
                    self.celestial_system = Some(CelestialSystem::read_from_file(path.clone())?);
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
            GuiMessage::SetDisplayNames(display_names) => {
                self.display_names = display_names;
            }
            GuiMessage::SetDisplayConstellations(display_constellations) => {
                self.display_constellations = display_constellations;
            }
            GuiMessage::DialogClosed => {
                self.dialog = None;
            }
            GuiMessage::ErrorEncountered(error) => {
                return Err(error);
            }
        }
        self.redraw();
        Ok(())
    }
}
