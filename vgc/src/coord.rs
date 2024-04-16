use common::types::Coord;
use common::Rgba;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub type CoordPtr = Rc<RefCell<Coord>>;

static ID_COUNTER: usize = 0;
pub struct CoordRef{
    pub id: usize,
    pub coord: Coord,
}

pub struct ShapeRef {
    pub id: usize,
    pub start: usize,
    pub curves: Vec<usize>,
    pub color: Rgba,
}

pub struct CurveRef {
    pub id: usize,
    pub cp0: usize,
    pub cp1: usize,
    pub p1: usize,
}

pub struct CurveSolid{
    pub id: usize,
    pub cp0: CoordRef,
    pub cp1: CoordRef,
    pub p1: CoordRef,
}

pub struct ShapeSolid {
    pub id: usize,
    pub start: CoordRef,
    pub curves: Vec<CurveSolid>,
    pub color: Rgba,
}

pub struct VgcDataBase{
    pub background: Rgba,
    pub shapes: Vec<ShapeRef>,
    pub coords: Vec<CoordRef>,
    pub curves: Vec<CurveRef>,
}

impl VgcDataBase{

    pub fn get_shapeSolid(&self, index: usize) -> Option<ShapeSolid>{
        if let Some(shape) = self.shapes.iter().find(|s| s.id == index){
            let mut curves = Vec::new();
            for curve_index in &shape.curves{
                if let Some(curve) = self.get_curveSolid(*curve_index){
                    curves.push(curve);
                }
            }
            if let Some(start) = self.get_coordSolid(shape.start){
                return Some(ShapeSolid{
                    id: shape.id,
                    start,
                    curves,
                    color: shape.color.clone(),
                });
            }
        }
        None
    }

    pub fn get_curveSolid(&self, index: usize) -> Option<CurveSolid>{
        if let Some(curve) = self.curves.iter().find(|c| c.id == index){
            if let Some(cp0) = self.get_coordSolid(curve.cp0){
                if let Some(cp1) = self.get_coordSolid(curve.cp1){
                    if let Some(p1) = self.get_coordSolid(curve.p1){
                        return Some(CurveSolid{
                            id: curve.id,
                            cp0,
                            cp1,
                            p1,
                        });
                    }
                }
            }
        }
        None
    }

    pub fn get_coordSolid(&self, index: usize) -> Option<CoordRef>{
        if let Some(coord) = self.coords.iter().find(|c| c.id == index){
            return Some(CoordRef{
                id: coord.id,
                coord: coord.coord,
            });
        }
        None
    }
}