use self::move_coord::MoveCoord;

use super::{Scene, MsgScene};

mod move_coord;

#[derive(Debug, Clone)]
pub enum Functionality {
    MoveCoord(MoveCoord),
    SeparateHandle,
    None,
}
impl Default for Functionality {
    fn default() -> Functionality {
        Functionality::None
    }
}

impl Functionality {
    pub fn MoveCoord_default() -> Functionality {
        Functionality::MoveCoord(MoveCoord::new())
    }
}



pub fn match_functionality(scene: &mut Scene, event: &MsgScene) {
    match &mut scene.functionality {
        Functionality::MoveCoord(move_coord) => move_coord::handle_move(event, move_coord, &mut scene.camera,&mut scene.vgc_data),
        Functionality::SeparateHandle => {}
        Functionality::None => {}
    }
}