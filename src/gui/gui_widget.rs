use super::{
    shared_widgets::surface_and_top_view_shared_control,
    surface_view_widget::{SurfaceViewMessage, SurfaceViewState},
    table_view::TableViewState,
    top_view_widget::{TopViewMessage, TopViewState},
    Gui,
};
use crate::{
    file_dialog,
    model::{
        celestial_body::CelestialBody, celestial_system::CelestialSystem, example::solar_system,
    },
};
use astro_utils::{
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
    FocusedBodySelected(CelestialBody),
    SetShowNames(bool),
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
        let celestial_bodies = celestial_system.get_current_data(Time::from_days(0.0));
        let central_body_data = celestial_system.get_central_body();
        let selected_focus = celestial_bodies
            .iter()
            .find(|body| body.get_name() == central_body_data.get_name())
            .cloned();
        Gui {
            opened_file: None,
            mode: GuiMode::SurfaceView,
            surface_view_state: SurfaceViewState::new(),
            top_view_state: TopViewState::new(),
            table_view_state: TableViewState::new(),
            time_since_epoch: Time::from_days(0.0),
            time_step: Time::from_days(1.0),
            celestial_system,
            celestial_bodies,
            focused_body: selected_focus,
            display_names: true,
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
            GuiMessage::AddPlanet => {
                todo!("Implement adding planets.");
                // self.update_bodies();
            }
            GuiMessage::AddStar => {
                todo!("Implement adding stars.");
                // self.update_bodies();
            }
            GuiMessage::GenerateStars => {
                let max_distance = Length::from_light_years(100.0);
                let stars = generate_random_stars(max_distance).unwrap();
                self.celestial_system.add_distant_stars(stars);
                self.update_bodies();
            }
            GuiMessage::FetchGaiaData => {
                let stars = fetch_brightest_stars().unwrap();
                self.celestial_system
                    .add_distant_stars_without_duplicates(stars);
                self.update_bodies();
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
                    self.update_bodies();
                }
            }
            GuiMessage::ModeSelected(mode) => {
                self.mode = mode;
            }
            GuiMessage::UpdateTime(time) => {
                self.time_since_epoch = time;
                self.update_bodies();
            }
            GuiMessage::UpdateTimeStep(time_step) => {
                self.time_step = time_step;
            }
            GuiMessage::FocusedBodySelected(body) => {
                self.focused_body = Some(body);
            }
            GuiMessage::SetShowNames(display_names) => {
                self.display_names = display_names;
            }
        }
        self.redraw();
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let toprow = Row::new()
            .push(Gui::gui_mode_tabs())
            .push(Gui::adding_buttons())
            .push(Gui::file_buttons())
            .padding(PADDING);
        let mut col = Column::new().push(toprow);

        match self.mode {
            GuiMode::SurfaceView => {
                let control_row = Row::new()
                    .push(surface_and_top_view_shared_control(
                        &self.time_since_epoch,
                        &self.time_step,
                        &self.celestial_bodies,
                        &self.focused_body,
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
                        &self.celestial_bodies,
                        &self.focused_body,
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
                    self.celestial_system.get_planets_data(),
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
                self.celestial_system.get_central_body(),
                &self.focused_body,
                self.time_since_epoch,
                &self.celestial_bodies,
                self.display_names,
            ),
            GuiMode::TopView => self.top_view_state.canvas(
                renderer,
                bounds,
                &self.focused_body,
                &self.celestial_bodies,
                self.display_names,
            ),
            _ => {
                println!("Invalid Gui state: Canvas Program is called from a Gui mode that does not have a canvas.");
                vec![]
            }
        }
    }
}
