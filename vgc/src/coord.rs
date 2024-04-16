use common::types::Coord;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub type CoordPtr = Rc<RefCell<Coord>>;
