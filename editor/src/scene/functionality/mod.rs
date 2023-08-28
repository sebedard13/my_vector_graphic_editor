use self::move_coord::MoveCoord;

use super::{MsgScene, Scene};

mod create_update_shape;
mod move_coord;

#[derive(Debug, Clone, Default)]
pub enum Functionality {
    MoveCoord(MoveCoord),

    SeparateHandle,

    CreateOrAddPoint,
    CreateNextPoint,
    RemovePoint,

    #[default]
    None,
}

impl Functionality {
    #[allow(non_snake_case)]
    pub fn MoveCoord_default() -> Functionality {
        Functionality::MoveCoord(MoveCoord::new())
    }

    #[allow(non_snake_case)]
    pub fn MoveHandle_default() -> Functionality {
        Functionality::SeparateHandle
    }

    #[allow(non_snake_case)]
    pub fn CreateOrAddPoint_default() -> Functionality {
        Functionality::CreateOrAddPoint
    }
}

pub fn match_functionality(scene: &mut Scene, event: &MsgScene) {
    match &mut scene.functionality {
        Functionality::MoveCoord(move_coord) => {
            move_coord::handle_move(event, move_coord, &mut scene.camera, &scene.selected)
        }
        Functionality::SeparateHandle => {
            move_coord::handle_seprate_handle(event, &mut scene.camera, &mut scene.vgc_data);
        }
        Functionality::CreateOrAddPoint => {
            create_update_shape::handle_create_or_add_point(
                event,
                &mut scene.camera,
                &mut scene.vgc_data,
                &scene.selected,
            );
        }
        _ => {}
    }
}
