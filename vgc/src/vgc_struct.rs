use std::cell::{RefCell, Ref};
use std::mem::swap;
use std::rc::Rc;

use crate::coord::{Coord, RefCoordType};
use crate::curve;
use crate::curve::Curve;

#[derive(Debug)]
pub struct Shape {
    pub start: Rc<RefCell<Coord>>,
    pub curves: Vec<Curve>,
    pub color: Rgba,
}

impl Shape {
    pub fn add_coord(&mut self,  mut curve: Curve, index: usize) {
        let curve_after = self.curves.get_mut(index).expect("Index should be valid because we should not be able to add a curve at the end of the shape because the last element close the curve with a link to the start coord in shape");

        swap(&mut curve.cp0, &mut curve.cp1);
        swap(&mut curve.cp0, &mut curve_after.cp0);
        self.curves.insert(index, curve);
    }

    pub fn toggle_separate_join_handle(&mut self,  index: usize) {
        if self.is_handles_joined(index) {
            self.separate_handle(index);
        } else {
            self.join_handle(index);
        }
    }

    fn is_handles_joined(&self, index: usize) -> bool {
        let curve = &self.curves[index];
        curve.cp0 == curve.p1 || curve.cp1 == curve.p1
    }

    pub fn is_closed(&self) -> bool {
        if self.curves.len() == 0 {
            return false;
        }
        let last_curve = self.curves.last().expect("Shape should have at least one curve ");
        last_curve.p1 == self.start
    }

    pub fn close(&mut self) {
        if !self.is_closed(){
            self.curves.push(Curve {
                cp0: self.start.clone(),
                cp1: self.start.clone(),
                p1: self.start.clone(),
            });
        }
    }

    pub fn separate_handle(&mut self,  curve_index_p1: usize) {
        let (coord_index0, coord_index1) = {
             //Todo check if index is not the last curve and what not
            let p0 = {
                if curve_index_p1 == 0 {
                    self.start.borrow()
                } else {
                    self.curves[curve_index_p1 - 1].p1.borrow()
                }
            };
            let current_curve = &self.curves[curve_index_p1];
            let cp0 = &current_curve.cp0.borrow();
            let cp1 = &current_curve.cp1.borrow();
            let p1 = &current_curve.p1.borrow();

            let next_curve = &self.curves[(curve_index_p1 + 1)% self.curves.len()];
            let cp2 = &next_curve.cp0.borrow();
            let cp3 = &next_curve.cp1.borrow();
            let p2 = &next_curve.p1.borrow();
            
            
            
            curve::tangent_cornor_pts(&p0, cp0, cp1, p1, cp2, cp3, p2)
        };

        self.curves[curve_index_p1].cp1 = Rc::new(RefCell::new(coord_index0));
        let len = self.curves.len();
        self.curves[(curve_index_p1 + 1) % len].cp0 = Rc::new(RefCell::new(coord_index1));
    }

    pub fn join_handle(&mut self,  curve_index_p1: usize) {
       //cp0
       let coord_index = &self.curves[curve_index_p1].p1;
       let curve_after = (curve_index_p1 + 1) % self.curves.len();
       
       self.curves[curve_after].cp0 = coord_index.clone();
        
        //cp1
        let coord_index = &self.curves[curve_index_p1].p1;
       
        self.curves[curve_index_p1].cp1 = coord_index.clone();
    }

    pub fn to_path(&self) -> String {
        let mut path = String::new();
        let start = self.start.borrow();
        path.push_str(&format!("M {} {}", start.x, start.y));
        for curve in &self.curves {
            path.push_str(" ");
            path.push_str(&curve.to_path());
        }
        if self.is_closed() {
            path.push_str(" Z");
        }
        path
    }


    /// Visit each curve of the shape and call the visitor function with the curve index and 4 coords of the curve so p0, cp0, cp1, p1
    pub fn visit_full_curves(&self,  mut visitor: impl FnMut(usize, &Coord, &Coord, &Coord, &Coord)) {
        let start = self.start.borrow();
        let mut prev_coord = start;
        for (index,curve) in self.curves.iter().enumerate() {
            let cp0 = curve.cp0.borrow();
            let cp1 = curve.cp1.borrow();
            let p1 = curve.p1.borrow();
          
            visitor(index, &prev_coord, &cp0, &cp1, &p1);

            prev_coord = p1;
        }
    }

    /// Visit each curve and calculate the closest point on the curve to the coord
    /// Return the curve index, the t value of the closest point on the curve, the distance to the closest point and the closest point
    pub fn closest_curve(&self, coord: &Coord)-> (usize, f32,f32, Coord){
        let mut min_distance = std::f32::MAX;
        let mut min_index = 0;
        let mut min_t = 0.0;
        let mut min_coord = self.start.borrow().clone();

        self.visit_full_curves(|curve_index, p0, cp0, cp1, p1|{
            let (t_min, distance, coord_closest) = curve::t_closest(&coord, p0, cp0, cp1, p1);

            if distance < min_distance{
                min_distance = distance;
                min_index = curve_index;
                min_t = t_min;
                min_coord = coord_closest;
            }
        });
        (min_index, min_t, min_distance, min_coord)
    }

    pub fn get_coords_of_curve(&self, curve_index:usize)->(Ref<Coord>, Ref<Coord>, Ref<Coord>, Ref<Coord>){
        let mut prev_coord =self.start.borrow();

        if curve_index > 0{
            let prev_curve = self.curves.get(curve_index - 1).expect("Index should be valid");
            prev_coord = prev_curve.p1.borrow();
        }
        let curve = self.curves.get(curve_index).expect("Index should be valid");
        let cp0 = curve.cp0.borrow();
        let cp1 = curve.cp1.borrow();
        let p1 = curve.p1.borrow();
          
        return (prev_coord, cp0, cp1, p1);
    }

    pub fn push_coord(&mut self,  cp0: Rc<RefCell<Coord>>, cp1: Rc<RefCell<Coord>>, p1: Rc<RefCell<Coord>>) {
        self.curves.push(Curve::new(cp0, cp1, p1));
    }

    pub fn get_cp_of_shape(&self) -> Vec<RefCoordType> {
        let mut vec = Vec::new();
        for (curve_index, curve) in  self.curves.iter().enumerate() {
            vec.push(RefCoordType::Cp0(curve_index, curve.cp0.borrow()));
            vec.push(RefCoordType::Cp1(curve_index, curve.cp1.borrow()));
        }
        vec
    }

    pub fn get_p_of_shape(&self) -> Vec<Ref<Coord>> {
        let mut vec = Vec::new();
        vec.push(self.start.borrow());
        for curve in  self.curves.iter() {
            vec.push(curve.p1.borrow());
        }
        vec
    }

    pub fn get_coords_of_shape(&self) -> Vec<Ref<Coord>> {
        let mut vec = Vec::new();
        vec.push(self.start.borrow());
        for curve in  self.curves.iter(){
            vec.push(curve.cp0.borrow());
            vec.push(curve.cp1.borrow());
            vec.push(curve.p1.borrow());
        }
        vec
    }
}




#[derive(Debug)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<[u8;4]> for Rgba{
    fn from(value: [u8;4]) -> Self {
        Rgba{
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }
}
