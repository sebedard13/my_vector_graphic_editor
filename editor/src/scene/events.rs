use std::collections::HashMap;

use iced::{widget::canvas::Event, Point, mouse};



#[derive(Default)]
pub struct EventsMerger{
    past_events: HashMap<String, Event>

}

enum MergeEvent{
    Click(Click),
    MouseMove(MouseMove)
}

pub struct MouseMove{
    pub start: Point,
    pub button :  mouse::Button,
    pub movement : Point,
}

pub struct Click{
    pub start: Point,
    pub button : mouse::Button,
}

pub enum EventStatus{
    Used(Option<MergeEvent>),
    Free
}

impl EventsMerger{

    pub fn push_event(&mut self, event :Event)->EventStatus{

        match event{
            Event::Mouse(mouse_event) => self.match_mouse_event(mouse_event),
            Event::Touch(touch_event) =>  EventStatus::Free,
            Event::Keyboard(key_board) =>  EventStatus::Free,
        }
    }


    fn match_mouse_event(&mut self, event :mouse::Event) ->EventStatus {

        match event{
            mouse::Event::CursorEntered => EventStatus::Free,
            mouse::Event::CursorLeft => EventStatus::Free,
            mouse::Event::CursorMoved { position } => todo!(),
            mouse::Event::ButtonPressed(mouse_button) => EventStatus::Free,
            mouse::Event::ButtonReleased(mouse_button) => EventStatus::Free,
            mouse::Event::WheelScrolled { delta } => EventStatus::Free,
        }

        
    }
}

