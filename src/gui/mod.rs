use self::topview::TopViewState;
use crate::model::celestial_body::CelestialBody;
use crate::model::{celestial_body::CelestialBodyData, example::sun};
use astro_utils::{
    units::{length::Length, time::Time},
    Float,
};
use iced::widget::canvas::{Path, Style};
use iced::Color;
use iced::{
    alignment::Horizontal,
    widget::{canvas, Button, Column, Container, PickList, Row, Text},
    Alignment, Sandbox,
};

mod topview;

pub(crate) struct Gui {
    time: Time,
    time_step: Time,
    topview_state: TopViewState,
    central_body_data: CelestialBodyData,
    celestial_bodies: Vec<CelestialBody>,
    selected_planet: Option<String>,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        let central_body_data = sun();
        let celestial_bodies = central_body_data.system(Time::from_days(0.0));
        Gui {
            time: Time::from_days(0.0),
            time_step: Time::from_days(1.0),
            topview_state: TopViewState::new(),
            central_body_data,
            celestial_bodies,
            selected_planet: None,
        }
    }

    fn title(&self) -> String {
        String::from("Elenath - Imaginary Skies")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            GuiMessage::UpdateTime(time) => {
                self.time = time;
                self.celestial_bodies = self.central_body_data.system(self.time);
                self.topview_state.redraw();
            }
            GuiMessage::UpdateTimeStep(time_step) => {
                self.time_step = time_step;
            }
            GuiMessage::UpdateLengthScale(m_per_px) => {
                self.topview_state.set_meter_per_pixel(m_per_px);
                self.topview_state.redraw();
            }
            GuiMessage::PlanetSelected(planet_name) => {
                self.selected_planet = Some(planet_name);
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Column::new()
            .push(self.topview_control_field())
            .push(
                canvas(self)
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill),
            )
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
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
        let background =
            self.topview_state
                .background_cache
                .draw(renderer, bounds.size(), |frame| {
                    let background = Path::rectangle(bounds.position(), bounds.size());
                    frame.fill(&background, Color::BLACK);
                });
        let bodies = self
            .topview_state
            .bodies_cache
            .draw(renderer, bounds.size(), |frame| {
                let bodies = Path::new(|path_builder| {
                    for body in self.celestial_bodies.iter() {
                        let x = body.get_position().x().as_meters()
                            / self.topview_state.meter_per_pixel;
                        let y = -body.get_position().y().as_meters()
                            / self.topview_state.meter_per_pixel; // y axis is inverted
                        let radius = 3.0;
                        let pos = frame.center() + iced::Vector::new(x as f32, y as f32);
                        path_builder.circle(pos, radius);

                        let mut name_widget = canvas::Text::default();
                        name_widget.color = Color::WHITE;
                        name_widget.content = body.get_name().to_string();
                        name_widget.position = pos;
                        frame.fill_text(name_widget);
                    }
                });
                frame.fill(&bodies, Color::WHITE);
            });
        let scale = self
            .topview_state
            .scale_cache
            .draw(renderer, bounds.size(), |frame| {
                const LENGTH: f32 = 200.0;
                let start_pos = bounds.position() + iced::Vector::new(50. as f32, 50. as f32);
                let middle_pos = start_pos + iced::Vector::new(LENGTH as f32 / 2., 0.0 as f32);
                let end_pos = start_pos + iced::Vector::new(LENGTH as f32, 0.0 as f32);
                let delimitor_vec = iced::Vector::new(0.0 as f32, 5. as f32);

                let scale = Path::new(|path_builder| {
                    path_builder.move_to(start_pos + delimitor_vec);
                    path_builder.line_to(start_pos - delimitor_vec);
                    path_builder.move_to(start_pos);
                    path_builder.line_to(end_pos);
                    path_builder.move_to(end_pos + delimitor_vec);
                    path_builder.line_to(end_pos - delimitor_vec);
                });
                let mut stroke = canvas::Stroke::default();
                stroke.style = Style::Solid(Color::WHITE);

                frame.stroke(&scale, stroke);

                let mut text = canvas::Text::default();
                text.color = Color::WHITE;
                text.content = format!(
                    "{}",
                    Length::from_meters(LENGTH * self.topview_state.meter_per_pixel)
                );
                text.position = middle_pos;
                text.horizontal_alignment = Horizontal::Center;
                frame.fill_text(text);
            });
        vec![background, bodies, scale]
    }
}

impl Gui {
    fn topview_control_field(&self) -> iced::Element<'_, GuiMessage> {
        let time_control_field = self.control_field(
            "Time:",
            format!("{}", self.time),
            GuiMessage::UpdateTime(self.time - self.time_step),
            GuiMessage::UpdateTime(self.time + self.time_step),
        );
        let time_step_control_field = self.control_field(
            "Time step:",
            format!("{}", self.time_step),
            GuiMessage::UpdateTimeStep(self.time_step / 2.),
            GuiMessage::UpdateTimeStep(self.time_step * 2.),
        );
        let m_per_px = self.topview_state.get_meter_per_pixel();
        let length_scale_control_field = self.control_field(
            "Length per 100px:",
            format!("{}", Length::from_meters(100. * m_per_px)),
            GuiMessage::UpdateLengthScale(m_per_px / 2.),
            GuiMessage::UpdateLengthScale(m_per_px * 2.),
        );
        let planet_picker = self.planet_picker();
        Column::new()
            .push(time_control_field)
            .push(time_step_control_field)
            .push(length_scale_control_field)
            .push(planet_picker)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }

    fn control_field<'a>(
        &self,
        label: &'a str,
        value: String,
        decrease: GuiMessage,
        increase: GuiMessage,
    ) -> Row<'a, GuiMessage> {
        let label = Container::new(Text::new(label))
            .align_x(Horizontal::Center)
            .width(iced::Length::Fixed(150.));
        let decrease_button = Container::new(Button::new(Text::new("<<")).on_press(decrease))
            .align_x(Horizontal::Center)
            .width(iced::Length::Fixed(50.));
        let value = Container::new(Text::new(value))
            .width(iced::Length::Fixed(100.))
            .align_x(Horizontal::Center);
        let increase_button = Container::new(Button::new(Text::new(">>")).on_press(increase))
            .align_x(Horizontal::Center)
            .width(iced::Length::Fixed(50.));
        Row::new()
            .push(label)
            .push(decrease_button)
            .push(value)
            .push(increase_button)
            .align_items(Alignment::Center)
    }

    fn planet_picker(&self) -> iced::Element<'_, GuiMessage> {
        let text = Text::new("Planet picker:").width(150.);
        let options: Vec<String> = self
            .celestial_bodies
            .iter()
            .map(|body| body.get_name().to_string())
            .collect();
        let pick_list = PickList::new(
            options,
            self.selected_planet.clone(),
            GuiMessage::PlanetSelected,
        )
        .width(200.);
        Row::new()
            .push(text)
            .push(pick_list)
            .align_items(Alignment::Center)
            .into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum GuiMessage {
    UpdateTime(Time),
    UpdateTimeStep(Time),
    UpdateLengthScale(Float),
    PlanetSelected(String),
}
