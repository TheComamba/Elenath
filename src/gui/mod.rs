use self::topview::TopViewState;
use crate::model::{celestial_body::CelestialBodyData, example::sun};
use astro_utils::{
    units::{length::Length, time::Time},
    Float,
};
use iced::{
    widget::{canvas, Button, Column, Row, Text},
    Sandbox,
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
}

impl Gui {
    fn topview_control_field(&self) -> iced::Element<'_, GuiMessage> {
        Column::new()
            .push(self.time_control_field())
            .push(self.time_step_control_field())
            .push(self.length_scale_control_field())
            .width(iced::Length::Fill)
            .into()
    }

    fn time_control_field(&self) -> Row<'_, GuiMessage> {
        let decrease_time_button = Button::new(Text::new("<<"))
            .on_press(GuiMessage::UpdateTime(self.time - self.time_step));
        let increase_time_button = Button::new(Text::new(">>"))
            .on_press(GuiMessage::UpdateTime(self.time + self.time_step));
        Row::new()
            .push(Text::new("Time: "))
            .push(decrease_time_button)
            .push(Text::new(format!("{}", self.time)))
            .push(increase_time_button)
            .width(iced::Length::Fill)
    }

    fn time_step_control_field(&self) -> Row<'_, GuiMessage> {
        let decrease_time_step_button =
            Button::new(Text::new("<<")).on_press(GuiMessage::UpdateTimeStep(self.time_step / 2.));
        let increase_time_step_button =
            Button::new(Text::new(">>")).on_press(GuiMessage::UpdateTimeStep(self.time_step * 2.));
        Row::new()
            .push(Text::new("Time step: "))
            .push(decrease_time_step_button)
            .push(Text::new(format!("{}", self.time_step)))
            .push(increase_time_step_button)
            .width(iced::Length::Fill)
    }

    fn length_scale_control_field(&self) -> Row<'_, GuiMessage> {
        let m_per_px = self.topview_state.get_meter_per_pixel();
        let decrease_length_scale_button =
            Button::new(Text::new("<<")).on_press(GuiMessage::UpdateLengthScale(m_per_px / 2.));
        let increase_length_scale_button =
            Button::new(Text::new(">>")).on_press(GuiMessage::UpdateLengthScale(m_per_px * 2.));
        let length_scale_control_field = Row::new()
            .push(Text::new("Length scale: "))
            .push(decrease_length_scale_button)
            .push(Text::new(format!("{}", Length::from_meters(m_per_px))))
            .push(increase_length_scale_button)
            .width(iced::Length::Fill);
        length_scale_control_field
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum GuiMessage {
    UpdateTime(Time),
    UpdateTimeStep(Time),
    UpdateLengthScale(Float),
}
