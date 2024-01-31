use super::table_col_data::TableColData;
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
    Alignment, Element,
};

const CELL_WIDTH: f32 = 150.;
const BUTTON_CELL_WIDTH: f32 = 50.;

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

    pub(crate) fn table_view<'a>(
        &'a self,
        planets: Vec<Planet>,
        stars: Vec<Star>,
    ) -> Element<'_, GuiMessage> {
        Column::new()
            .push(table_header(&self.planet_col_data))
            .push(Rule::horizontal(10))
            .push(twoway_scrollable(table(planets, &self.planet_col_data)))
            .push(table_header(&self.star_col_data))
            .push(Rule::horizontal(10))
            .push(twoway_scrollable(table(stars, &self.star_col_data)))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

fn twoway_scrollable<'a>(child: impl Into<Element<'a, GuiMessage>>) -> Element<'a, GuiMessage> {
    let direction = Direction::Both {
        vertical: Properties::default(),
        horizontal: Properties::default(),
    };
    Scrollable::new(child)
        .direction(direction)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}

fn table<'a, T>(bodies: Vec<T>, table_col_data: &'a Vec<TableColData<T>>) -> Element<'a, GuiMessage>
where
    T: PartOfCelestialSystem,
{
    let mut col = Column::new();
    for body in bodies {
        col = col.push(table_row(body, table_col_data));
    }
    col.into()
}

fn table_header<T>(table_col_data: &Vec<TableColData<T>>) -> Row<'static, GuiMessage> {
    let mut row = Row::new()
        .push(Container::new(Text::new("")).width(iced::Length::Fixed(BUTTON_CELL_WIDTH)));
    for col in table_col_data {
        row = row.push(table_cell(Text::new(col.header).into()));
    }
    row.align_items(Alignment::Center)
}

fn table_row<'a, T>(data: T, table_col_data: &'a Vec<TableColData<T>>) -> Row<'a, GuiMessage>
where
    T: PartOfCelestialSystem,
{
    let mut edit_button = Button::new(Text::new("Edit"));
    if let Some(index) = data.get_index() {
        match data.get_body_type() {
            BodyType::Planet => {
                edit_button = edit_button.on_press(GuiMessage::EditPlanetDialog(index));
            }
            BodyType::Star => {
                edit_button = edit_button.on_press(GuiMessage::EditStarDialog(index));
            }
        }
    }
    let edit_button = Container::new(edit_button).width(iced::Length::Fixed(BUTTON_CELL_WIDTH));
    let mut row = Row::new().push(edit_button);
    for col in table_col_data.iter() {
        row = row.push(table_cell(Text::new((col.content_closure)(&data)).into()));
    }
    row.align_items(Alignment::Center)
}

fn table_cell(content: Element<'_, GuiMessage>) -> Container<'_, GuiMessage> {
    Container::new(content).width(iced::Length::Fixed(CELL_WIDTH))
}
