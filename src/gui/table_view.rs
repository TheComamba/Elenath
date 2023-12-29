use super::{Gui, GuiMessage};
use crate::model::celestial_body_data::CelestialBodyData;
use iced::{
    widget::{
        scrollable::{Direction, Properties},
        Button, Column, Container, Row, Rule, Scrollable, Text,
    },
    Alignment, Element,
};

const CELL_WIDTH: f32 = 250.;

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
        let mut col = Column::new()
            .push(Self::table_header())
            .push(Rule::horizontal(10));
        for body in self.celestial_system.get_bodies_data() {
            col = col.push(Self::table_row(body));
        }
        col.into()
    }

    fn table_header() -> Row<'static, GuiMessage> {
        let edit_button_space = Self::table_cell(Text::new("").into());
        let name = Self::table_cell(Text::new("Name").into());
        let mass = Self::table_cell(Text::new("Mass").into());
        let radius = Self::table_cell(Text::new("Radius").into());
        let albedo = Self::table_cell(Text::new("Albedo").into());
        let semi_major_axis = Self::table_cell(Text::new("Semi-major axis").into());
        let eccentricity = Self::table_cell(Text::new("Eccentricity").into());
        let inclination = Self::table_cell(Text::new("Inclination").into());
        let longitude_of_ascending_node =
            Self::table_cell(Text::new("Longitude of ascending node").into());
        let argument_of_periapsis = Self::table_cell(Text::new("Argument of periapsis").into());
        Row::new()
            .push(edit_button_space)
            .push(name)
            .push(mass)
            .push(radius)
            .push(albedo)
            .push(semi_major_axis)
            .push(eccentricity)
            .push(inclination)
            .push(longitude_of_ascending_node)
            .push(argument_of_periapsis)
            .align_items(Alignment::Center)
    }

    fn table_row(data: &CelestialBodyData) -> Row<'_, GuiMessage> {
        let edit_button = Self::table_cell(Button::new(Text::new("Edit")).into());
        let name = Self::table_cell(Text::new(data.get_name()).into());
        let mass = Self::table_cell(Text::new(format!("{}", data.get_mass())).into());
        let radius = Self::table_cell(Text::new(format!("{}", data.get_radius())).into());
        let albedo = Self::table_cell(Text::new(format!("{}", data.get_albedo())).into());
        let semi_major_axis = Self::table_cell(
            Text::new(format!(
                "{}",
                data.get_orbital_parameters().get_semi_major_axis()
            ))
            .into(),
        );
        let eccentricity = Self::table_cell(
            Text::new(format!(
                "{}",
                data.get_orbital_parameters().get_eccentricity()
            ))
            .into(),
        );
        let inclination = Self::table_cell(
            Text::new(format!(
                "{}",
                data.get_orbital_parameters().get_inclination()
            ))
            .into(),
        );
        let longitude_of_ascending_node = Self::table_cell(
            Text::new(format!(
                "{}",
                data.get_orbital_parameters()
                    .get_longitude_of_ascending_node()
            ))
            .into(),
        );
        let argument_of_periapsis = Self::table_cell(
            Text::new(format!(
                "{}",
                data.get_orbital_parameters().get_argument_of_periapsis()
            ))
            .into(),
        );
        Row::new()
            .push(edit_button)
            .push(name)
            .push(mass)
            .push(radius)
            .push(albedo)
            .push(semi_major_axis)
            .push(eccentricity)
            .push(inclination)
            .push(longitude_of_ascending_node)
            .push(argument_of_periapsis)
            .align_items(Alignment::Center)
    }

    fn table_cell(content: Element<'_, GuiMessage>) -> Container<'_, GuiMessage> {
        Container::new(content).width(iced::Length::Fixed(CELL_WIDTH))
    }
}
