use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
    coord::{Coord, CoordPtr},
    curve::Curve,
    fill::Rgba,
    shape::Shape,
    Vgc,
};

#[derive(Serialize, Deserialize, Debug)]
struct VgcSerialization {
    ratio: f64,
    background: Rgba,
    shapes: Vec<ShapeSerialization>,
    coords: Vec<Coord>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ShapeSerialization {
    start: usize,
    curves: Vec<CurveSerialization>,
    color: Rgba,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurveSerialization {
    cp0: usize,
    cp1: usize,
    p1: usize,
}

impl VgcSerialization {
    fn from_vgc(vgc: &Vgc) -> VgcSerialization {
        let mut vgc_serialization = VgcSerialization {
            ratio: vgc.ratio,
            background: vgc.background.clone(),
            shapes: Vec::new(),
            coords: Vec::new(),
        };

        let mut coord_map: Vec<(CoordPtr, usize)> = Vec::new();
        let mut index: usize = 0;

        for shape in vgc.shapes.iter() {
            let mut shape_serialization = ShapeSerialization {
                start: 0,
                curves: Vec::new(),
                color: shape.color.clone(),
            };

            let start_ptr = shape.start.clone();
            shape_serialization.start = create_index_coord(
                &mut coord_map,
                &start_ptr,
                &mut index,
                &mut vgc_serialization,
            );

            for curve in shape.curves.iter() {
                let mut curve_serialization = CurveSerialization {
                    cp0: 0,
                    cp1: 0,
                    p1: 0,
                };

                let cp0_ptr = curve.cp0.clone();
                curve_serialization.cp0 = create_index_coord(
                    &mut coord_map,
                    &cp0_ptr,
                    &mut index,
                    &mut vgc_serialization,
                );

                let cp1_ptr = curve.cp1.clone();
                curve_serialization.cp1 = create_index_coord(
                    &mut coord_map,
                    &cp1_ptr,
                    &mut index,
                    &mut vgc_serialization,
                );

                let p1_ptr = curve.p1.clone();
                curve_serialization.p1 =
                    create_index_coord(&mut coord_map, &p1_ptr, &mut index, &mut vgc_serialization);

                shape_serialization.curves.push(curve_serialization);
            }

            vgc_serialization.shapes.push(shape_serialization);
        }

        vgc_serialization
    }

    fn into_vgc(self) -> Vgc {
        let mut vgc = Vgc::new(self.ratio, self.background);
        let mut coord_map: Vec<CoordPtr> = Vec::new();

        for coord in self.coords.iter() {
            coord_map.push(Rc::new(RefCell::new(coord.clone())));
        }

        for shape in self.shapes.iter() {
            let start = coord_map[shape.start].clone();
            let mut curves = Vec::new();

            for curve in shape.curves.iter() {
                let cp0 = coord_map[curve.cp0].clone();

                let cp1 = coord_map[curve.cp1].clone();

                let p1 = coord_map[curve.p1].clone();

                curves.push(Curve { cp0, cp1, p1 });
            }

            vgc.shapes.push(Shape {
                start,
                curves,
                color: shape.color.clone(),
            });
        }

        vgc
    }
}

fn create_index_coord(
    coord_map: &mut Vec<(CoordPtr, usize)>,
    coord_ptr: &CoordPtr,
    index: &mut usize,
    vgc_serialization: &mut VgcSerialization,
) -> usize {
    match find_coord_index(coord_map, coord_ptr) {
        Some(index) => index,
        None => {
            let current_index = *index;
            coord_map.push((coord_ptr.clone(), current_index));
            vgc_serialization.coords.push(coord_ptr.borrow().clone());
            *index += 1;
            current_index
        }
    }
}

fn find_coord_index(coord_map: &[(CoordPtr, usize)], ptr: &CoordPtr) -> Option<usize> {
    for (coord_ptr, index) in coord_map.iter() {
        if Rc::ptr_eq(coord_ptr, ptr) {
            return Some(*index);
        }
    }
    None
}

impl Serialize for Vgc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        VgcSerialization::from_vgc(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Vgc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let deser = VgcSerialization::deserialize(deserializer)?;
        Ok(deser.into_vgc())
    }
}

mod test {

    #[test]
    fn test_create_index_coord() {
        use super::*;
        use crate::coord::Coord;

        let mut coord_map: Vec<(CoordPtr, usize)> = Vec::new();
        let mut index: usize = 0;
        let mut vgc_serialization = VgcSerialization {
            ratio: 1.0,
            background: Rgba::new(0, 0, 0, 0),
            shapes: Vec::new(),
            coords: Vec::new(),
        };

        let start_ptr = Rc::new(RefCell::new(Coord { x: 0.0, y: 0.0 }));

        let start_index = create_index_coord(
            &mut coord_map,
            &start_ptr,
            &mut index,
            &mut vgc_serialization,
        );
        assert_eq!(start_index, 0);
        assert_eq!(coord_map.len(), 1);
        assert_eq!(vgc_serialization.coords.len(), 1);
        assert_eq!(index, 1);
    }

    #[test]
    fn test_serialization() {
        use super::*;
        use crate::{coord::Coord, generate_from_line};
        use postcard::{from_bytes, to_allocvec};

        let canvas_in = generate_from_line(vec![vec![
            Coord { x: 0.0, y: 0.0 },
            Coord {
                x: -0.46193975,
                y: 0.19134173,
            },
            Coord { x: 0.0, y: 1.0 },
            Coord { x: 1.0, y: 1.0 },
            Coord {
                x: 0.46193975,
                y: -0.19134173,
            },
        ]]);

        let output = to_allocvec(&canvas_in).expect("Serialization should be valid");

        let canvas_out = from_bytes::<Vgc>(&output).expect("Deserialization should be valid");

        assert_eq!(canvas_in.debug_string(), canvas_out.debug_string());
        assert_eq!(canvas_in.ratio, canvas_out.ratio);
        assert_eq!(canvas_in.background, canvas_out.background);
        assert_eq!(canvas_in.shapes.len(), canvas_out.shapes.len());
        assert_eq!(canvas_in.shapes[0].color, canvas_out.shapes[0].color);
        assert_eq!(
            Rc::strong_count(&canvas_in.shapes[0].start),
            Rc::strong_count(&canvas_out.shapes[0].start)
        );
        assert_eq!(
            Rc::strong_count(&canvas_in.shapes[0].curves[0].cp0),
            Rc::strong_count(&canvas_out.shapes[0].curves[0].cp0)
        );
        assert_eq!(
            Rc::strong_count(&canvas_in.shapes[0].curves[0].cp1),
            Rc::strong_count(&canvas_out.shapes[0].curves[0].cp1)
        );
        assert_eq!(
            Rc::strong_count(&canvas_in.shapes[0].curves[0].p1),
            Rc::strong_count(&canvas_out.shapes[0].curves[0].p1)
        );
    }
}
