use iced::{
    widget::canvas::{Event, Fill, Frame, Path},
    Color, Point,
};

use crate::scene::{point_in_radius, Scene};

use super::MsgScene;

pub struct SelectedShape {
    index_selected_coord: usize,
}

#[derive(Debug, Clone)]
pub enum SelectedShapeEvent {
    HoverCoord(usize),
}

impl Default for SelectedShape {
    fn default() -> Self {
        Self {
            index_selected_coord: 9999,
        }
    }
}

pub fn draw(scene: &Scene, frame: &mut Frame) {
    // Render points
    let coords = scene.vgc_data.list_coord();
    for coord in coords {
        let color = match scene.selected_shape.index_selected_coord == coord.i {
            true => Color::from_rgb8(0x0E, 0x90, 0xAA),
            false => Color::from_rgb8(0x3A, 0xD1, 0xEF),
        };

        let center = Point::new(
            coord.coord.x,
            coord.coord.y * 1.0 / scene.vgc_data.ratio as f32,
        );
        frame.fill(
            &Path::circle(center, scene.camera.fixed_length(5.0)),
            Fill::from(color),
        );
    }
}

/* 
pub fn handle_event(
    scene: &Scene,
    _event: Event,
    cursor_position: Point,
) -> (iced::event::Status, Option<MsgScene>) {
    let coords = scene.vgc_data.list_coord();
    for coord in coords {
        if point_in_radius(
            &scene.camera.project(cursor_position),
            &Point::new(coord.coord.x, coord.coord.y),
            scene.camera.fixed_length(12.0),
        ) {
            return (
                iced::event::Status::Captured,
                Some(MsgScene::HoverCoord(SelectedShapeEvent::HoverCoord(coord.i))),
            );
        }       
    }

    if scene.selected_shape.index_selected_coord != 9999 {
        return (
            iced::event::Status::Captured,
            Some(MsgScene::HoverCoord(SelectedShapeEvent::HoverCoord(9999))),
        );
    }

    (iced::event::Status::Ignored, None)
}

pub fn update(scene: &mut Scene, msg: SelectedShapeEvent) {
    match msg {
        SelectedShapeEvent::HoverCoord(index) => scene.selected_shape.index_selected_coord = index,
    }
}*/