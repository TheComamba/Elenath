use super::{
    surface_view_widget::{SurfaceViewMessage, SurfaceViewState},
    top_view_widget::{TopViewMessage, TopViewState},
    Gui,
};
use crate::{
    file_dialog,
    model::{
        celestial_body::CelestialBody, celestial_system::CelestialSystem, example::solar_system,
    },
};
use astro_utils::units::time::Time;
use iced::{
    widget::{canvas, Column},
    Sandbox,
};

#[derive(Debug, Clone)]
pub(crate) enum GuiMessage {
    UpdateSurfaceView(SurfaceViewMessage),
    UpdateTopView(TopViewMessage),
    SaveToFile,
    SaveToNewFile,
    OpenFile,
    ModeSelected(GuiMode),
    UpdateTime(Time),
    UpdateTimeStep(Time),
    FocusedBodySelected(CelestialBody),
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
        let central_body_data = celestial_system.get_central_body_data();
        let selected_focus = celestial_bodies
            .iter()
            .find(|body| body.get_name() == central_body_data.get_name())
            .cloned();
        let mut gui = Gui {
            opened_file: None,
            mode: GuiMode::TopView,
            time_since_epoch: Time::from_days(0.0),
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
            GuiMessage::UpdateSurfaceView(message) => {
                self.surface_view_state.update(message);
            }
            GuiMessage::UpdateTopView(message) => {
                self.topview_state.update(message);
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
                self.selected_body = Some(body);
            }
        }
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
