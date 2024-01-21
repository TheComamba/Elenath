use iced::{
    widget::canvas::{self, Path},
    Color, Point, Rectangle,
};

pub(super) fn draw_background(bounds: iced::Rectangle, frame: &mut canvas::Frame) {
    let background = Path::rectangle(Point::ORIGIN, bounds.size());
    frame.fill(&background, Color::BLACK);
}

pub(super) fn draw_name(name: &str, color: Color, body_center: Point, frame: &mut canvas::Frame) {
    const ORDINATE_OFFSET: f32 = 10.;
    if name.is_empty() {
        return;
    }
    let mut name_widget = canvas::Text::default();
    name_widget.color = color;
    name_widget.content = name.to_string();
    name_widget.position = body_center + iced::Vector::new(ORDINATE_OFFSET, ORDINATE_OFFSET);
    frame.fill_text(name_widget);
}

/*
 * Iced's bound.contains is currently broken:
 * https://github.com/TheComamba/IcedPlayground/blob/main/canvas_coordinates/src/main.rs
 */
pub(super) fn contains_workaround(bounds: &Rectangle, point: Point) -> bool {
    return point.x >= 0. && point.x <= bounds.width && point.y >= 0. && point.y <= bounds.height;
}
