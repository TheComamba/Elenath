use super::{
    dialog::error::ErrorDialog, message::GuiMessage,
    shared_widgets::surface_and_top_view_shared_control,
    surface_view::surface_view_widget::SurfaceViewState,
    table_view::table_view_widget::TableViewState, top_view::top_view_widget::TopViewState, Gui,
};
use crate::model::new_celestial_system::solar_system;
use astro_utils::units::time::Time;
use iced::{
    widget::{canvas, Column, Row},
    Element, Sandbox,
};
use iced_aw::Modal;

pub(super) const PADDING: f32 = 10.0;
pub(super) const SMALL_COLUMN_WIDTH: f32 = 150.0;
pub(super) const BIG_COLUMN_WIDTH: f32 = 3.5 * SMALL_COLUMN_WIDTH;

#[derive(Debug, Clone)]
pub(crate) enum GuiMode {
    SurfaceView,
    TopView,
    TableView,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        let celestial_system = solar_system(false);
        Gui {
            opened_file: None,
            mode: GuiMode::SurfaceView,
            surface_view_state: SurfaceViewState::new(),
            top_view_state: TopViewState::new(),
            table_view_state: TableViewState::new(),
            time_since_epoch: Time::from_days(0.0),
            time_step: Time::from_days(1.0),
            celestial_system,
            selected_planet_name: String::new(),
            display_names: true,
            dialog: None,
        }
    }

    fn title(&self) -> String {
        String::from("Elenath - Imaginary Skies")
    }

    fn update(&mut self, message: Self::Message) {
        if let Err(e) = self.handle_message(message) {
            self.dialog = Some(Box::new(ErrorDialog::new(e)));
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Modal::new(
            self.main_view(),
            self.dialog.as_ref().map(|d| d.to_element()),
        )
        .on_esc(GuiMessage::DialogClosed)
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

impl<GuiMessage> canvas::Program<GuiMessage> for Gui {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::theme::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        match self.mode {
            GuiMode::SurfaceView => self.surface_view_state.canvas(
                renderer,
                bounds,
                &self.get_selected_planet(),
                &self.celestial_system,
                self.time_since_epoch,
                self.display_names,
            ),
            GuiMode::TopView => self.top_view_state.canvas(
                renderer,
                bounds,
                &self.get_selected_planet(),
                &self.celestial_system,
                self.time_since_epoch,
                self.display_names,
            ),
            _ => {
                println!("Invalid Gui state: Canvas Program is called from a Gui mode that does not have a canvas.");
                vec![]
            }
        }
    }
}

impl Gui {
    fn main_view(&self) -> Element<'_, GuiMessage> {
        let mut toprow = Row::new().push(Gui::gui_mode_tabs());
        if self.celestial_system.is_generated() {
            toprow = toprow
                .push(Gui::adding_buttons())
                .push(Gui::generated_system_file_buttons());
        } else {
            toprow = toprow.push(Gui::real_system_file_buttons());
        }
        toprow = toprow.padding(PADDING).spacing(PADDING);
        let mut col = Column::new().push(toprow);

        match self.mode {
            GuiMode::SurfaceView => {
                let control_row = Row::new()
                    .push(surface_and_top_view_shared_control(
                        &self.time_since_epoch,
                        &self.time_step,
                        self.celestial_system.get_planet_data(),
                        self.get_selected_planet_data(),
                        self.display_names,
                    ))
                    .push(self.surface_view_state.control_field());
                col = col.push(control_row).push(
                    canvas(self)
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill),
                )
            }
            GuiMode::TopView => {
                let control_row = Row::new()
                    .push(surface_and_top_view_shared_control(
                        &self.time_since_epoch,
                        &self.time_step,
                        self.celestial_system.get_planet_data(),
                        self.get_selected_planet_data(),
                        self.display_names,
                    ))
                    .push(self.top_view_state.control_field());
                col = col.push(control_row).push(
                    canvas(self)
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill),
                )
            }
            GuiMode::TableView => {
                col = col.push(self.table_view_state.table_view(
                    self.celestial_system.get_planet_data(),
                    self.celestial_system.get_star_data(),
                ))
            }
        }

        col.width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .spacing(PADDING)
            .into()
    }
}
