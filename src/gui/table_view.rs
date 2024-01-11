use super::gui_widget::GuiMessage;
use super::table_col_data::TableColData;
use astro_utils::{planets::planet::Planet, stars::star_data::StarData};
use iced::{
    widget::{
        scrollable::{Direction, Properties},
        Button, Column, Container, Row, Rule, Scrollable, Text,
    },
    Alignment, Element,
};

const CELL_WIDTH: f32 = 150.;
const BUTTON_CELL_WIDTH: f32 = 50.;

pub(super) struct TableViewState {
    planet_col_data: Vec<TableColData<Planet>>,
    star_col_data: Vec<TableColData<StarData>>,
}

impl TableViewState {
    pub(super) fn new() -> TableViewState {
        TableViewState {
            planet_col_data: TableColData::default_planet_col_data(),
            star_col_data: TableColData::default_star_col_data(),
        }
    }

    pub(super) fn table_view<'a>(
        &'a self,
        planets: Vec<&'a Planet>,
        stars: Vec<&'a Star>,
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

fn table<'a, T>(
    bodies: Vec<&'a T>,
    table_col_data: &'a Vec<TableColData<T>>,
) -> Element<'a, GuiMessage> {
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

fn table_row<'a, T>(data: &'a T, table_col_data: &'a Vec<TableColData<T>>) -> Row<'a, GuiMessage> {
    let edit_button = Container::new(Button::new(Text::new("Edit")))
        .width(iced::Length::Fixed(BUTTON_CELL_WIDTH));
    let mut row = Row::new().push(edit_button);
    for col in table_col_data.iter() {
        row = row.push(table_cell(Text::new((col.content_closure)(data)).into()));
    }
    row.align_items(Alignment::Center)
}

fn table_cell(content: Element<'_, GuiMessage>) -> Container<'_, GuiMessage> {
    Container::new(content).width(iced::Length::Fixed(CELL_WIDTH))
}
