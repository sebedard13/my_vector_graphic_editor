use iced::{widget::canvas::{Frame, Text, Fill}, Point, Color, alignment, Vector, Size};

use super::Scene;




pub fn draw(scene: &Scene, frame: &mut Frame, cursor_pos: Point) {
    let pos = scene.camera.project_in_canvas(cursor_pos);


    if let Some(pos) = pos {
        let text = Text {
            color: Color::WHITE,
            size: 14.0,
            position: Point::new(frame.width(), frame.height()),
            horizontal_alignment: alignment::Horizontal::Right,
            vertical_alignment: alignment::Vertical::Bottom,
            ..Text::default()
        };

        let content = format!(
            "({:.4}, {:.4}) {:.0}%",
            pos.x,
            pos.y,
            scene.camera.scaling * 100.0
        );
    
        let overlay_width = content.len() as f32 * 6.58;
        let overlay_height = 16.0;
    
        frame.fill_rectangle(
            text.position - Vector::new(overlay_width, overlay_height),
            Size::new(overlay_width, overlay_height),
            Fill::from(Color::from_rgba8(0x00, 0x00, 0x00, 0.8)),
        );
    
        frame.fill_text(Text {
            content,
            position: text.position - Vector::new(0.0, 0.0),
            ..text
        });
    }

    
}