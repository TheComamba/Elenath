use super::gui_widget::GuiMessage;
use super::table_col_data::TableColData;
use crate::model::planet_data::PlanetData;
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
    pub(super) planet_col_data: Vec<TableColData<PlanetData>>,
}

impl TableViewState {
    pub(super) fn new() -> TableViewState {
        TableViewState {
            planet_col_data: TableColData::default_planet_col_data(),
        }
    }

    pub(super) fn table_view(&self, planets: Vec<&PlanetData>) -> Element<'_, GuiMessage> {
        Column::new()
            .push(twoway_scrollable(
                self.table(planets, &self.planet_col_data),
            ))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn table<T>(
        &self,
        bodies: Vec<&T>,
        table_col_data: &Vec<TableColData<T>>,
    ) -> Element<'_, GuiMessage> {
        let width = CELL_WIDTH + CELL_WIDTH * table_col_data.len() as f32;
        let mut col = Column::new()
            .push(self.table_header(table_col_data))
            .push(Rule::horizontal(10));
        for body in bodies {
            col = col.push(self.table_row(body, table_col_data));
        }
        Container::new(col).width(iced::Length::Fixed(width)).into()
    }

    fn table_header<T>(&self, table_col_data: &Vec<TableColData<T>>) -> Row<'static, GuiMessage> {
        let mut row = Row::new()
            .push(Container::new(Text::new("")).width(iced::Length::Fixed(BUTTON_CELL_WIDTH)));
        for col in table_col_data {
            row = row.push(table_cell(Text::new(col.header).into()));
        }
        row.align_items(Alignment::Center)
    }

    fn table_row<T>(&self, data: &T, table_col_data: &Vec<TableColData<T>>) -> Row<'_, GuiMessage> {
        let edit_button = Container::new(Button::new(Text::new("Edit")))
            .width(iced::Length::Fixed(BUTTON_CELL_WIDTH));
        let mut row = Row::new().push(edit_button);
        for col in table_col_data.iter() {
            row = row.push(table_cell(Text::new((col.content_closure)(data)).into()));
        }
        row.align_items(Alignment::Center)
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

fn table_cell(content: Element<'_, GuiMessage>) -> Container<'_, GuiMessage> {
    Container::new(content).width(iced::Length::Fixed(CELL_WIDTH))
}
