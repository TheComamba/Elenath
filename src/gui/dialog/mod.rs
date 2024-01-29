use super::message::GuiMessage;
use iced::{
    widget::{Container, Scrollable, Text},
    Element, Renderer,
};
use iced_aw::{style::CardStyles, Card};

pub(crate) mod error;
pub(crate) mod new_system;
pub(crate) mod planet;

pub(crate) trait Dialog {
    fn card_style(&self) -> CardStyles {
        CardStyles::Primary
    }

    fn header(&self) -> String;

    fn body<'a>(&self) -> Element<'a, GuiMessage>;

    fn to_element<'a>(&self) -> Element<'a, GuiMessage> {
        let header: Text<'a, Renderer> = Text::new(self.header());
        let body = self.body();
        let card =
            Card::new::<Element<'a, GuiMessage>, Element<'a, GuiMessage>>(header.into(), body)
                .style(self.card_style())
                .on_close(GuiMessage::DialogClosed);
        Container::new(Scrollable::new(card)).padding(10).into()
    }
}
