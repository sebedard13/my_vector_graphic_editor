use crate::coord::CoordDS;
use crate::Vgc;
use iced::widget::canvas::Frame;
use tiny_skia::Pixmap;

pub fn render_w(canvas: &Vgc, w: u32) -> Result<Pixmap, String> {
    let h = ((w as f64) * (1.0 / canvas.ratio)) as u32;
    let scaled_coord_ds = canvas.coord_ds.scale(w as f32, h as f32);
    render(canvas, &scaled_coord_ds, w, h)
}

fn render(canvas: &Vgc, coord_ds: &CoordDS, w: u32, h: u32) -> Result<Pixmap, String> {
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
            let coord_start = coord_ds.get(&region.start);
            pb.move_to(coord_start.x, coord_start.y);
            for i_curve in 0..region.curves.len() {
                pb.cubic_to(
                    coord_ds.get(&region.curves[i_curve].cp0).x,
                    coord_ds.get(&region.curves[i_curve].cp0).y,
                    coord_ds.get(&region.curves[i_curve].cp1).x,
                    coord_ds.get(&region.curves[i_curve].cp1).y,
                    coord_ds.get(&region.curves[i_curve].p1).x,
                    coord_ds.get(&region.curves[i_curve].p1).y,
                );
            }
            pb.close();
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

    let h = 1.0 / canvas.ratio;
    let coord_ds = canvas.coord_ds.scale(1.0, h as f32);

    for i_region in 0..canvas.shapes.len() {
        let region = &canvas.shapes[i_region];

        let fill = Fill::from(Color::from_rgba8(
            region.color.r,
            region.color.g,
            region.color.b,
            region.color.a as f32 / 255.0,
        ));

        let path = &Path::new(|builder| {
            let coord_start = coord_ds.get(&region.start);
            builder.move_to(Point::new(coord_start.x, coord_start.y));

            for i_curve in 0..region.curves.len() {
                builder.bezier_curve_to(
                    Point::new(
                        coord_ds.get(&region.curves[i_curve].cp0).x,
                        coord_ds.get(&region.curves[i_curve].cp0).y,
                    ),
                    Point::new(
                        coord_ds.get(&region.curves[i_curve].cp1).x,
                        coord_ds.get(&region.curves[i_curve].cp1).y,
                    ),
                    Point::new(
                        coord_ds.get(&region.curves[i_curve].p1).x,
                        coord_ds.get(&region.curves[i_curve].p1).y,
                    ),
                );
            }

            builder.close();
        });

        frame.fill(path, fill)
    }
}
