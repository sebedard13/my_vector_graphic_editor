use std::cell::{Ref, RefCell};
use std::rc::Rc;
use common::types::Coord;

pub type CoordPtr = Rc<RefCell<Coord>>;


pub enum RefCoordType<'a> {
    Start(Ref<'a, Coord>),
    /// Curve index, coord
    Cp0(usize, Ref<'a, Coord>),
    /// Curve index, coord
    Cp1(usize, Ref<'a, Coord>),
    /// Curve index, coord
    P1(usize, Ref<'a, Coord>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoordType {
    Start,
    Cp0(usize),
    /// Curve index
    Cp1(usize),
    /// Curve index
    P1(usize),
}

impl RefCoordType<'_> {
    pub fn get_coord(&self) -> &Coord {
        match self {
            RefCoordType::Start(coord) => coord,
            RefCoordType::Cp0(_, coord) => coord,
            RefCoordType::Cp1(_, coord) => coord,
            RefCoordType::P1(_, coord) => coord,
        }
    }

    pub fn to_coord_type(&self) -> CoordType {
        match self {
            RefCoordType::Start(_) => CoordType::Start,
            RefCoordType::Cp0(index, _) => CoordType::Cp0(*index),
            RefCoordType::Cp1(index, _) => CoordType::Cp1(*index),
            RefCoordType::P1(index, _) => CoordType::P1(*index),
        }
    }
}