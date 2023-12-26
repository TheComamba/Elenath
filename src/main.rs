use gui::Gui;
use iced::{Sandbox, Settings};

mod gui;
mod model;

fn main() -> iced::Result {
    Gui::run(Settings::default())
}
