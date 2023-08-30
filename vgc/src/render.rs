use crate::Vgc;
use iced::widget::canvas::Frame;
use tiny_skia::Pixmap;

pub fn render_w(canvas: &Vgc, w: u32) -> Result<Pixmap, String> {
    let h = ((w as f64) * (1.0 / canvas.ratio)) as u32;
    render(canvas, w, h)
}

fn render(canvas: &Vgc, w: u32, h: u32) -> Result<Pixmap, String> {
    use tiny_skia::{FillRule, Paint, PathBuilder, Transform};
    let mut image = Pixmap::new(w, h).expect("Valid Size");

    for i_region in 0..canvas.shapes.len() {
        let region = &canvas.shapes[i_region];

        let mut paint = Paint::default();
        paint.set_color_rgba8(
            region.color.r,
            region.color.g,
            region.color.b,
            region.color.a,
        );
        paint.anti_alias = true;

        let path = {
            let mut pb = PathBuilder::new();
            let coord_start = &region.start.borrow().scale(w, h);
            pb.move_to(coord_start.x, coord_start.y);
            for i_curve in 0..region.curves.len() {
                let cp0 = region.curves[i_curve].cp0.borrow().scale(w, h);
                let cp1 = region.curves[i_curve].cp1.borrow().scale(w, h);
                let p1 = region.curves[i_curve].p1.borrow().scale(w, h);

                pb.cubic_to(cp0.x, cp0.y, cp1.x, cp1.y, p1.x, p1.y);
            }
            pb.finish().unwrap()
        };

        image.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }

    Ok(image)
}

pub fn frame_render(canvas: &Vgc, frame: &mut Frame) {
    use iced::widget::canvas::{Fill, Path};
    use iced::{Color, Point};

    for i_region in 0..canvas.shapes.len() {
        let region = &canvas.shapes[i_region];

        let fill = Fill::from(Color::from_rgba8(
            region.color.r,
            region.color.g,
            region.color.b,
            region.color.a as f32 / 255.0,
        ));

        let path = &Path::new(|builder| {
            let coord_start = &region.start.borrow();
            builder.move_to(Point::new(coord_start.x, coord_start.y));

            for i_curve in 0..region.curves.len() {
                builder.bezier_curve_to(
                    Point::new(
                        region.curves[i_curve].cp0.borrow().x,
                        region.curves[i_curve].cp0.borrow().y,
                    ),
                    Point::new(
                        region.curves[i_curve].cp1.borrow().x,
                        region.curves[i_curve].cp1.borrow().y,
                    ),
                    Point::new(
                        region.curves[i_curve].p1.borrow().x,
                        region.curves[i_curve].p1.borrow().y,
                    ),
                );
            }
        });

        frame.fill(path, fill)
    }
}
