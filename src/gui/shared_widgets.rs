use crate::model::celestial_body::CelestialBody;

use super::{
    gui_widget::{GuiMessage, PADDING},
    Gui, GuiMode,
};
use astro_utils::units::time::Time;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Button, Column, Container, PickList, Row, Text},
    Alignment,
};

impl Gui {
    pub(super) fn gui_mode_tabs() -> iced::Element<'static, GuiMessage> {
        let local_view_button =
            std_button("Local View", GuiMessage::ModeSelected(GuiMode::SurfaceView));
        let top_view_button = std_button("Top View", GuiMessage::ModeSelected(GuiMode::TopView));
        let table_view_button =
            std_button("Table View", GuiMessage::ModeSelected(GuiMode::TableView));
        Row::new()
            .push(local_view_button)
            .push(top_view_button)
            .push(table_view_button)
            .align_items(Alignment::Center)
            .spacing(PADDING)
            .into()
    }

    pub(super) fn file_buttons() -> iced::Element<'static, GuiMessage> {
        let save_to_file_button = std_button("Save to file", GuiMessage::SaveToFile);
        let save_to_new_file_button = std_button("Save to new file", GuiMessage::SaveToNewFile);
        let open_file_button = std_button("Open file", GuiMessage::OpenFile);
        Row::new()
            .push(save_to_file_button)
            .push(save_to_new_file_button)
            .push(open_file_button)
            .align_items(Alignment::Center)
            .spacing(PADDING)
            .into()
    }
}

fn std_button(text: &str, message: GuiMessage) -> Button<'_, GuiMessage> {
    Button::new(
        Text::new(text)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .on_press(message)
    .width(150.)
}

pub(super) fn time_control_fields<'a>(
    time_since_epoch: &'a Time,
    time_step: &'a Time,
) -> iced::Element<'a, GuiMessage> {
    let time_control_field = control_field(
        "Time since Epoch:",
        format!("{}", time_since_epoch),
        GuiMessage::UpdateTime(*time_since_epoch - *time_step),
        GuiMessage::UpdateTime(*time_since_epoch + *time_step),
    );
    let time_step_control_field = control_field(
        "Time step:",
        format!("{}", time_step),
        GuiMessage::UpdateTimeStep(*time_step / 2.),
        GuiMessage::UpdateTimeStep(*time_step * 2.),
    );
    Column::new()
        .push(time_control_field)
        .push(time_step_control_field)
        .width(iced::Length::Fill)
        .align_items(Alignment::Center)
        .into()
}

pub(super) fn planet_picker<'a>(
    celestial_bodies: &'a Vec<CelestialBody>,
    selected_body: &'a Option<CelestialBody>,
) -> iced::Element<'a, GuiMessage> {
    let text = Text::new("Focused body:").width(150.);
    let pick_list = PickList::new(
        celestial_bodies.clone(),
        selected_body.clone(),
        GuiMessage::FocusedBodySelected,
    )
    .width(200.);
    Row::new()
        .push(text)
        .push(pick_list)
        .align_items(Alignment::Center)
        .into()
}

pub(super) fn control_field<'a, M>(
    label: &'a str,
    value: String,
    decrease: M,
    increase: M,
) -> Row<'a, GuiMessage>
where
    M: Into<GuiMessage>,
{
    let label = Container::new(Text::new(label))
        .align_x(Horizontal::Center)
        .width(iced::Length::Fixed(150.));
    let decrease_button = Container::new(Button::new(Text::new("<<")).on_press(decrease.into()))
        .align_x(Horizontal::Center)
        .width(iced::Length::Fixed(50.));
    let value = Container::new(Text::new(value))
        .width(iced::Length::Fixed(100.))
        .align_x(Horizontal::Center);
    let increase_button = Container::new(Button::new(Text::new(">>")).on_press(increase.into()))
        .align_x(Horizontal::Center)
        .width(iced::Length::Fixed(50.));
    Row::new()
        .push(label)
        .push(decrease_button)
        .push(value)
        .push(increase_button)
        .align_items(Alignment::Center)
}
