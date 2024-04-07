use super::Dialog;
use crate::{
    gui::{
        gui_widget::{PADDING, SMALL_COLUMN_WIDTH},
        message::GuiMessage,
    },
    model::star::StarDataType,
};
use iced::{
    widget::{component, Button, Column, Component, Radio, Row, Text},
    Alignment, Element, Length,
};

#[derive(Debug, Clone)]
pub(crate) struct LoadRealStarsDialog {
    data_type: StarDataType,
}

impl LoadRealStarsDialog {
    pub(crate) fn new() -> Self {
        LoadRealStarsDialog {
            data_type: StarDataType::GaiaMeasurement,
        }
    }
}

impl Dialog for LoadRealStarsDialog {
    fn header(&self) -> String {
        "Load Real Stars".to_string()
    }

    fn body<'a>(&self) -> Element<'a, GuiMessage> {
        component(self.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NewSystemDialogEvent {
    DataTypeSelected(StarDataType),
    Submit,
}

impl Component<GuiMessage> for LoadRealStarsDialog {
    type State = ();

    type Event = NewSystemDialogEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMessage> {
        match event {
            NewSystemDialogEvent::DataTypeSelected(data_type) => {
                self.data_type = data_type;
            }
            NewSystemDialogEvent::Submit => {
                return Some(GuiMessage::LoadStars(self.data_type));
            }
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let warning = Text::new("This will overwrite all stars in the current system.");

        let hardcoded_radio = Radio::new(
            "Hardcoded",
            StarDataType::Hardcoded,
            Some(self.data_type),
            NewSystemDialogEvent::DataTypeSelected,
        )
        .width(SMALL_COLUMN_WIDTH);
        let measurement_radio = Radio::new(
            "GaiaMeasurement",
            StarDataType::GaiaMeasurement,
            Some(self.data_type),
            NewSystemDialogEvent::DataTypeSelected,
        )
        .width(SMALL_COLUMN_WIDTH);
        let simulation_radio = Radio::new(
            "GaiaSimulation",
            StarDataType::GaiaSimulation,
            Some(self.data_type),
            NewSystemDialogEvent::DataTypeSelected,
        )
        .width(SMALL_COLUMN_WIDTH);
        let type_row = Row::new()
            .push(hardcoded_radio)
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
