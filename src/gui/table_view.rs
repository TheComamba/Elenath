use super::{Gui, GuiMessage};
use crate::model::celestial_body_data::CelestialBodyData;
use iced::{
    widget::{Button, Column, Container, Row, Rule, Text},
    Element,
};

const CELL_WIDTH: f32 = 100.;

impl Gui {
    pub(super) fn table_view(&self) -> Element<'_, GuiMessage> {
        let mut col = Column::new()
            .push(Self::table_header())
            .push(Rule::horizontal(10));
        for body in self.celestial_system.get_bodies_data() {
            col = col.push(Self::table_row(body));
        }
        col.width(iced::Length::Fill).into()
    }

    fn table_header() -> Row<'static, GuiMessage> {
        let edit_button_space = Self::table_cell(Text::new("").into());
        let name = Self::table_cell(Text::new("Name").into());
        Row::new().push(edit_button_space).push(name)
    }

    fn table_row(data: &CelestialBodyData) -> Row<'_, GuiMessage> {
        let edit_button = Self::table_cell(Button::new(Text::new("Edit")).into());
        let name = Self::table_cell(Text::new(data.get_name()).into());
        Row::new().push(edit_button).push(name)
    }

    fn table_cell(content: Element<'_, GuiMessage>) -> Container<'_, GuiMessage> {
        Container::new(content).width(iced::Length::Fixed(CELL_WIDTH))
    }
}
