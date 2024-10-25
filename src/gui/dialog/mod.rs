use super::message::GuiMessage;
use iced::{
    widget::{Container, Scrollable, Text},
    Element,
};
use iced_aw::{style, Card};
use load_real_stars::RealStarsEvent;
use new_system::NewSystemDialogEvent;
use planet::PlanetDialogEvent;

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

pub(crate) enum CardStyle {
    Primary,
    Warning,
    Error,
}

pub(crate) trait Dialog {
    fn card_style(&self) -> CardStyle {
        CardStyle::Primary
    }

    fn header(&self) -> String;

    fn body<'a>(&self) -> Element<'a, GuiMessage>;

    fn update(&mut self, message: DialogUpdate);

    fn submit(&self) -> GuiMessage;

    fn to_element<'a>(&self) -> Element<'a, GuiMessage> {
        let header: Text<'a> = Text::new(self.header());
        let body = self.body();
        let card =
            Card::new::<Element<'a, GuiMessage>, Element<'a, GuiMessage>>(header.into(), body)
                .on_close(GuiMessage::DialogClosed);

        let card = match self.card_style() {
            CardStyle::Primary => card.style(style::card::primary),
            CardStyle::Warning => card.style(style::card::warning),
            CardStyle::Error => card.style(style::card::danger),
        };
        Container::new(Scrollable::new(card)).padding(100).into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum DialogUpdate {
    LoadRealStarsUpdated(RealStarsEvent),
    NewSystemUpdated(NewSystemDialogEvent),
    PlanetUpdated(PlanetDialogEvent),
    Submit,
    Close,
}
