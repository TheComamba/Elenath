use std::vec;

use self::topview::TopViewState;
use crate::model::celestial_body::CelestialBody;
use crate::model::{celestial_body::CelestialBodyData, example::sun};
use astro_utils::{
    units::{length::Length, time::Time},
    Float,
};
use iced::{
    alignment::Horizontal,
    widget::{canvas, Button, Column, Container, PickList, Row, Text},
    Alignment, Sandbox,
};

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
        Column::new()
            .push(self.gui_mode_tabs())
            .push(self.topview_control_field())
            .push(
                canvas(self)
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill),
            )
            .width(iced::Length::Fill)
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
    fn gui_mode_tabs(&self) -> iced::Element<'_, GuiMessage> {
        let local_view_button = Button::new(Text::new("Local View"))
            .on_press(GuiMessage::ModeSelected(GuiMode::LocalView));
        let top_view_button =
            Button::new(Text::new("Top View")).on_press(GuiMessage::ModeSelected(GuiMode::TopView));
        let table_view_button = Button::new(Text::new("Table View"))
            .on_press(GuiMessage::ModeSelected(GuiMode::TableView));
        Row::new()
            .push(local_view_button)
            .push(top_view_button)
            .push(table_view_button)
            .align_items(Alignment::Center)
            .into()
    }

    fn topview_control_field(&self) -> iced::Element<'_, GuiMessage> {
        let time_control_field = self.control_field(
            "Time:",
            format!("{}", self.time),
            GuiMessage::UpdateTime(self.time - self.time_step),
            GuiMessage::UpdateTime(self.time + self.time_step),
        );
        let time_step_control_field = self.control_field(
            "Time step:",
            format!("{}", self.time_step),
            GuiMessage::UpdateTimeStep(self.time_step / 2.),
            GuiMessage::UpdateTimeStep(self.time_step * 2.),
        );
        let m_per_px = self.topview_state.get_meter_per_pixel();
        let length_scale_control_field = self.control_field(
            "Length per 100px:",
            format!("{}", Length::from_meters(100. * m_per_px)),
            GuiMessage::UpdateLengthScale(m_per_px / 2.),
            GuiMessage::UpdateLengthScale(m_per_px * 2.),
        );
        let planet_picker = self.planet_picker();
        Column::new()
            .push(time_control_field)
            .push(time_step_control_field)
            .push(length_scale_control_field)
            .push(planet_picker)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }

    fn control_field<'a>(
        &self,
        label: &'a str,
        value: String,
        decrease: GuiMessage,
        increase: GuiMessage,
    ) -> Row<'a, GuiMessage> {
        let label = Container::new(Text::new(label))
            .align_x(Horizontal::Center)
            .width(iced::Length::Fixed(150.));
        let decrease_button = Container::new(Button::new(Text::new("<<")).on_press(decrease))
            .align_x(Horizontal::Center)
            .width(iced::Length::Fixed(50.));
        let value = Container::new(Text::new(value))
            .width(iced::Length::Fixed(100.))
            .align_x(Horizontal::Center);
        let increase_button = Container::new(Button::new(Text::new(">>")).on_press(increase))
            .align_x(Horizontal::Center)
            .width(iced::Length::Fixed(50.));
        Row::new()
            .push(label)
            .push(decrease_button)
            .push(value)
            .push(increase_button)
            .align_items(Alignment::Center)
    }

    fn planet_picker(&self) -> iced::Element<'_, GuiMessage> {
        let text = Text::new("Focused body:").width(150.);
        let pick_list = PickList::new(
            self.celestial_bodies.clone(),
            self.selected_focus.clone(),
            GuiMessage::FocusedBodySelected,
        )
        .width(200.);
        Row::new()
            .push(text)
            .push(pick_list)
            .align_items(Alignment::Center)
            .into()
    }

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
