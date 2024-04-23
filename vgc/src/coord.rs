use common::types::Coord;

use std::cell::RefCell;

use std::rc::Rc;

pub type CoordPtr = Rc<RefCell<Coord>>;

pub fn coordptr_new(x: f32, y: f32) -> CoordPtr {
    Rc::new(RefCell::new(Coord::new(x, y)))
}
