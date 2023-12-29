use super::{Gui, GuiMessage, GuiMode};
use iced::{
    alignment::Horizontal,
    widget::{Button, Column, Container, PickList, Row, Text},
    Alignment,
};

impl Gui {
    pub(super) fn gui_mode_tabs(&self) -> iced::Element<'_, GuiMessage> {
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

    pub(super) fn control_field<'a>(
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

    pub(super) fn time_control_fields(&self) -> iced::Element<'_, GuiMessage> {
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
        Column::new()
            .push(time_control_field)
            .push(time_step_control_field)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }

    pub(super) fn planet_picker(&self) -> iced::Element<'_, GuiMessage> {
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
}
