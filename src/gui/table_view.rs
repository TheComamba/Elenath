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

pub(super) struct TableViewState {
    pub(super) table_col_data: Vec<TableColData>,
}

impl TableViewState {
    const CELL_WIDTH: f32 = 150.;
    const BUTTON_CELL_WIDTH: f32 = 50.;

    pub(super) fn new() -> TableViewState {
        TableViewState {
            table_col_data: TableColData::default_table_col_data(),
        }
    }

    pub(super) fn table_view(&self, bodies: &Vec<&PlanetData>) -> Element<'_, GuiMessage> {
        Column::new()
            .push(twoway_scrollable(self.table(bodies)))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn table(&self, bodies: &Vec<&PlanetData>) -> Element<'_, GuiMessage> {
        let width = Self::CELL_WIDTH + Self::CELL_WIDTH * self.table_col_data.len() as f32;
        let mut col = Column::new()
            .push(self.table_header())
            .push(Rule::horizontal(10));
        for body in bodies {
            col = col.push(self.table_row(body));
        }
        Container::new(col).width(iced::Length::Fixed(width)).into()
    }

    fn table_header(&self) -> Row<'static, GuiMessage> {
        let mut row = Row::new().push(
            Container::new(Text::new("")).width(iced::Length::Fixed(Self::BUTTON_CELL_WIDTH)),
        );
        for col in &self.table_col_data {
            row = row.push(Self::table_cell(Text::new(col.header).into()));
        }
        row.align_items(Alignment::Center)
    }

    fn table_row(&self, data: &PlanetData) -> Row<'_, GuiMessage> {
        let edit_button = Container::new(Button::new(Text::new("Edit")))
            .width(iced::Length::Fixed(Self::BUTTON_CELL_WIDTH));
        let mut row = Row::new().push(edit_button);
        for col in self.table_col_data.iter() {
            row = row.push(Self::table_cell(
                Text::new((col.content_closure)(data)).into(),
            ));
        }
        row.align_items(Alignment::Center)
    }

    fn table_cell(content: Element<'_, GuiMessage>) -> Container<'_, GuiMessage> {
        Container::new(content).width(iced::Length::Fixed(Self::CELL_WIDTH))
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
