use crate::coord::Coord;
use crate::vcg_struct::Rgba;

pub struct ShapeInstruction {
    pub start: Coord,
    pub curves: Vec<CurveInstruction>,
    pub color: Rgba,
}

#[derive(Clone)]
pub struct CurveInstruction {
    //c1 become c1 in curve after
    pub c1: Coord, // become c2 in current curve
    pub p: Coord,  // stay point
    pub c2: Coord, // become c1 in curve after
}

pub struct AddCurve {
    pub curve: CurveInstruction,
    pub index_shape: usize,
    pub index_curve: usize,
}

#[derive(Debug)]
pub struct CoordWithIndex<'a> {
    pub coord: &'a Coord,
    pub i: usize,
}
