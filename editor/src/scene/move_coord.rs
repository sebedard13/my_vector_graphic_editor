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
        MsgScene::ClickMain(_)=>{
            scene.move_coord.id_point = None;
        },
        MsgScene::MousedownMain(mousedown)=>{
            let coords = &scene.vgc_data.list_coord();
            let posi = coords.iter().position(|coord| -> bool {
                let point = &scene.camera.project(mousedown.start_press);
                point_in_radius(
                    point,
                    &Point::new(coord.coord.x, coord.coord.y),
                    scene.camera.fixed_length(12.0),
                )
            });
            scene.move_coord.id_point = posi;
        },
        MsgScene::DragMain(pressmove) => {
            let coords = &scene.vgc_data.list_coord();
            if let Some(index) = scene.move_coord.id_point {
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