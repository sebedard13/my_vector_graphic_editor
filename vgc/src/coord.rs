use common::types::Coord;

use std::cell::RefCell;

use std::rc::Rc;

pub type CoordPtr = Rc<RefCell<Coord>>;
