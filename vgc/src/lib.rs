use std::cell::RefCell;
use std::rc::Rc;

use crate::coord::Coord;
use coord::{CoordPtr, RefCoordType};
use shape::Shape;

pub use fill::Rgba;
#[cfg(feature = "tiny-skia")]
pub use render::TinySkiaRenderer;
pub use render::VgcRenderer;

pub mod coord;
mod curve;
mod fill;
pub mod render;

#[cfg(feature = "serialization")]
mod serialization;

mod shape;

#[derive(Debug)]
pub struct Vgc {
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
            start: Rc::new(RefCell::new(start)),
            curves: Vec::new(),
            color,
        };
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    pub fn get_shape(&self, index_shape: usize) -> Option<&Shape> {
        self.shapes.get(index_shape)
    }

    pub fn get_shape_mut(&mut self, index_shape: usize) -> Option<&mut Shape> {
        self.shapes.get_mut(index_shape)
    }

    pub fn render<T>(&self, renderer: &mut T) -> Result<(), String>
    where
        T: render::VgcRenderer,
    {
        render::render_true(self, renderer)
    }

    pub fn visit(&self, f: &mut dyn FnMut(usize, RefCoordType)) {
        for (shape_index, shape) in self.shapes.iter().enumerate() {
            f(shape_index, RefCoordType::Start(shape.start.borrow()));
            for (curve_index, curve) in shape.curves.iter().enumerate() {
                f(
                    shape_index,
                    RefCoordType::Cp0(curve_index, curve.cp0.borrow()),
                );
                f(
                    shape_index,
                    RefCoordType::Cp1(curve_index, curve.cp1.borrow()),
                );
                f(
                    shape_index,
                    RefCoordType::P1(curve_index, curve.p1.borrow()),
                );
            }
        }
    }

    pub fn visit_ptr(&self, f: &mut dyn FnMut(&CoordPtr)) {
        for (_, shape) in self.shapes.iter().enumerate() {
            f(&shape.start);
            for (_, curve) in shape.curves.iter().enumerate() {
                f(&curve.cp0);
                f(&curve.cp1);
                f(&curve.p1);
            }
        }
    }

    pub fn visit_vec(&self) -> Vec<(usize, RefCoordType)> {
        let mut vec = Vec::new();
        for (shape_index, shape) in self.shapes.iter().enumerate() {
            vec.push((shape_index, RefCoordType::Start(shape.start.borrow())));
            for (curve_index, curve) in shape.curves.iter().enumerate() {
                vec.push((
                    shape_index,
                    RefCoordType::Cp0(curve_index, curve.cp0.borrow()),
                ));
                vec.push((
                    shape_index,
                    RefCoordType::Cp1(curve_index, curve.cp1.borrow()),
                ));
                vec.push((
                    shape_index,
                    RefCoordType::P1(curve_index, curve.p1.borrow()),
                ));
            }
        }

        vec
    }

    pub fn shapes_closest(&self, coord: &Coord) -> Vec<(usize, usize, f32, Coord)> {
        let mut vec = Vec::new();
        for (shape_index, shape) in self.shapes.iter().enumerate() {
            let (curve_index, _, distance, coord) = shape.closest_curve(coord);

            vec.push((shape_index, curve_index, distance, coord));
        }
        vec.sort_by(|(_, _, distance1, _), (_, _, distance2, _)| {
            distance1
                .partial_cmp(distance2)
                .expect("No Nan value possible")
        });
        vec
    }

    pub fn debug_string(&self) -> String {
        let mut string = "".to_string();
        for shape in &self.shapes {
            string.push_str(&shape.to_path());
            string.push('\n');
        }
        string
    }

    pub fn remove_shape(&mut self, shape_index: usize) {
        self.shapes.remove(shape_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_closest_coord_on_shape_triangle() {
        let canvas = generate_from_line(vec![vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 0.0, y: 1.0 },
            Coord { x: 1.0, y: 1.0 },
        ]]);

        let shape = canvas.get_shape(0).unwrap();
        let (_, _, _, coord) = shape.closest_curve(&Coord::new(1.0, 0.0));
        assert_eq!(coord.x, 0.5);
        assert_eq!(coord.y, 0.5);
    }

    #[test]
    fn genreate_from_push() {
        let canvas = generate_from_push(vec![vec![
            Coord { x: 0.0, y: 0.0 },
            Coord {
                x: -0.46193975,
                y: 0.19134173,
            },
            Coord { x: 0.0, y: 1.0 },
            Coord { x: 0.0, y: 1.0 },
            Coord { x: 0.0, y: 1.0 },
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 1.0, y: 1.0 },
            Coord {
                x: 0.46193975,
                y: -0.19134173,
            },
            Coord { x: 0.0, y: 0.0 },
        ]]);

        assert_eq!(canvas.debug_string(), "M 0 0 C -0.46193975 0.19134173 0 1 0 1 C 0 1 1 1 1 1 C 1 1 0.46193975 -0.19134173 0 0 Z\n");
    }
}

pub fn generate_from_line(shapes_coords: Vec<Vec<Coord>>) -> Vgc {
    let color = Rgba {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    let mut canvas = Vgc::new(1.0, color);

    for shape_coords in shapes_coords {
        if !shape_coords.is_empty() {
            let p0 = &shape_coords[0];

            let shape_index = canvas.create_shape(
                p0.clone(),
                Rgba {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                },
            );

            let shape = canvas.get_shape_mut(shape_index).unwrap();
            let mut previous = shape.start.clone();
            for coord in shape_coords.iter().skip(1) {
                let p1 = Rc::new(RefCell::new(coord.clone()));
                shape.push_coord(previous, p1.clone(), p1.clone());
                previous = p1;
            }
            shape.close()
        }
    }

    canvas
}

pub fn generate_from_push(shapes_coords: Vec<Vec<Coord>>) -> Vgc {
    let color = Rgba {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    let mut canvas = Vgc::new(1.0, color);

    for shape_coords in shapes_coords {
        if !shape_coords.is_empty() {
            let p0 = &shape_coords[0];

            let shape_index = canvas.create_shape(
                p0.clone(),
                Rgba {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                },
            );

            let shape = canvas.get_shape_mut(shape_index).unwrap();

            for i in 0..((shape_coords.len() - 1) / 3) {
                let index = i * 3 + 1;
                shape.push_coord(
                    Rc::new(RefCell::new(shape_coords[index].clone())),
                    Rc::new(RefCell::new(shape_coords[index + 1].clone())),
                    Rc::new(RefCell::new(shape_coords[index + 2].clone())),
                );
            }
            shape.close()
        }
    }

    canvas
}

pub fn create_circle(canvas: &mut Vgc, center: Coord, radius: f32) {
    //https://spencermortensen.com/articles/bezier-circle/
    let a = 1.000_055_2;
    let b = 0.553_426_9;
    let c = 0.998_735_9;

    let p0 = Coord::new(0.0, a);

    let shape_index = canvas.create_shape(
        p0.clone(),
        Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },
    );
    let shape = canvas.get_shape_mut(shape_index).unwrap();

    let vec = vec![
        shape.start.clone(),
        Rc::new(RefCell::new(Coord::new(b, c))),
        Rc::new(RefCell::new(Coord::new(c, b))),
        Rc::new(RefCell::new(Coord::new(a, 0.0))),
        Rc::new(RefCell::new(Coord::new(c, -b))),
        Rc::new(RefCell::new(Coord::new(b, -c))),
        Rc::new(RefCell::new(Coord::new(0.0, -a))),
        Rc::new(RefCell::new(Coord::new(-b, -c))),
        Rc::new(RefCell::new(Coord::new(-c, -b))),
        Rc::new(RefCell::new(Coord::new(-a, 0.0))),
        Rc::new(RefCell::new(Coord::new(-c, b))),
        Rc::new(RefCell::new(Coord::new(-b, c))),
    ];

    shape.push_coord(vec[1].clone(), vec[2].clone(), vec[3].clone());

    shape.push_coord(vec[4].clone(), vec[5].clone(), vec[6].clone());

    shape.push_coord(vec[7].clone(), vec[8].clone(), vec[9].clone());

    shape.push_coord(vec[10].clone(), vec[11].clone(), vec[0].clone());

    for coord_ref in vec {
        let mut coord = coord_ref.borrow_mut();
        coord.x *= radius;
        coord.y *= radius;
        coord.x += center.x;
        coord.y += center.y;
    }
}
