use super::Dialog;
use crate::{
    error::ElenathError,
    gui::{
        gui_widget::{PADDING, SMALL_COLUMN_WIDTH},
        message::GuiMessage,
    },
    model::{
        celestial_system::CelestialSystem,
        new_celestial_system::{gaia_universe_simulation, solar_system},
    },
};
use iced::{
    widget::{component, Button, Column, Component, Radio, Row, Text},
    Alignment, Element, Length,
};

#[derive(Debug, Clone)]
pub(crate) struct LoadGaiaDataDialog {
    data_type: GaiaDataType,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub(crate) enum GaiaDataType {
    Measurement,
    Simulation,
}

impl LoadGaiaDataDialog {
    pub(crate) fn new() -> Self {
        LoadGaiaDataDialog {
            data_type: GaiaDataType::Measurement,
        }
    }

    fn celestial_system(&self) -> Result<CelestialSystem, ElenathError> {
        match self.data_type {
            GaiaDataType::Measurement => solar_system(true),
            GaiaDataType::Simulation => gaia_universe_simulation(),
        }
    }
}

impl Dialog for LoadGaiaDataDialog {
    fn header(&self) -> String {
        "Load Gaia Data".to_string()
    }

    fn body<'a>(&self) -> Element<'a, GuiMessage> {
        component(self.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NewSystemDialogEvent {
    DataTypeSelected(GaiaDataType),
    Submit,
}

impl Component<GuiMessage> for LoadGaiaDataDialog {
    type State = ();

    type Event = NewSystemDialogEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMessage> {
        match event {
            NewSystemDialogEvent::DataTypeSelected(data_type) => {
                self.data_type = data_type;
            }
            NewSystemDialogEvent::Submit => {
                return Some(GuiMessage::NewSystem(self.celestial_system()));
            }
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let warning = Text::new("This will overwrite all stars in the current system.");

        let measurement_radio = Radio::new(
            "Measurement",
            GaiaDataType::Measurement,
            Some(self.data_type),
            NewSystemDialogEvent::DataTypeSelected,
        )
        .width(SMALL_COLUMN_WIDTH);
        let simulation_radio = Radio::new(
            "Simulation",
            GaiaDataType::Simulation,
            Some(self.data_type),
            NewSystemDialogEvent::DataTypeSelected,
        )
        .width(SMALL_COLUMN_WIDTH);
        let type_row = Row::new()
            .push(measurement_radio)
            .push(simulation_radio)
            .padding(PADDING)
            .spacing(PADDING);

        let submit_button = Button::new(Text::new("Submit")).on_press(NewSystemDialogEvent::Submit);

        Column::new()
            .push(warning)
            .push(type_row)
            .push(submit_button)
            .padding(PADDING)
            .spacing(PADDING)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}
