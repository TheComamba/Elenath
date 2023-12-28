use self::topview::TopViewState;
use crate::model::{celestial_body::CelestialBodyData, example::sun};
use astro_utils::{
    units::{length::Length, time::Time},
    Float,
};
use iced::{
    alignment::Horizontal,
    widget::{canvas, Button, Column, Container, Row, Text},
    Alignment, Sandbox,
};

mod topview;

pub(crate) struct Gui {
    time: Time,
    time_step: Time,
    topview_state: TopViewState,
    central_body_data: CelestialBodyData,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        let central_body_data = sun();
        Gui {
            time: Time::from_days(0.0),
            time_step: Time::from_days(1.0),
            topview_state: TopViewState::new(central_body_data.system(Time::from_days(0.0))),
            central_body_data,
        }
    }

    fn title(&self) -> String {
        String::from("Elenath - Imaginary Skies")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            GuiMessage::UpdateTime(time) => {
                self.time = time;
                self.topview_state
                    .set_celestial_bodies(self.central_body_data.system(self.time));
                self.topview_state.redraw();
            }
            GuiMessage::UpdateTimeStep(time_step) => {
                self.time_step = time_step;
            }
            GuiMessage::UpdateLengthScale(m_per_px) => {
                self.topview_state.set_meter_per_pixel(m_per_px);
                self.topview_state.redraw();
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Column::new()
            .push(self.topview_control_field())
            .push(
                canvas(&self.topview_state)
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

impl Gui {
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
        Column::new()
            .push(time_control_field)
            .push(time_step_control_field)
            .push(length_scale_control_field)
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
            .align_x(Horizontal::Center)
            .width(iced::Length::Fixed(100.));
        let increase_button = Container::new(Button::new(Text::new(">>")).on_press(increase))
            .align_x(Horizontal::Center)
            .width(iced::Length::Fixed(50.));
        Row::new()
            .push(label)
            .push(decrease_button)
            .push(value)
            .push(increase_button)
            //.width(iced::Length::Fill)
            .align_items(Alignment::Center)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum GuiMessage {
    UpdateTime(Time),
    UpdateTimeStep(Time),
    UpdateLengthScale(Float),
}
