use self::surface_view::SurfaceViewState;
use self::top_view::TopViewState;
use crate::file_dialog;
use crate::gui::table_col_data::TableColData;
use crate::model::example::solar_system;
use crate::model::{celestial_body::CelestialBody, celestial_system::CelestialSystem};
use astro_utils::units::angle::Angle;
use astro_utils::{units::time::Time, Float};
use iced::{
    widget::{canvas, Column},
    Sandbox,
};
use std::path::PathBuf;
use std::vec;

mod shared_widgets;
mod surface_view;
mod table_col_data;
mod table_view;
mod top_view;

pub(crate) struct Gui {
    opened_file: Option<PathBuf>,
    mode: GuiMode,
    time: Time,
    time_step: Time,
    surface_view_state: SurfaceViewState,
    topview_state: TopViewState,
    celestial_system: CelestialSystem,
    celestial_bodies: Vec<CelestialBody>,
    selected_body: Option<CelestialBody>,
    table_col_data: Vec<TableColData>,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        let celestial_system = solar_system();
        let celestial_bodies = celestial_system.get_current_data(Time::from_days(0.0));
        let central_body_data = celestial_system.get_central_body_data();
        let selected_focus = celestial_bodies
            .iter()
            .find(|body| body.get_name() == central_body_data.get_name())
            .cloned();
        let mut gui = Gui {
            opened_file: None,
            mode: GuiMode::TopView,
            time: Time::from_days(0.0),
            time_step: Time::from_days(1.0),
            surface_view_state: SurfaceViewState::new(),
            topview_state: TopViewState::new(),
            celestial_system,
            celestial_bodies,
            selected_body: selected_focus,
            table_col_data: vec![],
        };
        gui.init_table_col_data();
        gui
    }

    fn title(&self) -> String {
        String::from("Elenath - Imaginary Skies")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
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
                self.time = time;
                self.update_bodies();
            }
            GuiMessage::UpdateTimeStep(time_step) => {
                self.time_step = time_step;
            }
            GuiMessage::UpdateSurfaceLongitude(longitude) => {
                self.surface_view_state.surface_longitude = longitude;
            }
            GuiMessage::UpdateSurfaceLatitude(latitude) => {
                self.surface_view_state.surface_latitude = latitude;
            }
            GuiMessage::UpdateLengthScale(m_per_px) => {
                self.topview_state.set_meter_per_pixel(m_per_px);
            }
            GuiMessage::UpdateViewLongitude(longitude) => {
                self.topview_state.view_ecliptic.set_longitude(longitude);
                self.topview_state.view_ecliptic.normalize();
            }
            GuiMessage::UpdateViewLatitude(latitude) => {
                self.topview_state.view_ecliptic.set_latitude(latitude);
                self.topview_state.view_ecliptic.normalize();
            }
            GuiMessage::FocusedBodySelected(body) => {
                self.selected_body = Some(body);
            }
        }
        self.surface_view_state.redraw(); //If performance is an issue, only redraw when needed
        self.topview_state.redraw(); //If performance is an issue, only redraw when needed
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let mut col = Column::new()
            .push(self.gui_mode_tabs())
            .push(self.file_buttons());

        match self.mode {
            GuiMode::SurfaceView => {
                col = col.push(self.surface_view_control_field()).push(
                    canvas(self)
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill),
                )
            }
            GuiMode::TopView => {
                col = col.push(self.topview_control_field()).push(
                    canvas(self)
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill),
                )
            }
            GuiMode::TableView => col = col.push(self.table_view()),
        }

        col.width(iced::Length::Fill)
            .height(iced::Length::Fill)
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
            GuiMode::SurfaceView => self.surface_view_canvas(renderer, bounds),
            GuiMode::TopView => self.topview_canvas(renderer, bounds),
            _ => {
                println!("Invalid Gui state: Canvas Program is called from a Gui mode that does not have a canvas.");
                vec![]
            }
        }
    }
}

impl Gui {
    fn update_bodies(&mut self) {
        self.celestial_bodies = self.celestial_system.get_current_data(self.time);
        if let Some(focus) = &self.selected_body {
            self.selected_body = self
                .celestial_bodies
                .iter()
                .find(|body| body.get_name() == focus.get_name())
                .cloned();
        }
    }
}

#[derive(Debug, Clone)]
pub(super) enum GuiMessage {
    SaveToFile,
    SaveToNewFile,
    OpenFile,
    ModeSelected(GuiMode),
    UpdateTime(Time),
    UpdateTimeStep(Time),
    UpdateSurfaceLongitude(Angle),
    UpdateSurfaceLatitude(Angle),
    UpdateLengthScale(Float),
    UpdateViewLongitude(Angle),
    UpdateViewLatitude(Angle),
    FocusedBodySelected(CelestialBody),
}

#[derive(Debug, Clone)]
pub(super) enum GuiMode {
    SurfaceView,
    TopView,
    TableView,
}
