use std::cell::RefCell;
use std::rc::Rc;

use crate::coord::Coord;
use crate::fill::Rgba;
use coord::RefCoordType;
use iced::widget::canvas::Frame;
use shape::Shape;

pub mod coord;
pub mod render;
mod fill;
mod curve;
mod shape;

#[derive(Debug)]
pub struct Vgc{
    /// width/height
    pub ratio: f64,
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

    pub fn get_shape(&self, index_shape: usize) -> Option<&Shape>{
        self.shapes.get(index_shape)
    }

    pub fn get_mut_shape(&mut self, index_shape: usize) -> Option<&mut Shape>{
        self.shapes.get_mut(index_shape)
    }

    pub fn frame_render(&self, frame: &mut Frame) {
        render::frame_render(self, frame);
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

    pub fn debug_string(&self)->String{
        let mut string = "".to_string();
        for shape in &self.shapes{
            string.push_str(&shape.to_path());
            string.push_str("\n");
        }
        string
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

        let shape = canvas.get_shape(0).unwrap();
        let (_,_,_,coord) = shape.closest_curve(&Coord::new(1.0, 0.0));
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

        let shape = canvas.get_mut_shape(shape_index).unwrap();
        let mut previous = shape.start.clone();
        for i in 1..y.len() {
            let p1 = Rc::new(RefCell::new(y[i].clone()));
            shape.push_coord(previous,p1.clone(), p1.clone());
            previous = p1;
        }
        shape.close()
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

        let shape = canvas.get_mut_shape(shape_index).unwrap();
    
        for i in 0..((y.len()-1)/3) {
            let index = i*3+1;
            shape.push_coord(Rc::new(RefCell::new(y[index].clone())), Rc::new(RefCell::new(y[index+1].clone())), Rc::new(RefCell::new(y[index+2].clone())));
        }
        shape.close()
    }
   
    canvas
}