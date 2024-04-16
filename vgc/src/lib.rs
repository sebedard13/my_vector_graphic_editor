use std::rc::Rc;
use std::cell::RefCell;

use common::{
    pures::Vec2,
    types::{Coord, Rect},
};
use coord::CoordPtr;
use shape::Shape;

use common::{dbg_str, Rgba};
#[cfg(feature = "tiny-skia")]
pub use render::TinySkiaRenderer;
pub use render::VgcRenderer;

pub mod coord;
mod curve;
mod curve2;

pub mod render;

#[cfg(feature = "serialization")]
mod serialization;

pub mod shape;

/// Maximum size of the image, if we want to have detail for each pixel
/// This is a limit because of f32 precision with 2^-23 for the smallest value
/// See decision.md for more information
#[allow(dead_code)]
static MAX_DETAIL_SIZE: u32 = 52000000;

#[derive(Debug)]
pub struct Vgc {
    pub background: Rgba,
    pub shapes: Vec<Shape>,
}

impl Vgc {
    pub fn new(background: Rgba) -> Vgc {
        Vgc {
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

    pub fn push_shape(&mut self, shape: Shape) -> usize {
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    pub fn replace_shape(&mut self, index_shape: usize, shape: Shape) {
        if index_shape < self.shapes.len() {
            self.shapes[index_shape] = shape;
        } else {
            log::error!(
                "{}",
                dbg_str!("Index out of bound {}/{}", index_shape, self.shapes.len())
            );
        }
    }

    pub fn render<T>(&self, renderer: &mut T) -> Result<(), String>
    where
        T: render::VgcRenderer,
    {
        render::render_true(self, renderer)
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

    pub fn shapes_contains(&self, coord: &Coord) -> Vec<usize> {
        let mut vec = Vec::new();
        for shape_index in (0..self.shapes.len()).rev() {
            let shape = &self.shapes[shape_index];
            if shape.contains(coord) {
                vec.push(shape_index);
            }
        }
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

    pub fn max_rect(&self) -> Rect {
        Rect::new(-1.0, -1.0, 1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_closest_coord_on_shape_triangle() {
        let canvas = generate_from_line(vec![vec![
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 1.0),
            Coord::new(1.0, 1.0),
        ]]);

        let shape = canvas.get_shape(0).unwrap();
        let (_, _, _, coord) = shape.closest_curve(&Coord::new(1.0, 0.0));
        assert_eq!(coord.x(), 0.5);
        assert_eq!(coord.y(), 0.5);
    }

    #[test]
    fn genreate_from_push() {
        let canvas = generate_from_push(vec![vec![
            Coord::new(0.0, 0.0),
            Coord::new(-0.46193975, 0.19134173),
            Coord::new(0.0, 1.0),
            Coord::new(0.0, 1.0),
            Coord::new(0.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(0.46193975, -0.19134173),
            Coord::new(0.0, 0.0),
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

    let mut canvas = Vgc::new(color);

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

    let mut canvas = Vgc::new(color);

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

//Return the index of the created shape
pub fn create_circle(canvas: &mut Vgc, center: Coord, radius_x: f32, radius_y: f32) -> usize {
    let color = Rgba {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    let shape = Shape::new_circle(center, Vec2::new(radius_x, radius_y), color);

    canvas.push_shape(shape)
}
