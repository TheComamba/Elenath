use gui::Gui;
use iced::Sandbox;

mod error;
mod file_dialog;
mod gui;
mod model;

fn main() -> iced::Result {
    let window_settings = iced::window::Settings {
        size: (1820, 980),
        ..Default::default()
    };
    let settings = iced::settings::Settings {
        window: window_settings,
        antialiasing: true,
        ..Default::default()
    };
    Gui::run(settings)
}
