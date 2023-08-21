use serde::{Deserialize, Serialize};
use std::mem::swap;

use crate::coord::{CoordDS, CoordIndex};
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
