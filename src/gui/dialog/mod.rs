use super::message::GuiMessage;
use iced::{
    widget::{Container, Scrollable, Text},
    Element,
};
use iced_aw::{style::card, Card};

pub(crate) mod error;
pub(crate) mod load_real_planets;
pub(crate) mod load_real_stars;
pub(crate) mod new_system;
pub(crate) mod planet;
pub(crate) mod randomize_planets;
pub(crate) mod randomize_stars;
pub(crate) mod star;

#[derive(Debug, Clone)]
pub(crate) enum DialogType {
    NewSystem,
    NewPlanet,
    EditPlanet(usize),
    NewStar,
    EditStar(Option<usize>),
    LoadRealPlanets,
    RandomizePlanets,
    LoadGaiaData,
    RandomizeStars,
}

pub(crate) trait Dialog {
    fn card_style(&self) -> card::Style {
        card::Style::Primary
    }

    fn header(&self) -> String;

    fn body<'a>(&self) -> Element<'a, GuiMessage>;

    fn to_element<'a>(&self) -> Element<'a, GuiMessage> {
        let header: Text<'a> = Text::new(self.header());
        let body = self.body();
        let card =
            Card::new::<Element<'a, GuiMessage>, Element<'a, GuiMessage>>(header.into(), body)
                .style(self.card_style())
                .on_close(GuiMessage::DialogClosed);
        Container::new(Scrollable::new(card)).padding(100).into()
    }
}
