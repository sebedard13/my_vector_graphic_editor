use serde::{Deserialize, Serialize};
use crate::coord::{Coord, CoordDS, insert_shape};
use crate::instructions::{CurveInstruction, ShapeInstruction};
use crate::vcg_struct::{Shape, RGBA};

mod vcg_struct;
mod render;
mod coord;
mod instructions;


/*Todo API
- Coord::move
- Region::add_coord
- move_coord(index
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Canvas {
    ratio: f64,
    background: RGBA,
    shapes: Vec<Shape>,
    coord_ds: CoordDS,
}

impl Canvas {
    pub fn new(ratio: f64, background: RGBA) -> Canvas {
        return Canvas { ratio, background, shapes: Vec::new(), coord_ds : CoordDS::new() };
    }

    pub fn from_byte(byte: &[u8]) -> Result<Canvas, String> {
        postcard::from_bytes(byte).map_err(|e| e.to_string())
    }

    pub fn to_byte(&self) -> Result<Vec<u8>, String> {
        postcard::to_allocvec(self).map_err(|e| e.to_string())
    }

    pub fn add_shape(&mut self, shape_instruction: ShapeInstruction) {

        let shape = insert_shape(&mut self.coord_ds, shape_instruction);
         self.shapes.push(shape);
        //Todo: update coord_ds

        //Todo: refactor and remove colliding shape?
    }

    pub fn list_coord(&self)->Vec<&Coord>{
        return self.coord_ds.array.iter().filter_map(|op_c| {
            return match op_c {
                Some(c) => {
                    Some(c)
                }
                None => { None }
            };
        }).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_render_file() {
        let (file, coord_ds) =  generate_exemple();

        match render_w(&file, 512) {
            Ok(img) => { /*img.save_png("data/test1.png").expect("Able to save image");*/ }
            Err(e) => { panic!(e) }
        }
    }
}


fn generate_exemple() -> Canvas {
    let color = RGBA {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    let mut canvas = Canvas::new(1.0, color);

    let p0 = Coord { x: 0.5, y: 0.0 };
    let mut vec_curve = Vec::default();
    let curve : CurveInstruction = {
        let c1 =Coord { x: 0.6, y: 0.25 };
        let c2 = Coord { x: 0.6, y: 0.25 };
        let p = Coord { x: 0.5, y: 0.5 };
        CurveInstruction  { c1, c2, p, }
    };
    vec_curve.push(curve);
    let curve : CurveInstruction = {
        let c1 = Coord { x: 0.4, y: 0.75 };
        let c2 = Coord { x: 0.4, y: 0.75 };
        let p = Coord { x: 0.5, y: 1.0 };
        CurveInstruction  { c1, c2, p, }
    };
    vec_curve.push(curve);
    let curve : CurveInstruction = {
        let c1 = Coord { x: 1.0, y: 1.0 };
        let c2 = Coord { x: 1.0, y: 1.0 };
        let p = Coord { x: 1.0, y: 1.0 };
        CurveInstruction  { c1, c2, p, }
    };
    vec_curve.push(curve);let curve : CurveInstruction = {
        let c1 = Coord { x: 1.0, y: 0.0 };
        let c2 = Coord { x: 1.0, y: 0.0 };
        let p = Coord { x: 1.0, y: 0.0 };
        CurveInstruction  { c1, c2, p, }
    };
    vec_curve.push(curve);


    canvas.add_shape(ShapeInstruction {
        start: p0,
        curves: vec_curve,
        color: RGBA {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        },
    });

   return canvas;
}
