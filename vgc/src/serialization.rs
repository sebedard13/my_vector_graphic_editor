use std::{rc::Rc, cell::RefCell};

use serde::{Deserialize, Serialize};

use crate::{fill::Rgba, coord::{Coord, CoordPtr}, Vgc, shape::Shape, curve::Curve};

#[derive(Serialize, Deserialize)]
struct VgcSerialization{
    ratio: f64,
    background: Rgba,
    shapes: Vec<ShapeSerialization>,
    coords: Vec<Coord>,
}

#[derive(Serialize, Deserialize)]
struct ShapeSerialization{
    start: usize,
    curves: Vec<CurveSerialization>,
    color: Rgba,
}

#[derive(Serialize, Deserialize)]
struct CurveSerialization{
    cp0: usize,
    cp1: usize,
    p1: usize,
}


impl VgcSerialization{
    pub fn from_vgc(vgc: &Vgc) -> VgcSerialization {
        let mut vgc_serialization = VgcSerialization {
            ratio: vgc.ratio,
            background: vgc.background.clone(),
            shapes: Vec::new(),
            coords: Vec::new(),
        };

        let mut coord_map:Vec<(CoordPtr, usize)> = Vec::new();
        let mut index : usize = 0;
        
        for shape in vgc.shapes.iter() {
            let mut shape_serialization = ShapeSerialization {
                start: 0,
                curves: Vec::new(),
                color: shape.color.clone(),
            };

            let start_ptr = shape.start.clone();
            index = create_index_coord(&mut coord_map, &start_ptr, index, &mut vgc_serialization);
            shape_serialization.start = index;

            for curve in shape.curves.iter() {
                let mut curve_serialization = CurveSerialization {
                    cp0: 0,
                    cp1: 0,
                    p1: 0,
                };

                let cp0_ptr = curve.cp0.clone();
                index = create_index_coord(&mut coord_map, &cp0_ptr, index, &mut vgc_serialization);
                curve_serialization.cp0 = index;

                let cp1_ptr = curve.cp1.clone();
                index = create_index_coord(&mut coord_map, &cp1_ptr, index, &mut vgc_serialization);
                curve_serialization.cp1 = index;

                let p1_ptr = curve.p1.clone();
                index = create_index_coord(&mut coord_map, &p1_ptr, index, &mut vgc_serialization);
                curve_serialization.p1 = index;

                shape_serialization.curves.push(curve_serialization);
            }

            vgc_serialization.shapes.push(shape_serialization);
        }
    
        vgc_serialization
    }

    pub fn into_vgc(self)->Vgc{
        let mut vgc = Vgc::new(self.ratio, self.background);
        let mut coord_map:Vec<CoordPtr> = Vec::new();


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

                curves.push(Curve {cp0, cp1, p1});
            }
            
            vgc.shapes.push(Shape {start, curves, color:shape.color.clone()});
        }

        vgc
    }
}

fn create_index_coord(coord_map: &mut Vec<(CoordPtr, usize)>, coord_ptr: &CoordPtr, index: usize, vgc_serialization: &mut VgcSerialization)->usize {
    match find_coord_index(&coord_map, &coord_ptr) {
        Some(index) => {
           index
        },
        None => {
            coord_map.push((coord_ptr.clone(), index));
            vgc_serialization.coords.push(coord_ptr.borrow().clone());
            index+1
        }
    }
}

fn find_coord_index(coord_map: &Vec<(CoordPtr, usize)>, ptr: &CoordPtr)->Option<usize>{
    for (coord_ptr, index) in coord_map.iter() {
        if Rc::ptr_eq(coord_ptr,ptr) {
            return Some(*index);
        }
    }
    None
}