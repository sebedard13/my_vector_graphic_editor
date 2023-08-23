use serde::{Deserialize, Serialize};
use std::mem::swap;

use crate::coord::{CoordDS, CoordIndex, Coord};
use crate::curve;
use crate::curve::Curve;

#[derive(Deserialize, Serialize, Debug)]
pub struct Shape {
    pub start: CoordIndex,
    pub curves: Vec<Curve>,
    pub color: Rgba,
}

impl Shape {
    pub fn add_coord(&mut self, coord_ds: &mut CoordDS, mut curve: Curve, index: usize) {
        let curve_after = self.curves.get_mut(index).expect("Index should be valid because we should not be able to add a curve at the end of the shape because the last element close the curve with a link to the start coord in shape");

        swap(&mut curve.cp0, &mut curve.cp1);
        swap(&mut curve.cp0, &mut curve_after.cp0);
        self.curves.insert(index, curve);
    }

    pub fn toggle_separate_join_handle(&mut self, coord_ds: &mut CoordDS, index: usize) {
        if self.is_handles_joined(index) {
            self.separate_handle(coord_ds, index);
        } else {
            self.join_handle(coord_ds, index);
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
            });//TODO: clone is not good
        }
    }

    pub fn separate_handle(&mut self, coord_ds: &mut CoordDS, index: usize) {
        //Todo check if index is not the last curve and what not
        let p0 = {
            if index == 0 {
                coord_ds.get(&self.start)
            } else {
                coord_ds.get(&self.curves[index - 1].p1)
            }
        };
        let current_curve = &self.curves[index];
        let cp0 = coord_ds.get(&current_curve.cp0);
        let cp1 = coord_ds.get(&current_curve.cp1);
        let p1 = coord_ds.get(&current_curve.p1);

        let next_curve = &self.curves[(index + 1)% self.curves.len()];
        let cp2 = coord_ds.get(&next_curve.cp0);
        let cp3 = coord_ds.get(&next_curve.cp1);
        let p2 = coord_ds.get(&next_curve.p1);

        let coords_separate = curve::tangent_cornor_pts(p0, cp0, cp1, p1, cp2, cp3, p2);

        let coord_index0 = coord_ds.insert(coords_separate[0].clone()); //TODO clone not good
        let coord_index1 = coord_ds.insert(coords_separate[1].clone());

        self.curves[index].cp1 = coord_index0;
        let len = self.curves.len();
        self.curves[(index + 1) % len].cp0 = coord_index1;
    }

    pub fn join_cp0_handle(&mut self, coord_ds: &mut CoordDS, index: usize) {
        let coord_index = &self.curves[index].p1;
        let curve_after = (index + 1) % self.curves.len();
        let coord_index_to_remove = &self.curves[curve_after].cp0;
        coord_ds.remove(coord_index_to_remove);
        
        self.curves[curve_after].cp0 = coord_index.clone();
    }

    pub fn join_cp1_handle(&mut self, coord_ds: &mut CoordDS, index: usize) {
        let coord_index = &self.curves[index].p1;
        let coord_index_to_remove = &self.curves[index].cp1;
        coord_ds.remove(coord_index_to_remove);
    
        self.curves[index].cp1 = coord_index.clone();
    }

    pub fn join_handle(&mut self, coord_ds: &mut CoordDS, index: usize) {
        self.join_cp0_handle(coord_ds, index);
        self.join_cp1_handle(coord_ds, index);
    }

    pub fn to_path(&self, coord_ds: &CoordDS) -> String {
        let mut path = String::new();
        let start = coord_ds.get(&self.start);
        path.push_str(&format!("M {} {}", start.x, start.y));
        for curve in &self.curves {
            let cp0 = coord_ds.get(&curve.cp0);
            let cp1 = coord_ds.get(&curve.cp1);
            let p1 = coord_ds.get(&curve.p1);
            path.push_str(&format!(
                " C {} {} {} {} {} {}",
                cp0.x, cp0.y, cp1.x, cp1.y, p1.x, p1.y
            ));
        }
        path.push_str(" Z");
        path
    }


    /// Visit each curve of the shape and call the visitor function with the curve index and 4 coords of the curve so p0, cp0, cp1, p1
    pub fn visit_full_curves(&self, coord_ds: &CoordDS, mut visitor: impl FnMut(usize, &Coord, &Coord, &Coord, &Coord)) {
        let start = coord_ds.get(&self.start);
        let mut prev_coord = start;
        for (index,curve) in self.curves.iter().enumerate() {
            let cp0 = coord_ds.get(&curve.cp0);
            let cp1 = coord_ds.get(&curve.cp1);
            let p1 = coord_ds.get(&curve.p1);
          
            visitor(index, prev_coord, cp0, cp1, p1);

            prev_coord = p1;
        }
    }


    pub fn closest_curve(&self,coord_ds: &CoordDS, coord: &Coord)-> (usize, f32,f32, Coord){
        let mut min_distance = std::f32::MAX;
        let mut min_index = 0;
        let mut min_t = 0.0;
        let mut min_coord = coord_ds.get(&self.start).clone();

        self.visit_full_curves(coord_ds, |curve_index, p0, cp0, cp1, p1|{
            let (t_min, distance, coord) = curve::t_closest(&coord, p0, cp0, cp1, p1);

            if distance < min_distance{
                min_distance = distance;
                min_index = curve_index;
                min_t = t_min;
                min_coord = coord;
            }
        });
        (min_index, min_t, min_distance, min_coord)
    }

    pub fn get_coords_of_curve<'a>(&self,coord_ds: &'a CoordDS, curve_index:usize)->(&'a Coord, &'a Coord, &'a Coord, &'a Coord){
        let mut prev_coord = coord_ds.get(&self.start);

        if curve_index > 0{
            let prev_curve = self.curves.get(curve_index - 1).expect("Index should be valid");
            prev_coord = coord_ds.get(&prev_curve.p1);
        }
        let curve = self.curves.get(curve_index).expect("Index should be valid");
        let cp0 = coord_ds.get(&curve.cp0);
        let cp1 = coord_ds.get(&curve.cp1);
        let p1 = coord_ds.get(&curve.p1);
          
        return (prev_coord, cp0, cp1, p1);
    }

    pub fn closest_coord_on(&self,coord_ds: &CoordDS, coord: Coord)-> Coord{
    
        let  (_,_,_,closest_coord) = self.closest_curve(coord_ds, &coord);
        closest_coord
    }

    pub fn push_coord(&mut self, coord_ds: &mut CoordDS, cp0: Coord, cp1: Coord, p1: Coord) {
        let cp0 = coord_ds.insert(cp0);
        let cp1 = coord_ds.insert(cp1);
        let p1 = coord_ds.insert(p1);
        self.curves.push(Curve {
            cp0,
            cp1,
            p1,
        });
    }
}




#[derive(Deserialize, Serialize, Debug)]
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
