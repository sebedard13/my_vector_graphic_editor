use iced::{Point, widget::canvas::Event, mouse::{Cursor, self}, Rectangle, event};

use crate::grid::{Grid, point_in_radius, MsgGrid};



pub struct MoveCoord{
    id_point:Option<usize>
}

#[derive(Debug, Clone)]
pub enum MoveCoordStep{
    Click(Point, usize),
    Drag(Point),
    Release,
}

impl MoveCoord {

    pub fn new() -> Self {
        Self {
            id_point: None,
        }
    }


    pub fn update(grid:&mut Grid, msg: MoveCoordStep){
        match msg{
            MoveCoordStep::Click(_, id) => {
                grid.move_coord.id_point = Some(id);
            },
            MoveCoordStep::Drag(pt) => {
                match grid.move_coord.id_point{
                    Some(id) => {
                        grid.vgc_data.move_coord(id, pt.x, pt.y);
                    },
                    None => {}
                }
            },
            MoveCoordStep::Release => grid.move_coord.id_point = None,
        }
    }


    pub fn handle_event(grid:&Grid,  event: Event, cursor_position: Point, cursor: Cursor, bounds: Rectangle) -> (iced::event::Status, Option<MsgGrid>){
        match grid.move_coord.id_point {
            Some(_)=>{
                match event {
                    Event::Mouse(mouse_event) => {
                        match mouse_event{
                            mouse::Event::ButtonReleased(mouse::Button::Left) => {
                                return (event::Status::Captured, Some(MsgGrid::MoveCoord(MoveCoordStep::Release)));          
                            }
                            mouse::Event::CursorMoved { .. } => {
                                let pt = grid.camera.project(cursor_position, bounds.size());
                                return (event::Status::Captured, Some(MsgGrid::MoveCoord(MoveCoordStep::Drag(pt))));
                            }
                            _ => {}
                        }  
                    }
                    _ => {},
                }
            }
            None=>{
                match event {
                    Event::Mouse(mouse_event) => {
                        match mouse_event{
                            mouse::Event::ButtonPressed(mouse::Button::Left) => {
                                let coords = grid.vgc_data.list_coord();
                                for coord in coords {
                                    match cursor.position_in(bounds) {
                                        Some(p) => {
                                            if point_in_radius(&grid.camera.project(p, bounds.size()), 
                                                &Point::new(coord.coord.x, coord.coord.y), grid.camera.fixed_length(12.0)) {
                                                    let pt = grid.camera.project(p, bounds.size());
                                                    return (event::Status::Captured, Some(MsgGrid::MoveCoord(MoveCoordStep::Click(pt, coord.i))));
                                            }
                                        },
                                        None => {}
                                    }
                                }
                
                                return (event::Status::Ignored, None);
                            }
                            _ => {}
                    }
                    }
                    _ => {}
                }
            }
        }

        (event::Status::Ignored, None)
    }
}