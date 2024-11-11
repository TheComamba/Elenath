use super::{Dialog, DialogUpdate, ElenathError};
use crate::{
    gui::{
        gui_widget::{PADDING, SMALL_COLUMN_WIDTH},
        message::GuiMessage,
    },
    model::star::StarDataType,
};
use iced::{
    widget::{Button, Column, Radio, Row, Text},
    Alignment, Element, Length,
};

#[derive(Debug, Clone)]
pub(crate) struct LoadRealStarsDialog {
    data_type: StarDataType,
}

impl LoadRealStarsDialog {
    pub(crate) fn new() -> Self {
        LoadRealStarsDialog {
            data_type: StarDataType::GaiaMeasurementSmall,
        }
    }
}

impl Dialog for LoadRealStarsDialog {
    fn header(&self) -> String {
        "Load Real Stars".to_string()
    }

    fn body<'a>(&'a self) -> Element<'a, GuiMessage> {
        let warning = Text::new("This will overwrite all stars in the current system.");

        let on_change = |data_type| {
            GuiMessage::DialogUpdate(DialogUpdate::LoadRealStarsUpdated(
                RealStarsEvent::DataTypeSelected(data_type),
            ))
        };
        let hardcoded_radio = Radio::new(
            "Hardcoded",
            StarDataType::Hardcoded,
            Some(self.data_type),
            on_change,
        )
        .width(SMALL_COLUMN_WIDTH);
        let measurement_small_radio = Radio::new(
            "Gaia Measurement (only brightest)",
            StarDataType::GaiaMeasurementSmall,
            Some(self.data_type),
            on_change,
        )
        .width(SMALL_COLUMN_WIDTH);
        let measurement_large_radio = Radio::new(
            "Gaia Measurement (loads)",
            StarDataType::GaiaMeasurementLarge,
            Some(self.data_type),
            on_change,
        )
        .width(SMALL_COLUMN_WIDTH);
        let simulation_radio = Radio::new(
            "GaiaSimulation",
            StarDataType::GaiaSimulation,
            Some(self.data_type),
            on_change,
        )
        .width(SMALL_COLUMN_WIDTH);
        let type_row = Row::new()
            .push(hardcoded_radio)
            .push(measurement_small_radio)
            .push(measurement_large_radio)
            .push(simulation_radio)
            .padding(PADDING)
            .spacing(PADDING);

        let submit_button = Button::new(Text::new("Submit")).on_press(GuiMessage::DialogSubmit);

        Column::new()
            .push(warning)
            .push(type_row)
            .push(submit_button)
            .padding(PADDING)
            .spacing(PADDING)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn update(&mut self, event: DialogUpdate) {
        if let DialogUpdate::LoadRealStarsUpdated(event) = event {
            match event {
                RealStarsEvent::DataTypeSelected(data_type) => {
                    self.data_type = data_type;
                }
            }
        }
    }

    fn submit(&self) -> GuiMessage {
        GuiMessage::LoadStars(self.data_type)
    }

    fn get_error(&self) -> Option<ElenathError> {
        None
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RealStarsEvent {
    DataTypeSelected(StarDataType),
}
