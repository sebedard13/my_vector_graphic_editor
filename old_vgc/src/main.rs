

use crate::coord::{Coord, CoordDS};
use crate::render::render_w;
use crate::vcg_struct::{Curve, File, Region, RGBA};

mod vcg_struct;
mod render;
mod coord;

fn main() {

   let (file, coord_ds) =  generate_exemple();


    match render_w(&file,&coord_ds, 512) {
        Ok(img) => { img.save_png("old_vgc/data/test1.png").expect("Able to save image"); }
        Err(e) => { eprintln!("{}", e) }
    }
}


fn generate_exemple() -> (File, CoordDS) {
    let mut coord_ds = CoordDS::new();

    let color = RGBA {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    let p0 = coord_ds.insert(Coord { x: 0.5, y: 0.0 });
    let mut vec_curve = Vec::default();
    let curve : Curve = {
        let c1 = coord_ds.insert(Coord { x: 0.6, y: 0.25 });
        let c2 = coord_ds.insert(Coord { x: 0.6, y: 0.25 });
        let p = coord_ds.insert(Coord { x: 0.5, y: 0.5 });
        Curve { c1, c2, p, }
    };
    vec_curve.push(curve);
    let curve : Curve = {
        let c1 = coord_ds.insert(Coord { x: 0.4, y: 0.75 });
        let c2 = coord_ds.insert(Coord { x: 0.4, y: 0.75 });
        let p = coord_ds.insert(Coord { x: 0.5, y: 1.0 });
        Curve { c1, c2, p, }
    };
    vec_curve.push(curve);
    let curve : Curve = {
        let c1 = coord_ds.insert(Coord { x: 1.0, y: 1.0 });
        let c2 = coord_ds.insert(Coord { x: 1.0, y: 1.0 });
        let p = coord_ds.insert(Coord { x: 1.0, y: 1.0 });
        Curve { c1, c2, p, }
    };
    vec_curve.push(curve);let curve : Curve = {
        let c1 = coord_ds.insert(Coord { x: 1.0, y: 0.0 });
        let c2 = coord_ds.insert(Coord { x: 1.0, y: 0.0 });
        let p = coord_ds.insert(Coord { x: 1.0, y: 0.0 });
        Curve { c1, c2, p, }
    };
    vec_curve.push(curve);


    let region1 = Region {
        start: p0,
        curves: vec_curve,
        color,
    };

    let mut vec_region = Vec::default();
    vec_region.push(region1);

    (File {
        version: 1,
        background: RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },

        ratio: 1.0,
        regions: vec_region,
    }, coord_ds)
}
