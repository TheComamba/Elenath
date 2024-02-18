use super::col_data::TableColData;
use crate::{
    gui::message::GuiMessage,
    model::{
        part_of_celestial_system::{BodyType, PartOfCelestialSystem},
        planet::Planet,
        star::Star,
    },
};
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
    planet_col_data: Vec<TableColData<Planet>>,
    star_col_data: Vec<TableColData<Star>>,
}

impl TableViewState {
    pub(crate) fn new() -> TableViewState {
        TableViewState {
            planet_col_data: TableColData::default_planet_col_data(),
            star_col_data: TableColData::default_star_col_data(),
        }
    }

    pub(crate) fn table_view(
        &self,
        planets: Vec<Planet>,
        stars: Vec<Star>,
        is_system_loaded: bool,
    ) -> Element<'_, GuiMessage> {
        let planet_table_width =
            Length::Fixed(self.planet_col_data.len() as f32 * CELL_WIDTH + 2. * BUTTON_CELL_WIDTH);
        let planet_table = Scrollable::new(
            Column::new()
                .push(table_header(
                    GuiMessage::NewPlanetDialog,
                    &self.planet_col_data,
                    is_system_loaded,
                ))
                .push(Container::new(Rule::horizontal(10)).width(planet_table_width))
                .push(table(planets, &self.planet_col_data)),
        )
        .direction(Direction::Horizontal(Properties::default()))
        .width(Length::Fill)
        .height(Length::Fill);

        let star_table_width =
            Length::Fixed(self.star_col_data.len() as f32 * CELL_WIDTH + 2. * BUTTON_CELL_WIDTH);
        let star_table = Scrollable::new(
            Column::new()
                .push(table_header(
                    GuiMessage::NewStarDialog,
                    &self.star_col_data,
                    is_system_loaded,
                ))
                .push(Container::new(Rule::horizontal(10)).width(star_table_width))
                .push(table(stars, &self.star_col_data)),
        )
        .direction(Direction::Horizontal(Properties::default()))
        .width(Length::Fill)
        .height(Length::Fill);

        Column::new()
            .push(planet_table)
            .push(star_table)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

fn table<T>(bodies: Vec<T>, table_col_data: &[TableColData<T>]) -> Element<'_, GuiMessage>
where
    T: PartOfCelestialSystem,
{
    let mut col = Column::new();
    let length = bodies.len();
    for (sorting_index, body) in bodies.into_iter().enumerate().take(MAX_ROWS) {
        col = col.push(table_row(sorting_index, body, table_col_data));
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
    table_col_data: &[TableColData<T>],
) -> Row<'_, GuiMessage>
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
