#![warn(clippy::unwrap_used)]

use gui::Gui;
use iced::{window, Result, Size};

mod error;
mod file_dialog;
mod gui;
mod model;

static APP_TITLE: &str = "Elenath";

fn main() -> Result {
    let window_settings = window::Settings {
        size: (Size {
            width: 1820.,
            height: 980.,
        }),
        ..Default::default()
    };
    iced::application(APP_TITLE, Gui::update, Gui::view)
        .antialiasing(true)
        .window(window_settings)
        .run()
}
