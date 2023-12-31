use super::{Gui, GuiMessage};
use iced::{
    widget::{
        canvas::{self, Path},
        Column,
    },
    Alignment, Color,
};

pub(super) struct SurfaceViewState {
    pub(super) background_cache: canvas::Cache,
}

impl SurfaceViewState {
    pub(super) fn new() -> Self {
        SurfaceViewState {
            background_cache: canvas::Cache::default(),
        }
    }

    pub(super) fn redraw(&mut self) {
        self.background_cache.clear();
    }
}

impl Gui {
    pub(super) fn surface_view_control_field(&self) -> iced::Element<'_, GuiMessage> {
        let planet_picker = self.planet_picker();
        Column::new()
            .push(self.time_control_fields())
            .push(planet_picker)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }

    pub(super) fn surface_view_canvas(
        &self,
        renderer: &iced::Renderer,
        bounds: iced::Rectangle,
    ) -> Vec<canvas::Geometry> {
        let background =
            self.surface_view_state
                .background_cache
                .draw(renderer, bounds.size(), |frame| {
                    let radius = bounds.size().width.min(bounds.size().height) / 2.;
                    let background = Path::circle(frame.center(), radius);
                    frame.fill(&background, Color::BLACK);
                });
        vec![background]
    }
}
