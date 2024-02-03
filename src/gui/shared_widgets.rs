use std::fmt::Display;

use super::{
    gui_widget::{BIG_COLUMN_WIDTH, PADDING, SMALL_COLUMN_WIDTH},
    message::GuiMessage,
    Gui, GuiMode,
};
use astro_utils::{planets::planet_data::PlanetData, units::time::Time};
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Button, Column, Container, PickList, Row, Text, TextInput, Toggler},
    Alignment, Renderer,
};
use iced_aw::Element;

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

    pub(super) fn real_system_file_buttons() -> iced::Element<'static, GuiMessage> {
        let new_button = std_button("New system", GuiMessage::NewSystemDialog);
        let open_file_button = std_button("Open file", GuiMessage::OpenFile);
        Row::new()
            .push(new_button)
            .push(open_file_button)
            .align_items(Alignment::Center)
            .spacing(PADDING)
            .into()
    }

    pub(super) fn generated_system_file_buttons(
        has_system: bool,
    ) -> iced::Element<'static, GuiMessage> {
        let new_button = std_button("New system", GuiMessage::NewSystemDialog);
        let mut row = Row::new().push(new_button);
        if has_system {
            let save_to_file_button = std_button("Save to file", GuiMessage::SaveToFile);
            let save_to_new_file_button = std_button("Save to new file", GuiMessage::SaveToNewFile);
            row = row.push(save_to_file_button).push(save_to_new_file_button);
        }
        let open_file_button = std_button("Open file", GuiMessage::OpenFile);

        row = row.push(open_file_button);
        row.align_items(Alignment::Center).spacing(PADDING).into()
    }
}

fn std_button(text: &str, message: GuiMessage) -> Button<'_, GuiMessage> {
    Button::new(
        Text::new(text)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .on_press(message)
    .width(SMALL_COLUMN_WIDTH)
}

pub(super) fn planet_picker<'a>(
    planets: Vec<&PlanetData>,
    selected_planet: Option<&PlanetData>,
) -> iced::Element<'a, GuiMessage> {
    let text = Text::new("Focused body:")
        .width(SMALL_COLUMN_WIDTH)
        .horizontal_alignment(Horizontal::Right)
        .vertical_alignment(Vertical::Center);
    let mut planet_names = vec![String::new()];
    for name in planets.iter().map(|p| p.get_name()) {
        planet_names.push(name.clone());
    }
    let selected_planet_name = match selected_planet {
        Some(planet) => planet.get_name().clone(),
        None => String::new(),
    };
    let pick_list = PickList::new(
        planet_names,
        Some(selected_planet_name),
        GuiMessage::PlanetSelected,
    )
    .width(1.25 * SMALL_COLUMN_WIDTH + PADDING);
    Row::new()
        .push(text)
        .push(pick_list)
        .spacing(PADDING)
        .align_items(Alignment::Center)
        .into()
}

pub(super) fn surface_and_top_view_shared_control<'a>(
    time_since_epoch: &'a Time,
    time_step: &'a Time,
    planets: Vec<&PlanetData>,
    selected_planet: Option<&PlanetData>,
    display_names: bool,
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
    let planet_picker = planet_picker(planets, selected_planet);
    let display_names_toggle = Container::new(Toggler::new(
        Some("Display Names".to_string()),
        display_names,
        |state| GuiMessage::SetShowNames(state),
    ))
    .width(iced::Length::Fixed(1.5 * SMALL_COLUMN_WIDTH));
    Column::new()
        .push(time_control_field)
        .push(time_step_control_field)
        .push(planet_picker)
        .push(display_names_toggle)
        .width(iced::Length::Fixed(BIG_COLUMN_WIDTH))
        .align_items(Alignment::Center)
        .spacing(PADDING)
        .into()
}

pub(crate) fn control_field<'a, M>(
    label: &'a str,
    value: String,
    decrease: M,
    increase: M,
) -> Row<'a, GuiMessage>
where
    M: Into<GuiMessage>,
{
    let label = Text::new(label)
        .vertical_alignment(Vertical::Center)
        .horizontal_alignment(Horizontal::Right)
        .width(iced::Length::Fixed(SMALL_COLUMN_WIDTH));
    let decrease_button = Container::new(Button::new(Text::new("<<")).on_press(decrease.into()))
        .align_x(Horizontal::Center)
        .width(iced::Length::Fixed(0.25 * SMALL_COLUMN_WIDTH));
    let value = Text::new(value)
        .width(iced::Length::Fixed(0.75 * SMALL_COLUMN_WIDTH))
        .horizontal_alignment(Horizontal::Center);
    let increase_button = Container::new(Button::new(Text::new(">>")).on_press(increase.into()))
        .align_x(Horizontal::Center)
        .width(iced::Length::Fixed(0.25 * SMALL_COLUMN_WIDTH));
    Row::new()
        .push(label)
        .push(decrease_button)
        .push(value)
        .push(increase_button)
        .spacing(PADDING)
        .align_items(Alignment::Center)
}

pub(crate) fn edit<'a, Fun, Mes, Val>(
    description: &'static str,
    data: &String,
    units: &'static str,
    message: Fun,
    actual_value: &Option<Val>,
) -> Element<'a, Mes, Renderer>
where
    Fun: 'a + Fn(String) -> Mes,
    Mes: 'a + Clone,
    Val: 'a + Display,
{
    let description = if description.ends_with(":") {
        description.to_string()
    } else {
        format!("{}:", description)
    };
    let description = Text::new(description)
        .width(SMALL_COLUMN_WIDTH)
        .horizontal_alignment(Horizontal::Right);
    let data = TextInput::new("", &data)
        .on_input(message)
        .width(SMALL_COLUMN_WIDTH);
    let units = Text::new(units).width(SMALL_COLUMN_WIDTH);
    let parsed_text = match actual_value {
        Some(value) => format! {"Parsed value:\n{}",value},
        None => "Parsed value:\nNone".to_string(),
    };
    let value = Text::new(parsed_text).width(SMALL_COLUMN_WIDTH);
    Row::new()
        .push(description)
        .push(data)
        .push(units)
        .push(value)
        .spacing(PADDING)
        .into()
}
