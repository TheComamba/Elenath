use super::{
    gui_widget::{GuiMessage, BIG_COLUMN_WIDTH, PADDING, SMALL_COLUMN_WIDTH},
    Gui, GuiMode,
};
use astro_utils::{planets::planet::Planet, units::time::Time};
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Button, Column, Container, PickList, Row, Text, Toggler},
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

    pub(super) fn adding_buttons() -> iced::Element<'static, GuiMessage> {
        let add_planet_button = std_button("Add Planet", GuiMessage::AddPlanet);
        let add_star_button = std_button("Add Star", GuiMessage::AddStar);
        let generate_stars_button = std_button("Generate Stars", GuiMessage::GenerateStars);
        let fetch_gaia_data_button = std_button("Fetch Gaia Data", GuiMessage::FetchGaiaData);
        Row::new()
            .push(add_planet_button)
            .push(add_star_button)
            .push(generate_stars_button)
            .push(fetch_gaia_data_button)
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
    .width(SMALL_COLUMN_WIDTH)
}

pub(super) fn planet_picker<'a>(
    planets: &'a &Vec<Planet>,
    selected_planet: &'a Option<Planet>,
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
    planets: &'a &Vec<Planet>,
    selected_planet: &'a Option<Planet>,
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

pub(super) fn control_field<'a, M>(
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
