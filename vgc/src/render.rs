use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};
use crate::Canvas;
use crate::coord::CoordDS;


pub fn render_w(canvas: &Canvas, w: u32) -> Result<Pixmap, String> {
    let h = ((w as f64) * (1.0 / canvas.ratio)) as u32;
    let scaled_coord_ds = canvas.coord_ds.scale(w as f32, h as f32);
    render(canvas, &scaled_coord_ds, w, h)
}


fn render(canvas: &Canvas, coord_ds: &CoordDS, w: u32, h: u32) -> Result<Pixmap, String> {
    let mut image = Pixmap::new(w, h).expect("Valid Size");


    for i_region in 0..canvas.shapes.len() {
        let region = &canvas.shapes[i_region];

        let mut paint = Paint::default();
        paint.set_color_rgba8(region.color.r, region.color.g, region.color.b, region.color.a);
        paint.anti_alias = true;


        let path = {
            let mut pb = PathBuilder::new();
            let coord_start = coord_ds.get(&region.start);
            pb.move_to(coord_start.x, coord_start.y);
            for i_curve in 0..region.curves.len() {
                pb.cubic_to(coord_ds.get(&region.curves[i_curve].cp0).x, coord_ds.get(&region.curves[i_curve].cp0).y,
                            coord_ds.get(&region.curves[i_curve].cp1).x, coord_ds.get(&region.curves[i_curve].cp1).y,
                            coord_ds.get(&region.curves[i_curve].p1).x, coord_ds.get(&region.curves[i_curve].p1).y,
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