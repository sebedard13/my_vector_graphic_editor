use iced::Point;

use super::{point_in_radius, MsgScene, Scene};

pub struct MoveCoord {
    id_point: Option<usize>,
}

impl MoveCoord {
    pub fn new() -> Self {
        Self { id_point: None }
    }
}

pub fn handle_move(scene: &mut Scene, event: &MsgScene) {
    match event {
        MsgScene::DragMain(pressmove) => {
            let coords = &scene.vgc_data.list_coord();
            println!("Not handle");
            let posi = coords.iter().position(|coord| -> bool {
                let point = &scene.camera.project(pressmove.current_coord);
                let v =point_in_radius(
                    point,
                    &Point::new(coord.coord.x, coord.coord.y),
                    scene.camera.fixed_length(12.0),
                );
                let v2 = v;
                v2
            });

            if let Some(index) = posi {
                println!("Not handle2");
                let index = coords[index].i;
                let point = &scene.camera.project(pressmove.current_coord);
                scene.vgc_data.move_coord(
                    index,
                    point.x,
                    point.y,
                );
            };
        }
        _ => {}
    }
}