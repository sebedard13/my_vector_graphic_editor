use iced::{
    widget::canvas::{Event, Fill, Frame, Path},
    Color, Point,
};

use crate::scene::{point_in_radius, Scene, SceneOverlay};

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
            index_selected_coord: 999,
        }
    }
}

impl SceneOverlay for SelectedShape {
    type T = SelectedShapeEvent;

    fn draw(&self, frame: &mut Frame, scene: &Scene) {
        // Render points
        let coords = scene.vgc_data.list_coord();
        for coord in coords {
            let color = match self.index_selected_coord == coord.i {
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

    fn handle_event(
        &self,
        scene: &Scene,
        _event: Event,
        cursor_position: Option<Point>,
    ) -> (iced::event::Status, Option<Self::T>) {
        let coords = scene.vgc_data.list_coord();
        for coord in coords {
            match cursor_position {
                Some(p) => {
                    if point_in_radius(
                        &scene.camera.project(p),
                        &Point::new(coord.coord.x, coord.coord.y),
                        scene.camera.fixed_length(12.0),
                    ) {
                        return (
                            iced::event::Status::Captured,
                            Some(SelectedShapeEvent::HoverCoord(coord.i)),
                        );
                    } else {
                    }
                }
                None => {}
            }
        }

        return (
            iced::event::Status::Captured,
            Some(SelectedShapeEvent::HoverCoord(9999)),
        );
    }

    fn update(&mut self, msg: Self::T) {
        match msg {
            SelectedShapeEvent::HoverCoord(index) => self.index_selected_coord = index,
        }
    }
}
