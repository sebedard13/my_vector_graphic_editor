#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize};

use crate::coord::{Coord, CoordDS, insert_curve, insert_shape};
use crate::instructions::{AddCurve, CoordWithIndex, CurveInstruction, ShapeInstruction};
use crate::vcg_struct::{Rgba, Shape};

mod vcg_struct;
mod render;
mod coord;
mod instructions;

#[derive(Debug, Serialize, Deserialize)]
pub struct Canvas {
    ratio: f64,
    background: Rgba,
    shapes: Vec<Shape>,
    coord_ds: CoordDS,
}

impl Canvas {
    pub fn new(ratio: f64, background: Rgba) -> Canvas {
        Canvas { ratio, background, shapes: Vec::new(), coord_ds : CoordDS::new() }
    }

    pub fn from_byte(byte: &[u8]) -> Result<Canvas, String> {
        postcard::from_bytes(byte).map_err(|e| e.to_string())
    }

    pub fn to_byte(&self) -> Result<Vec<u8>, String> {
        postcard::to_allocvec(self).map_err(|e| e.to_string())
    }

    pub fn add_shape(&mut self, shape_instruction: ShapeInstruction) -> usize {

        let shape = insert_shape(&mut self.coord_ds, shape_instruction);
        self.shapes.push(shape);
        self.shapes.len() - 1
        //Todo: refactor and remove colliding shape?<
    }


    pub fn list_coord(&self) -> Vec<CoordWithIndex> {
        let mut vec = Vec::new();
        for i in 0..self.coord_ds.array.len() {
            match &self.coord_ds.array[i] {
                Some(c) => {
                    vec.push(CoordWithIndex { coord: c, i });
                }
                None => { }
            }
        }
        vec
    }

    pub fn move_coord(&mut self, index: usize, x: f32, y: f32) {
        self.coord_ds.modify(index, Coord { x, y })
    }

    pub fn add_coord(&mut self, add_curve_coord: AddCurve) {
        let curve = insert_curve(&mut self.coord_ds, add_curve_coord.curve);

       self.shapes[add_curve_coord.index_shape].add_coord(&mut self.coord_ds, curve, add_curve_coord.index_curve);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_render_file() {
        let canvas = generate_exemple();

       assert_eq!(canvas.shapes[0].to_path(&canvas.coord_ds),"M 0.5 0 C 0.6 0.25 0.6 0.25 0.5 0.5 C 0.4 0.75 0.4 0.75 0.5 1 C 1 1 1 1 1 1 C 1 0 1 0 1 0 C 1 0 0.5 0 0.5 0 Z");
    }
}


fn generate_exemple() -> Canvas {
    let color = Rgba {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    let mut canvas = Canvas::new(1.0, color);

    let p0 = Coord { x: 0.5, y: 0.0 };

    let shape_index = canvas.add_shape(ShapeInstruction {
        start: p0,
        curves: Vec::default(),
        color: Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },
    });

    canvas.shapes[shape_index].separate_handle(&mut canvas.coord_ds,0);
    println!("Coords : {:?}", canvas.list_coord());

    canvas.coord_ds.modify(1,Coord { x: 0.5, y: 0.0 });
    canvas.coord_ds.modify(2,Coord { x: 0.6, y: 0.25 });

    let curve : CurveInstruction = {
        let c1 = Coord { x: 0.6, y: 0.25 };
        let c2 = Coord { x: 0.4, y: 0.75 };
        let p = Coord { x: 0.5, y: 0.5 };
        CurveInstruction  { c1, c2, p, }
    };
    canvas.add_coord(AddCurve { curve, index_shape: shape_index, index_curve: 0 });
    let curve : CurveInstruction = {
        let c1 = Coord { x: 0.4, y: 0.75 };
        let c2 = Coord { x: 1.0, y: 1.0 };
        let p = Coord { x: 0.5, y: 1.0 };
        CurveInstruction  { c1, c2, p, }
    };
    canvas.add_coord(AddCurve { curve, index_shape: shape_index, index_curve: 1 });
    let curve : CurveInstruction = {
        let c1 = Coord { x: 1.0, y: 1.0 };
        let c2 = Coord { x: 1.0, y: 0.0 };
        let p = Coord { x: 1.0, y: 1.0 };
        CurveInstruction  { c1, c2, p, }
    };
    canvas.add_coord(AddCurve { curve, index_shape: shape_index, index_curve: 2 });
    let curve: CurveInstruction = {
        let c1 = Coord { x: 1.0, y: 0.0 };
        let c2 = Coord { x: 1.0, y: 0.0 };
        let p = Coord { x: 1.0, y: 0.0 };
        CurveInstruction  { c1, c2, p, }
    };
    canvas.add_coord(AddCurve { curve, index_shape: shape_index, index_curve: 3 });

    canvas
}
