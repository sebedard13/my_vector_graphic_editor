use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};
use crate::coord::CoordDS;

use crate::vcg_struct::File;


pub fn render_w(file: &File, coord_ds: &CoordDS, w: u32) -> Result<Pixmap, String> {
    let h = ((w as f64) * (1.0 / file.ratio)) as u32;
    let scaled_coord_ds = coord_ds.scale(w as f32,h as f32);
    return render(file, &scaled_coord_ds, w, h);
}


fn render(file: &File,coord_ds: &CoordDS, w: u32, h: u32) -> Result<Pixmap, String> {
    let mut image = Pixmap::new(w, h).expect("Valid Size");


    for i_region in 0..file.regions.len() {
        let region = &file.regions[i_region];

        let mut paint = Paint::default();
        paint.set_color_rgba8(region.color.r, region.color.g, region.color.b, region.color.a);
        paint.anti_alias = true;


        let path = {
            let mut pb = PathBuilder::new();
            let coord_start = coord_ds.get(&region.start);
            pb.move_to(coord_start.x, coord_start.y);
            for i_curve in 0..region.curves.len() {
                pb.cubic_to(coord_ds.get(&region.curves[i_curve].c1).x, coord_ds.get(&region.curves[i_curve].c1).y,
                            coord_ds.get(&region.curves[i_curve].c2).x, coord_ds.get(&region.curves[i_curve].c2).y,
                            coord_ds.get(&region.curves[i_curve].p).x, coord_ds.get(&region.curves[i_curve].p).y,
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