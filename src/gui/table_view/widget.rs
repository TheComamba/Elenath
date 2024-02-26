use super::col_data::{TableColData, TableDataType};
use crate::{
    gui::{gui_widget::PADDING, message::GuiMessage, shared_widgets::std_button},
    model::{
        celestial_system::CelestialSystem,
        part_of_celestial_system::{BodyType, PartOfCelestialSystem},
    },
};
use iced::{
    widget::{
        scrollable::{Direction, Properties},
        Button, Column, Container, Row, Rule, Scrollable, Text,
    },
    Alignment, Element, Length,
};
use simple_si_units::base::Time;

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
        system: &Option<CelestialSystem>,
        time_since_epoch: Time<f64>,
    ) -> Element<'_, GuiMessage> {
        let mut col = Column::new().push(data_type_selection_tabs());

        if let Some(system) = system {
            let table = match self.displayed_body_type {
                TableDataType::Planet => {
                    let planet_col_data = TableColData::default_planet_col_data();
                    let planets = system.get_planets();
                    table(planet_col_data, planets, GuiMessage::NewPlanetDialog)
                }
                TableDataType::Star => {
                    let star_col_data = TableColData::default_star_col_data();
                    let stars = system.get_stars();
                    table(star_col_data, stars, GuiMessage::NewStarDialog)
                }
                TableDataType::Supernova => {
                    let supernova_col_data = TableColData::default_supernova_col_data();
                    let supernovae = system.get_supernovae();
                    table(supernova_col_data, supernovae, GuiMessage::NewStarDialog)
                }
            };
            col = col.push(table);
        }

        col.width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

fn table<T>(
    col_data: Vec<TableColData<T>>,
    bodies: Vec<T>,
    new_message: GuiMessage,
) -> Scrollable<'static, GuiMessage>
where
    T: PartOfCelestialSystem,
{
    let width = table_width(&col_data);
    Scrollable::new(
        Column::new()
            .push(table_header(new_message, &col_data))
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
) -> Row<'static, GuiMessage> {
    let new_button = Button::new("New").on_press(new_dialog_message);

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
