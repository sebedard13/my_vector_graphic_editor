use std::cell::{RefCell, Ref};
use std::rc::Rc;

use crate::coord::Coord;
use crate::instructions::{AddCurve};
use crate::vgc_struct::{Rgba, Shape};
use iced::widget::canvas::Frame;

pub mod coord;
mod instructions;
pub mod render;
mod vgc_struct;
mod curve;

#[derive(Debug)]
pub struct Vgc{
    pub ratio: f64, //width/height 16/9
    pub background: Rgba,
    shapes: Vec<Shape>,
}

impl Vgc {
    pub fn new(ratio: f64, background: Rgba) -> Vgc {
        Vgc {
            ratio,
            background,
            shapes: Vec::new(),
        }
    }

    pub fn create_shape(&mut self, start: Coord, color: Rgba) -> usize {
        let shape = Shape {
            start:  Rc::new(RefCell::new(start)),
            curves: Vec::new(),
            color,
        };
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    pub fn move_coord(&mut self, index_shape: usize, coord_type: &CoordType, x: f32, y: f32) {
        match coord_type {
            CoordType::Start => {
                let mut coord = self.shapes[index_shape].start.borrow_mut();
                coord.x = x;
                coord.y = y;
            }
            CoordType::Cp0(index_curve) => {
                let mut coord = self.shapes[index_shape].curves[*index_curve].cp0.borrow_mut();
                coord.x = x;
                coord.y = y;
            }
            CoordType::Cp1(index_curve) => {
                let mut coord = self.shapes[index_shape].curves[*index_curve].cp1.borrow_mut();
                coord.x = x;
                coord.y = y;
            }
            CoordType::P1(index_curve) => {
                let mut coord = self.shapes[index_shape].curves[*index_curve].p1.borrow_mut();
                coord.x = x;
                coord.y = y;
            }
        }

    }

    pub fn add_coord(&mut self, _: AddCurve) {
        // TODO
        /*let curve = insert_curve(&mut self.coord_ds, add_curve_coord.curve);

        self.shapes[add_curve_coord.index_shape].add_coord(
            &mut self.coord_ds,
            curve,
            add_curve_coord.index_curve,
        );*/
    }

    pub fn push_coord(&mut self, index_shape: usize, cp0 : Coord, cp1 : Coord, p1 : Coord) {
       
        self.shapes[index_shape].push_coord(cp0, cp1, p1);
    }

    pub fn frame_render(&self, frame: &mut Frame) {
        render::frame_render(self, frame);
    }

    pub fn optimize_coord(&mut self) {
        //TODO
        // TODO Maybe Coord implement Hash and be use in HashMap directly
        /*let mut coord_map: HashMap<u64, Rc<RefCell<Coord>>> = HashMap::new();


        
        for shape in self.shapes.iter_mut() {
            shape.start = optimize_coord_index(&shape.start, &mut coord_map, &mut coord_ds, &self.coord_ds);
            for curve in shape.curves.iter_mut() {
                curve.cp0 = optimize_coord_index(&curve.cp0, &mut coord_map, &mut coord_ds, &self.coord_ds);
                curve.cp1 = optimize_coord_index(&curve.cp1, &mut coord_map, &mut coord_ds, &self.coord_ds);
                curve.p1 = optimize_coord_index(&curve.p1, &mut coord_map, &mut coord_ds, &self.coord_ds);
            }
        }

        self.coord_ds = coord_ds;*/
    }

    pub fn visit(&self, f: &mut dyn FnMut(usize, RefCoordType)) {
        for (shape_index, shape) in self.shapes.iter().enumerate() {
            f(shape_index, RefCoordType::Start(shape.start.borrow()));
            for (curve_index, curve) in shape.curves.iter().enumerate() {
                f(shape_index, RefCoordType::Cp0(curve_index, curve.cp0.borrow()));
                f(shape_index, RefCoordType::Cp1(curve_index, curve.cp1.borrow()));
                f(shape_index, RefCoordType::P1(curve_index, curve.p1.borrow()));
            }
        }
    }

    pub fn visit_vec(&self)->Vec<(usize, RefCoordType)>{
        let mut vec = Vec::new();
        for (shape_index, shape) in self.shapes.iter().enumerate() {
            vec.push((shape_index, RefCoordType::Start(shape.start.borrow())));
            for (curve_index, curve) in shape.curves.iter().enumerate() {
                vec.push((shape_index, RefCoordType::Cp0(curve_index, curve.cp0.borrow())));
                vec.push((shape_index, RefCoordType::Cp1(curve_index, curve.cp1.borrow())));
                vec.push((shape_index, RefCoordType::P1(curve_index, curve.p1.borrow())));
            }
        }
        
        vec
    }

    pub fn get_cp_of_shape<'a>(&'a self, shape_index: usize) -> Vec<RefCoordType<'a>> {
        let mut vec = Vec::new();
        for (curve_index, curve) in  self.shapes[shape_index].curves.iter().enumerate() {
            vec.push(RefCoordType::Cp0(curve_index, curve.cp0.borrow()));
            vec.push(RefCoordType::Cp1(curve_index, curve.cp1.borrow()));
        }
        vec
    }

    pub fn get_p_of_shape(&self, shape_index: usize) -> Vec<Ref<Coord>> {
        let mut vec = Vec::new();
        vec.push(self.shapes[shape_index].start.borrow());
        for curve in  self.shapes[shape_index].curves.iter() {
            vec.push(curve.p1.borrow());
        }
        vec
    }

    pub fn get_coords_of_shape(&self, shape_index: usize) -> Vec<Ref<Coord>> {
        let mut vec = Vec::new();
        vec.push(self.shapes[shape_index].start.borrow());
        for curve in  self.shapes[shape_index].curves.iter(){
            vec.push(curve.cp0.borrow());
            vec.push(curve.cp1.borrow());
            vec.push(curve.p1.borrow());
        }
        vec
    }

    pub fn toggle_separate_join_handle(&mut self, shape_index: usize, curve_index: usize) {
        self.shapes[shape_index].toggle_separate_join_handle(curve_index);
    }

    pub fn debug_string(&self)->String{
        let mut string = "".to_string();
        for shape in &self.shapes{
            string.push_str(&shape.to_path());
            string.push_str("\n");
        }
        string
    }

    pub fn set_shape_background(&mut self, shape_index: usize, color: Rgba) {
        self.shapes[shape_index].color = color;
    }

    pub fn get_closest_coord_on_shape(&self, shape_index: usize, x: f32, y: f32) -> Coord {
        self.shapes[shape_index].closest_coord_on(Coord {x, y})
    }
}

pub enum RefCoordType<'a> {
    Start(Ref<'a, Coord>),
    /// Curve index, coord
    Cp0(usize, Ref<'a, Coord>),
    /// Curve index, coord
    Cp1(usize, Ref<'a, Coord>),
    /// Curve index, coord
    P1(usize, Ref<'a, Coord>),
}

#[derive(Debug, Clone)]
pub enum CoordType{
    Start,
    Cp0(usize),
    /// Curve index
    Cp1(usize),
    /// Curve index
    P1(usize),
}

impl RefCoordType<'_> {
    pub fn get_coord(&self) -> &Coord {
        match self {
            RefCoordType::Start(coord) => coord,
            RefCoordType::Cp0(_, coord) => coord,
            RefCoordType::Cp1(_, coord) => coord,
            RefCoordType::P1(_, coord) => coord,
        }
    }

    pub fn to_coord_type(&self)->CoordType{
        match self {
            RefCoordType::Start(_) => CoordType::Start,
            RefCoordType::Cp0(index, _) => CoordType::Cp0(*index),
            RefCoordType::Cp1(index, _) => CoordType::Cp1(*index),
            RefCoordType::P1(index, _) => CoordType::P1(*index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

pub fn generate_from_line(y: &[Coord]) -> Vgc {
    let color = Rgba {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    let mut canvas = Vgc::new(16.0 / 16.0, color);

    if y.len() >0 {
        let p0 = &y[0];

        let shape_index = canvas.create_shape(
            p0.clone(),
            Rgba {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
        );
    
        for i in 1..y.len() {
            canvas.push_coord(shape_index, y[i-1].clone(), y[i].clone(), y[i].clone())
        }
    }
   
    canvas
}

pub fn generate_from_push(y: &[Coord]) -> Vgc {
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