use super::col_data::{TableColData, TableDataType};
use crate::{
    gui::{gui_widget::PADDING, message::GuiMessage, shared_widgets::std_button},
    model::{
        part_of_celestial_system::{BodyType, PartOfCelestialSystem},
        planet::Planet,
        star::Star,
    },
};
use astro_utils::{stars::fate::StarFate, units::time::TIME_ZERO};
use iced::{
    widget::{
        scrollable::{Direction, Properties},
        Button, Column, Container, Row, Rule, Scrollable, Text,
    },
    Alignment, Element, Length,
};

const CELL_WIDTH: f32 = 150.;
const BUTTON_CELL_WIDTH: f32 = 50.;
const MAX_ROWS: usize = 250;

pub(crate) struct TableViewState {
    pub(crate) displayed_body_type: TableDataType,
}

impl TableViewState {
    pub(crate) fn new() -> TableViewState {
        TableViewState {
            displayed_body_type: TableDataType::Planet,
        }
    }

    pub(crate) fn table_view(
        &self,
        planets: Vec<Planet>,
        stars: Vec<Star>,
        is_system_loaded: bool,
    ) -> Element<'_, GuiMessage> {
        let table = match self.displayed_body_type {
            TableDataType::Planet => {
                let planet_col_data = TableColData::default_planet_col_data();
                table(
                    planet_col_data,
                    is_system_loaded,
                    planets,
                    GuiMessage::NewPlanetDialog,
                )
            }
            TableDataType::Star => {
                let star_col_data = TableColData::default_star_col_data();
                table(
                    star_col_data,
                    is_system_loaded,
                    stars,
                    GuiMessage::NewStarDialog,
                )
            }
            TableDataType::Supernova => {
                let supernova_col_data = TableColData::default_supernova_col_data();
                let mut supernovae: Vec<Star> = stars
                    .into_iter()
                    .filter(|s| {
                        if let Some(data) = s.get_data() {
                            data.get_fate() == &StarFate::TypeIISupernova
                        } else {
                            false
                        }
                    })
                    .collect();
                supernovae.sort_by(|a, b| {
                    a.get_data()
                        .unwrap()
                        .get_time_until_death(TIME_ZERO)
                        .partial_cmp(&b.get_data().unwrap().get_time_until_death(TIME_ZERO))
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                table(
                    supernova_col_data,
                    is_system_loaded,
                    supernovae,
                    GuiMessage::NewStarDialog,
                )
            }
        };

        Column::new()
            .push(data_type_selection_tabs())
            .push(table)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

fn table<T>(
    col_data: Vec<TableColData<T>>,
    is_system_loaded: bool,
    bodies: Vec<T>,
    new_message: GuiMessage,
) -> Scrollable<'static, GuiMessage>
where
    T: PartOfCelestialSystem,
{
    let width = table_width(&col_data);
    Scrollable::new(
        Column::new()
            .push(table_header(new_message, &col_data, is_system_loaded))
            .push(Container::new(Rule::horizontal(10)).width(width))
            .push(table_contents(bodies, col_data)),
    )
    .direction(Direction::Horizontal(Properties::default()))
    .width(Length::Fill)
    .height(Length::Fill)
}

fn table_width<T>(table_col_data: &[TableColData<T>]) -> Length {
    let planet_table_width =
        Length::Fixed(table_col_data.len() as f32 * CELL_WIDTH + 2. * BUTTON_CELL_WIDTH);
    planet_table_width
}

fn data_type_selection_tabs() -> Element<'static, GuiMessage> {
    let planet_button = std_button(
        "Planets",
        GuiMessage::TableDataTypeSelected(TableDataType::Planet),
        true,
    );
    let star_button = std_button(
        "Stars",
        GuiMessage::TableDataTypeSelected(TableDataType::Star),
        true,
    );
    let supernova_button = std_button(
        "Supernovae",
        GuiMessage::TableDataTypeSelected(TableDataType::Supernova),
        true,
    );
    Row::new()
        .push(planet_button)
        .push(star_button)
        .push(supernova_button)
        .align_items(Alignment::Center)
        .spacing(PADDING)
        .padding(PADDING)
        .into()
}

fn table_contents<T>(
    bodies: Vec<T>,
    table_col_data: Vec<TableColData<T>>,
) -> Element<'static, GuiMessage>
where
    T: PartOfCelestialSystem,
{
    let mut col = Column::new();
    let length = bodies.len();
    for (sorting_index, body) in bodies.into_iter().enumerate().take(MAX_ROWS) {
        col = col.push(table_row(sorting_index, body, &table_col_data));
    }
    if length > MAX_ROWS {
        col = col.push(Text::new(format!("... and {} more", length - MAX_ROWS)));
    }
    Scrollable::new(col)
        .direction(Direction::Vertical(Properties::default()))
        .height(iced::Length::Fill)
        .into()
}

fn table_header<T>(
    new_dialog_message: GuiMessage,
    table_col_data: &Vec<TableColData<T>>,
    is_system_loaded: bool,
) -> Row<'static, GuiMessage> {
    let mut new_button = Button::new("New");
    if is_system_loaded {
        new_button = new_button.on_press(new_dialog_message);
    }
    let mut row = Row::new()
        .push(Container::new(new_button).width(Length::Fixed(BUTTON_CELL_WIDTH)))
        .push(Container::new(Text::new("")).width(Length::Fixed(BUTTON_CELL_WIDTH)));
    for col in table_col_data {
        row = row.push(table_cell(Text::new(col.header).into()));
    }
    row.align_items(Alignment::Center)
}

fn table_row<T>(
    sorting_index: usize,
    data: T,
    table_col_data: &Vec<TableColData<T>>,
) -> Row<'static, GuiMessage>
where
    T: PartOfCelestialSystem,
{
    let mut edit_button = Button::new(Text::new("Edit"));
    let index = data.get_index();
    match data.get_body_type() {
        BodyType::Planet => {
            if let Some(index) = index {
                edit_button = edit_button.on_press(GuiMessage::EditPlanetDialog(index));
            }
        }
        BodyType::Star => {
            edit_button = edit_button.on_press(GuiMessage::EditStarDialog(data.get_index()));
        }
    }
    let mut row = Row::new()
        .push(Container::new(edit_button).width(iced::Length::Fixed(BUTTON_CELL_WIDTH)))
        .push(
            Container::new(Text::new(format!("{}", sorting_index + 1)))
                .width(Length::Fixed(BUTTON_CELL_WIDTH)),
        );
    for col in table_col_data.iter() {
        let content = (col.content_closure)(&data).unwrap_or("N/A".to_string());
        row = row.push(table_cell(Text::new(content).into()));
    }
    row.align_items(Alignment::Center)
}

fn table_cell(content: Element<'_, GuiMessage>) -> Container<'_, GuiMessage> {
    Container::new(content).width(iced::Length::Fixed(CELL_WIDTH))
}
