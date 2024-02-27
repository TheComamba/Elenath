use gui::Gui;
use iced::{settings, window, Result, Sandbox, Size};

mod error;
mod file_dialog;
mod gui;
mod model;

fn main() -> Result {
    let window_settings = window::Settings {
        size: (Size {
            width: 1820.,
            height: 980.,
        }),
        ..Default::default()
    };
    let settings = settings::Settings {
        window: window_settings,
        antialiasing: true,
        ..Default::default()
    };
    Gui::run(settings)
}
