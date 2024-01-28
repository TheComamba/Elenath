use super::{
    dialog::new_system::NewSystemDialog,
    shared_widgets::surface_and_top_view_shared_control,
    surface_view::surface_view_widget::{SurfaceViewMessage, SurfaceViewState},
    table_view::table_view_widget::TableViewState,
    top_view::top_view_widget::{TopViewMessage, TopViewState},
    Gui,
};
use crate::{
    file_dialog,
    model::{
        celestial_system::{CelestialSystem, SystemType},
        example::solar_system,
    },
};
use astro_utils::{
    data::stars::SUN_DATA,
    stars::{gaia_data::fetch_brightest_stars, random_stars::generate_random_stars},
    units::{length::Length, time::Time},
};
use iced::{
    widget::{canvas, Column, Row},
    Sandbox,
};

pub(super) const PADDING: f32 = 10.0;
pub(super) const SMALL_COLUMN_WIDTH: f32 = 150.0;
pub(super) const BIG_COLUMN_WIDTH: f32 = 3.5 * SMALL_COLUMN_WIDTH;

#[derive(Debug, Clone)]
pub(crate) enum GuiMessage {
    UpdateSurfaceView(SurfaceViewMessage),
    UpdateTopView(TopViewMessage),
    NewSystem,
    SaveToFile,
    SaveToNewFile,
    OpenFile,
    ModeSelected(GuiMode),
    AddPlanet,
    AddStar,
    GenerateStars,
    FetchGaiaData,
    UpdateTime(Time),
    UpdateTimeStep(Time),
    PlanetSelected(String),
    SetShowNames(bool),
    DialogClosed,
}

#[derive(Debug, Clone)]
pub(crate) enum GuiMode {
    SurfaceView,
    TopView,
    TableView,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        let celestial_system = solar_system();
        Gui {
            opened_file: None,
            mode: GuiMode::SurfaceView,
            surface_view_state: SurfaceViewState::new(),
            top_view_state: TopViewState::new(),
            table_view_state: TableViewState::new(),
            time_since_epoch: Time::from_days(0.0),
            time_step: Time::from_days(1.0),
            celestial_system,
            selected_planet_name: String::new(),
            display_names: true,
            dialog: None,
        }
    }

    fn title(&self) -> String {
        String::from("Elenath - Imaginary Skies")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            GuiMessage::UpdateSurfaceView(message) => {
                self.surface_view_state.update(message);
            }
            GuiMessage::UpdateTopView(message) => {
                self.top_view_state.update(message);
            }
            GuiMessage::NewSystem => {
                self.dialog = Some(Box::new(NewSystemDialog::default()));
                self.celestial_system =
                    CelestialSystem::new(SystemType::Generated, SUN_DATA.to_star_data());
            }
            GuiMessage::AddPlanet => {
                todo!("Implement adding planets.");
            }
            GuiMessage::AddStar => {
                todo!("Implement adding stars.");
            }
            GuiMessage::GenerateStars => {
                let max_distance = Length::from_light_years(100.0);
                let stars = generate_random_stars(max_distance).unwrap();
                for star_data in stars {
                    self.celestial_system.add_star_from_data(star_data);
                }
            }
            GuiMessage::FetchGaiaData => {
                let stars = fetch_brightest_stars().unwrap();
                self.celestial_system
                    .add_star_appearances_without_duplicates(stars);
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
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let mut toprow = Row::new().push(Gui::gui_mode_tabs());
        if self.celestial_system.is_generated() {
            toprow = toprow
                .push(Gui::adding_buttons())
                .push(Gui::generated_system_file_buttons());
        } else {
            toprow = toprow.push(Gui::real_system_file_buttons());
        }
        toprow = toprow.padding(PADDING).spacing(PADDING);
        let mut col = Column::new().push(toprow);

        match self.mode {
            GuiMode::SurfaceView => {
                let control_row = Row::new()
                    .push(surface_and_top_view_shared_control(
                        &self.time_since_epoch,
                        &self.time_step,
                        self.celestial_system.get_planet_data(),
                        self.get_selected_planet_data(),
                        self.display_names,
                    ))
                    .push(self.surface_view_state.control_field());
                col = col.push(control_row).push(
                    canvas(self)
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill),
                )
            }
            GuiMode::TopView => {
                let control_row = Row::new()
                    .push(surface_and_top_view_shared_control(
                        &self.time_since_epoch,
                        &self.time_step,
                        self.celestial_system.get_planet_data(),
                        self.get_selected_planet_data(),
                        self.display_names,
                    ))
                    .push(self.top_view_state.control_field());
                col = col.push(control_row).push(
                    canvas(self)
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill),
                )
            }
            GuiMode::TableView => {
                col = col.push(self.table_view_state.table_view(
                    self.celestial_system.get_planet_data(),
                    self.celestial_system.get_star_data(),
                ))
            }
        }

        col.width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .spacing(PADDING)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

impl<GuiMessage> canvas::Program<GuiMessage> for Gui {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::theme::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        match self.mode {
            GuiMode::SurfaceView => self.surface_view_state.canvas(
                renderer,
                bounds,
                &self.get_selected_planet(),
                &self.celestial_system,
                self.time_since_epoch,
                self.display_names,
            ),
            GuiMode::TopView => self.top_view_state.canvas(
                renderer,
                bounds,
                &self.get_selected_planet(),
                &self.celestial_system,
                self.time_since_epoch,
                self.display_names,
            ),
            _ => {
                println!("Invalid Gui state: Canvas Program is called from a Gui mode that does not have a canvas.");
                vec![]
            }
        }
    }
}
