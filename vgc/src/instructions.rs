use crate::coord::Coord;
use crate::vcg_struct::RGBA;

pub struct ShapeInstruction {
    pub start: Coord,
    pub curves: Vec<CurveInstruction>,
    pub color: RGBA,
}

#[derive(Clone)]
pub struct CurveInstruction {
    pub c1: Coord,
    pub c2: Coord,
    pub p: Coord,
}