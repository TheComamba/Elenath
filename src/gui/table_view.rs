use super::{Gui, GuiMessage};
use crate::model::celestial_body_data::CelestialBodyData;
use iced::{
    widget::{
        scrollable::{Direction, Properties},
        Button, Column, Container, Row, Rule, Scrollable, Text,
    },
    Alignment, Element,
};

const CELL_WIDTH: f32 = 150.;
const BUTTON_CELL_WIDTH: f32 = 50.;

impl Gui {
    pub(super) fn table_view(&self) -> Element<'_, GuiMessage> {
        let direction = Direction::Both {
            vertical: Properties::default(),
            horizontal: Properties::default(),
        };
        Scrollable::new(self.table())
            .direction(direction)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn table(&self) -> Element<'_, GuiMessage> {
        let width = CELL_WIDTH + CELL_WIDTH * self.table_col_data.len() as f32;
        let mut col = Column::new()
            .push(self.table_header())
            .push(Rule::horizontal(10));
        for body in self.celestial_system.get_bodies_data() {
            col = col.push(self.table_row(body));
        }
        Container::new(col).width(iced::Length::Fixed(width)).into()
    }

    fn table_header(&self) -> Row<'static, GuiMessage> {
        let mut row = Row::new()
            .push(Container::new(Text::new("")).width(iced::Length::Fixed(BUTTON_CELL_WIDTH)));
        for col in &self.table_col_data {
            row = row.push(Self::table_cell(Text::new(col.header).into()));
        }
        row.align_items(Alignment::Center)
    }

    fn table_row(&self, data: &CelestialBodyData) -> Row<'_, GuiMessage> {
        let edit_button = Container::new(Button::new(Text::new("Edit")))
            .width(iced::Length::Fixed(BUTTON_CELL_WIDTH));
        let mut row = Row::new().push(edit_button);
        for col in self.table_col_data.iter() {
            row = row.push(Self::table_cell(
                Text::new((col.content_closure)(data)).into(),
            ));
        }
        row.align_items(Alignment::Center)
    }

    fn table_cell(content: Element<'_, GuiMessage>) -> Container<'_, GuiMessage> {
        Container::new(content).width(iced::Length::Fixed(CELL_WIDTH))
    }
}
