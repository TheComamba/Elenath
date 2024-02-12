use super::{
    dialog::error::ErrorDialog, message::GuiMessage,
    shared_widgets::surface_and_top_view_shared_control,
    surface_view::surface_view_widget::SurfaceViewState,
    table_view::table_view_widget::TableViewState, top_view::top_view_widget::TopViewState, Gui,
};
use iced::{
    widget::{canvas, Column, Container, Row, Text},
    Element, Length, Sandbox,
};
use iced_aw::Modal;
use simple_si_units::base::Time;

pub(super) const PADDING: f32 = 10.0;
pub(super) const SMALL_COLUMN_WIDTH: f32 = 150.0;
pub(super) const BIG_COLUMN_WIDTH: f32 = 3.5 * SMALL_COLUMN_WIDTH;

#[derive(Debug, Clone)]
pub(crate) enum GuiViewMode {
    Surface,
    Top,
    Table,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        Gui {
            opened_file: None,
            mode: GuiViewMode::Surface,
            surface_view_state: SurfaceViewState::new(),
            top_view_state: TopViewState::new(),
            table_view_state: TableViewState::new(),
            time_since_epoch: Time::from_days(0.0),
            time_step: Time::from_days(1.0),
            celestial_system: None,
            selected_planet_name: String::new(),
            display_names: true,
            display_constellations: false,
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
            GuiViewMode::Surface => self.surface_view_state.canvas(
                renderer,
                bounds,
                &self.get_selected_planet(),
                &self.celestial_system,
                self.time_since_epoch,
                self.display_names,
            ),
            GuiViewMode::Top => self.top_view_state.canvas(
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
        let toprow = Row::new()
            .push(Gui::gui_mode_tabs())
            .push(Container::new(Text::new("")).width(Length::Fill))
            .push(Gui::file_buttons(self.celestial_system.is_some()))
            .padding(PADDING)
            .spacing(PADDING);
        let mut col = Column::new().push(toprow);

        match self.mode {
            GuiViewMode::Surface => {
                let control_row = Row::new()
                    .push(surface_and_top_view_shared_control(
                        &self.time_since_epoch,
                        &self.time_step,
                        self.get_planet_data(),
                        self.get_selected_planet_data(),
                        self.display_names,
                        self.display_constellations,
                    ))
                    .push(self.surface_view_state.control_field());
                col = col.push(control_row).push(
                    canvas(self)
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill),
                )
            }
            GuiViewMode::Top => {
                let control_row = Row::new()
                    .push(surface_and_top_view_shared_control(
                        &self.time_since_epoch,
                        &self.time_step,
                        self.get_planet_data(),
                        self.get_selected_planet_data(),
                        self.display_names,
                        self.display_constellations,
                    ))
                    .push(self.top_view_state.control_field());
                col = col.push(control_row).push(
                    canvas(self)
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill),
                )
            }
            GuiViewMode::Table => {
                let (planets, stars) = match &self.celestial_system {
                    Some(system) => {
                        let planets = system.get_planets_at_time(self.time_since_epoch);
                        let stars = system.get_stars().into_iter().collect();
                        (planets, stars)
                    }
                    None => (Vec::new(), Vec::new()),
                };
                col = col.push(self.table_view_state.table_view(
                    planets,
                    stars,
                    self.celestial_system.is_some(),
                ));
            }
        }

        col.width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .spacing(PADDING)
            .into()
    }
}
