use astro_utils::stars::star_data::StarData;
use iced::Element;

use crate::gui::message::GuiMessage;

use super::Dialog;

#[derive(Debug, Clone)]
pub(crate) struct StarDialog {
    star: StarData,
    star_index: Option<usize>,
}

impl StarDialog {
    pub(crate) fn new() -> Self {
        StarDialog {
            star: todo!(),
            star_index: None,
        }
    }

    pub(crate) fn edit(star: StarData, star_index: usize) -> Self {
        StarDialog {
            star,
            star_index: Some(star_index),
        }
    }
}

impl Dialog for StarDialog {
    fn header(&self) -> String {
        match self.star_index {
            Some(index) => format!("Edit Star {}", index),
            None => "Create Star".to_string(),
        }
    }

    fn body<'a>(&self) -> Element<'a, GuiMessage> {
        todo!()
    }
}
