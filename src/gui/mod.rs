use std::vec;

use self::topview::TopViewState;
use crate::model::celestial_body::CelestialBody;
use crate::model::{celestial_body::CelestialBodyData, example::sun};
use astro_utils::{units::time::Time, Float};
use iced::{
    widget::{canvas, Column},
    Sandbox,
};

mod shared_widgets;
mod topview;

pub(crate) struct Gui {
    mode: GuiMode,
    time: Time,
    time_step: Time,
    topview_state: TopViewState,
    central_body_data: CelestialBodyData,
    celestial_bodies: Vec<CelestialBody>,
    selected_focus: Option<CelestialBody>,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        let central_body_data = sun();
        let celestial_bodies = central_body_data.system(Time::from_days(0.0));
        Gui {
            mode: GuiMode::TopView,
            time: Time::from_days(0.0),
            time_step: Time::from_days(1.0),
            topview_state: TopViewState::new(),
            central_body_data,
            celestial_bodies,
            selected_focus: None,
        }
    }

    fn title(&self) -> String {
        String::from("Elenath - Imaginary Skies")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
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
            GuiMessage::UpdateLengthScale(m_per_px) => {
                self.topview_state.set_meter_per_pixel(m_per_px);
            }
            GuiMessage::FocusedBodySelected(planet_name) => {
                self.selected_focus = Some(planet_name);
            }
        }
        self.topview_state.redraw(); //If performance is an issue, only redraw when needed
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let mut col = Column::new().push(self.gui_mode_tabs());

        match self.mode {
            GuiMode::LocalView => (),
            GuiMode::TopView => {
                col = col.push(self.topview_control_field()).push(
                    canvas(self)
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill),
                )
            }
            GuiMode::TableView => (),
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
            GuiMode::LocalView => todo![],
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
        self.celestial_bodies = self.central_body_data.system(self.time);
        if let Some(focus) = &self.selected_focus {
            self.selected_focus = self
                .celestial_bodies
                .iter()
                .find(|body| body.get_name() == focus.get_name())
                .cloned();
        }
    }
}

#[derive(Debug, Clone)]
pub(super) enum GuiMessage {
    ModeSelected(GuiMode),
    UpdateTime(Time),
    UpdateTimeStep(Time),
    UpdateLengthScale(Float),
    FocusedBodySelected(CelestialBody),
}

#[derive(Debug, Clone)]
pub(super) enum GuiMode {
    LocalView,
    TopView,
    TableView,
}
