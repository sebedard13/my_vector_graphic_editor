#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

use coord::CoordIndex;
use serde::{Deserialize, Serialize};

use crate::coord::{insert_curve, insert_shape, Coord, CoordDS};
use crate::instructions::{AddCurve, CoordWithIndex, CoordInstruction, ShapeInstruction};
use crate::vgc_struct::{Rgba, Shape};
use iced::widget::canvas::Frame;

mod coord;
mod instructions;
pub mod render;
mod vgc_struct;
mod curve;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vgc {
    pub ratio: f64, //width/height 16/9
    pub background: Rgba,
    shapes: Vec<Shape>,
    coord_ds: CoordDS,
}

impl Vgc {
    pub fn new(ratio: f64, background: Rgba) -> Vgc {
        Vgc {
            ratio,
            background,
            shapes: Vec::new(),
            coord_ds: CoordDS::new(),
        }
    }

    pub fn create_shape(&mut self, start: Coord, color: Rgba) -> usize {
        let shape = Shape {
            start: self.coord_ds.insert(start),
            curves: Vec::new(),
            color,
        };
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    pub fn from_byte(byte: &[u8]) -> Result<Vgc, String> {
        postcard::from_bytes(byte).map_err(|e| e.to_string())
    }

    pub fn to_byte(&self) -> Result<Vec<u8>, String> {
        postcard::to_allocvec(self).map_err(|e| e.to_string())
    }

    pub fn add_shape(&mut self, shape_instruction: ShapeInstruction) -> usize {
        let shape = insert_shape(&mut self.coord_ds, shape_instruction);
        self.shapes.push(shape);
        self.shapes.len() - 1
        //TODO: refactor and remove colliding shape?<
    }

    pub fn list_coord(&self) -> Vec<CoordWithIndex> {
        let mut vec = Vec::new();
        for i in 0..self.coord_ds.array.len() {
            match &self.coord_ds.array[i] {
                Some(c) => {
                    vec.push(CoordWithIndex { coord: c, i });
                }
                None => {}
            }
        }
        vec
    }

    pub fn move_coord(&mut self, index: usize, x: f32, y: f32) {
        self.coord_ds.modify(index, Coord { x, y })
    }

    pub fn add_coord(&mut self, add_curve_coord: AddCurve) {
        let curve = insert_curve(&mut self.coord_ds, add_curve_coord.curve);

        self.shapes[add_curve_coord.index_shape].add_coord(
            &mut self.coord_ds,
            curve,
            add_curve_coord.index_curve,
        );
    }

    pub fn push_coord(&mut self, index_shape: usize, cp0 : Coord, cp1 : Coord, p1 : Coord) {
       
        self.shapes[index_shape].push_coord(
            &mut self.coord_ds,
            cp0, cp1, p1
        );
    }

    pub fn frame_render(&self, frame: &mut Frame) {
        render::frame_render(self, frame);
    }

    pub fn optimize_coord(&mut self) {
        // TODO Maybe Coord implement Hash and be use in HashMap directly
        let mut coord_map: HashMap<u64, CoordIndex> = HashMap::new();

        let mut coord_ds = CoordDS::new();

        
        for shape in self.shapes.iter_mut() {
            shape.start = optimize_coord_index(&shape.start, &mut coord_map, &mut coord_ds, &self.coord_ds);
            for curve in shape.curves.iter_mut() {
                curve.cp0 = optimize_coord_index(&curve.cp0, &mut coord_map, &mut coord_ds, &self.coord_ds);
                curve.cp1 = optimize_coord_index(&curve.cp1, &mut coord_map, &mut coord_ds, &self.coord_ds);
                curve.p1 = optimize_coord_index(&curve.p1, &mut coord_map, &mut coord_ds, &self.coord_ds);
            }
        }

        self.coord_ds = coord_ds;
    }

    pub fn visit(&self, f: &mut dyn FnMut(usize, CoordType)) {
        for (shape_index, shape) in self.shapes.iter().enumerate() {
            f(shape_index, CoordType::Start(self.coord_ds.get(&shape.start)));
            for (curve_index, curve) in shape.curves.iter().enumerate() {
                f(shape_index, CoordType::Cp0(curve_index, self.coord_ds.get(&curve.cp0)));
                f(shape_index, CoordType::Cp1(curve_index, self.coord_ds.get(&curve.cp1)));
                f(shape_index, CoordType::P1(curve_index, self.coord_ds.get(&curve.p1)));
            }
        }
    }

    pub fn visit_vec(&self)->Vec<(usize, CoordType)>{
        let mut vec = Vec::new();
        for (shape_index, shape) in self.shapes.iter().enumerate() {
            vec.push((shape_index, CoordType::Start(self.coord_ds.get(&shape.start))));
            for (curve_index, curve) in shape.curves.iter().enumerate() {
                vec.push((shape_index, CoordType::Cp0(curve_index, self.coord_ds.get(&curve.cp0))));
                vec.push((shape_index, CoordType::Cp1(curve_index, self.coord_ds.get(&curve.cp1))));
                vec.push((shape_index, CoordType::P1(curve_index, self.coord_ds.get(&curve.p1))));
            }
        }
        
        vec
    }

    pub fn get_cp_of_shape<'a>(&'a self, shape_index: usize) -> Vec<CoordType<'a>> {
        let mut vec = Vec::new();
        for (curve_index, curve) in  self.shapes[shape_index].curves.iter().enumerate() {
            vec.push(CoordType::Cp0(curve_index, self.coord_ds.get(&curve.cp0)));
            vec.push(CoordType::Cp1(curve_index, self.coord_ds.get(&curve.cp1)));
        }
        vec
    }

    pub fn get_p_of_shape<'a>(&'a self, shape_index: usize) -> Vec< &'a Coord> {
        let mut vec = Vec::new();
        vec.push(self.coord_ds.get(&self.shapes[shape_index].start));
        for curve in  self.shapes[shape_index].curves.iter() {
            vec.push(self.coord_ds.get(&curve.p1));
        }
        vec
    }

    pub fn get_coords_of_shape<'a>(&'a self, shape_index: usize) -> Vec< &'a Coord> {
        let mut vec = Vec::new();
        vec.push(self.coord_ds.get(&self.shapes[shape_index].start));
        for curve in  self.shapes[shape_index].curves.iter(){
            vec.push(self.coord_ds.get(&curve.cp0));
            vec.push(self.coord_ds.get(&curve.cp1));
            vec.push(self.coord_ds.get(&curve.p1));
        }
        vec
    }

    pub fn toggle_separate_join_handle(&mut self, shape_index: usize, curve_index: usize) {
        self.shapes[shape_index].toggle_separate_join_handle(&mut self.coord_ds, curve_index);
    }

    pub fn debug_string(&self)->String{
        let mut string = "".to_string();
        for shape in &self.shapes{
            string.push_str(&shape.to_path(&self.coord_ds));
            string.push_str("\n");
        }
        string
    }

    pub fn set_shape_background(&mut self, shape_index: usize, color: Rgba) {
        self.shapes[shape_index].color = color;
    }

    pub fn get_closest_coord_on_shape(&self, shape_index: usize, x: f32, y: f32) -> Coord {
        self.shapes[shape_index].closest_coord_on(&self.coord_ds, Coord {x, y})
    }
}

pub enum CoordType<'a> {
    Start(&'a Coord),
    /// Curve index, coord
    Cp0(usize, &'a Coord),
    /// Curve index, coord
    Cp1(usize, &'a Coord),
    /// Curve index, coord
    P1(usize, &'a Coord),
}


fn optimize_coord_index(cp1:&CoordIndex, coord_map: &mut HashMap<u64, CoordIndex>, new_coord_ds: &mut CoordDS,coord_ds: & CoordDS) -> CoordIndex{
    let coord = coord_ds.get(cp1);
    let key = coord.key();
    match coord_map.get(&key) {
        Some(coord_index) => {
            coord_index.clone()
        }
        None => {
            let coord_index = new_coord_ds.insert(coord.clone());
            coord_map.insert(key, coord_index.clone());
            coord_index
        }
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

    #[test]
    fn get_closest_coord_on_shape_triangle() {
        let canvas = generate_from_line(&[
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 0.0, y: 1.0 },
            Coord { x: 1.0, y: 1.0 },
        ]);

        let coord = canvas.get_closest_coord_on_shape(0, 1.0, 0.0);
        assert_eq!(coord.x, 0.5);
        assert_eq!(coord.y, 0.5);
    }

    #[test]
    fn genreate_from_push() {
        let canvas = generate_from_push(&[
            Coord { x: 0.0, y: 0.0 },
           
            Coord { x: -0.46193975, y: 0.19134173 },
            Coord { x: 0.0, y: 1.0 },
            Coord { x: 0.0, y: 1.0 },
            
            Coord { x: 0.0, y: 1.0 },
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 1.0, y: 1.0 },
            
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 0.46193975, y: -0.19134173 },
            Coord { x: 0.0, y: 0.0 },
        ]);
       
        assert_eq!(canvas.debug_string(), "M 0 0 C -0.46193975 0.19134173 0 1 0 1 C 0 1 1 1 1 1 C 1 1 0.46193975 -0.19134173 0 0 Z\n");
    }

    
}

pub fn generate_exemple() -> Vgc {
    
    let color = Rgba {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    let mut canvas = Vgc::new(16.0 / 9.0, color);

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

    canvas.shapes[shape_index].separate_handle(&mut canvas.coord_ds, 0);

    canvas.coord_ds.modify(1, Coord { x: 0.5, y: 0.0 });
    canvas.coord_ds.modify(2, Coord { x: 0.6, y: 0.25 });

    let curve: CoordInstruction = {
        let c1 = Coord { x: 0.6, y: 0.25 };
        let c2 = Coord { x: 0.4, y: 0.75 };
        let p = Coord { x: 0.5, y: 0.5 };
        CoordInstruction { c1, c2, p }
    };
    canvas.add_coord(AddCurve {
        curve,
        index_shape: shape_index,
        index_curve: 0,
    });
    let curve: CoordInstruction = {
        let c1 = Coord { x: 0.4, y: 0.75 };
        let c2 = Coord { x: 1.0, y: 1.0 };
        let p = Coord { x: 0.5, y: 1.0 };
        CoordInstruction { c1, c2, p }
    };
    canvas.add_coord(AddCurve {
        curve,
        index_shape: shape_index,
        index_curve: 1,
    });
    let curve: CoordInstruction = {
        let c1 = Coord { x: 1.0, y: 1.0 };
        let c2 = Coord { x: 1.0, y: 0.0 };
        let p = Coord { x: 1.0, y: 1.0 };
        CoordInstruction { c1, c2, p }
    };
    canvas.add_coord(AddCurve {
        curve,
        index_shape: shape_index,
        index_curve: 2,
    });
    let curve: CoordInstruction = {
        let c1 = Coord { x: 1.0, y: 0.0 };
        let c2 = Coord { x: 1.0, y: 0.0 };
        let p = Coord { x: 1.0, y: 0.0 };
        CoordInstruction { c1, c2, p }
    };
    canvas.add_coord(AddCurve {
        curve,
        index_shape: shape_index,
        index_curve: 3,
    });

    canvas.optimize_coord();

    canvas
}


fn generate_from_line(y: &[Coord]) -> Vgc {
    let color = Rgba {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    let mut canvas = Vgc::new(16.0 / 16.0, color);

    if y.len() >0 {
        let p0 = &y[0];

        let shape_index = canvas.add_shape(ShapeInstruction {
            start: p0.clone(),
            curves: Vec::default(),
            color: Rgba {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
        });
    
        for i in 1..y.len() {
            let curve = {
                let c1 = y[i].clone();
                let c2 = y[i].clone();
                let p = y[i].clone();
                CoordInstruction { c1, c2, p }
            };
            canvas.add_coord(AddCurve {
                curve,
                index_shape: shape_index,
                index_curve: i - 1,
            });
        }
    }
   
    canvas
}

fn generate_from_push(y: &[Coord]) -> Vgc {
    let color = Rgba {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    let mut canvas = Vgc::new(16.0 / 16.0, color);

    if y.len() >0 {
        let p0 = &y[0];

        let shape_index = canvas.create_shape(p0.clone(), Rgba {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            });
    
        for i in 0..((y.len()-1)/3) {
            let index = i*3+1;
            canvas.push_coord(shape_index, y[index].clone(), y[index+1].clone(), y[index+2].clone());
        }
    }
   
    canvas
}

pub fn generate_triangle_exemple() -> Vgc {
    let color = Rgba {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    let mut canvas = Vgc::new(16.0 / 16.0, color);

    let p0 = Coord { x: 0.0, y: 0.0 };

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


    let curve: CoordInstruction = {
        let c1 = Coord { x: 0.0, y: 1.0 };
        let c2 = Coord { x: 0.0, y: 1.0 };
        let p = Coord { x: 0.0, y: 1.0 };
        CoordInstruction { c1, c2, p }
    };
    canvas.add_coord(AddCurve {
        curve,
        index_shape: shape_index,
        index_curve: 0,
    });
    let curve: CoordInstruction = {
        let c1 = Coord { x: 1.0, y: 1.0 };
        let c2 = Coord { x: 1.0, y: 1.0 };
        let p = Coord { x: 1.0, y: 1.0 };
        CoordInstruction { c1, c2, p }
    };
    canvas.add_coord(AddCurve {
        curve,
        index_shape: shape_index,
        index_curve: 1,
    });
    

    canvas.optimize_coord();

    canvas
}


// TODO : How to create a stable api for this?