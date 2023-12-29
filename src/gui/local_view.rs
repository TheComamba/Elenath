use super::{Gui, GuiMessage};
use iced::{widget::Column, Alignment};

impl Gui {
    pub(super) fn local_view_control_field(&self) -> iced::Element<'_, GuiMessage> {
        let planet_picker = self.planet_picker();
        Column::new()
            .push(self.time_control_fields())
            .push(planet_picker)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}
