use iced::{mouse, widget::canvas::Event, Point};
use itertools::Itertools;

#[derive(Default)]
pub struct EventsMerger {
    past_events: Vec<MergeEvent>,
}

/// Custom event for canvas scene.
/// This event is used to merge mouse events to create Pressmove with the button pressed. this could be used to create a drag system.
/// It should handle to create key combo but it is not implemented yet.
#[derive(Debug, Clone, PartialEq)]
pub enum MergeEvent {
    Click(Click), //Can be used as pressup
    Pressmove(Pressmove),
    Mousedown(Mousedown),
    Scroll(Scroll),
    Mousemove(Mousemove)

}

#[derive(Debug, Clone, PartialEq)]
pub enum EventStatus {
    Used(Option<MergeEvent>),
    Free,
}

impl EventsMerger {
    pub fn push_event(&mut self, cursor_position: Option<Point>, event: Event) -> EventStatus {
        match event {
            Event::Mouse(mouse_event) => self.match_mouse_event(cursor_position, mouse_event),
            Event::Touch(_) => EventStatus::Free,
            Event::Keyboard(_) => {
                  /*let message = match key_code {
                    keyboard::KeyCode::PageUp => Some(MsgScene::Scaled(
                        (self.scaling * 1.1).clamp(Self::MIN_SCALING, Self::MAX_SCALING),
                        None,
                    )),
                    keyboard::KeyCode::PageDown => Some(MsgScene::Scaled(
                        (self.scaling / 1.1).clamp(Self::MIN_SCALING, Self::MAX_SCALING),
                        None,
                    )),
                    keyboard::KeyCode::Home => Some(MsgScene::Scaled(1.0, Some(self.home))),
                    _ => None,
                };*/
                
                EventStatus::Free
            },
        }
    }

    pub fn match_mouse_event(
        &mut self,
        cursor_position: Option<Point>,
        event: mouse::Event,
    ) -> EventStatus {
        match event {
            mouse::Event::CursorEntered => EventStatus::Free,
            mouse::Event::CursorLeft => EventStatus::Free,
            mouse::Event::CursorMoved { position } => {
                let valid_event = self
                    .past_events
                    .iter()
                    .filter(|event| match event {
                        MergeEvent::Mousedown { .. } => true,
                        _ => false,
                    })
                    .collect_vec();

                match valid_event.first() {
                    Some(event) => match event {
                        MergeEvent::Mousedown(press) => {
                            EventStatus::Used(Some(MergeEvent::Pressmove(Pressmove {
                                start: press.start_press.clone(),
                                button: press.button.clone(),
                                current_coord: position,
                            })))
                        }
                        _ => EventStatus::Free,
                    },
                    None => {
                        EventStatus::Used(Some(MergeEvent::Mousemove(Mousemove{
                            current_coord: position,
                        })))
                    },
                }
            }
            mouse::Event::ButtonPressed(mouse_button) => {
                if let Some(cursor_position) = cursor_position {
                    let v = MergeEvent::Mousedown(Mousedown {
                        start_press: cursor_position,
                        button: mouse_button.clone(),
                    });
                    self.past_events.push(v.clone());
                    EventStatus::Used(Some(v))
                }
                else{
                    EventStatus::Used(None)
                }
            }
            mouse::Event::ButtonReleased(mouse_button) => {
                let end_press = match cursor_position {
                    Some(cursor_position) => cursor_position,
                    None => return EventStatus::Free,
                };

                let valid_event = self
                    .past_events
                    .iter()
                    .filter(|event| match event {
                        MergeEvent::Mousedown(press) if mouse_button == press.button => true,
                        _ => false,
                    })
                    .collect_vec();

                let rtn = match valid_event.first() {
                    Some(event) => {
                        let start_press = match event {
                            MergeEvent::Mousedown(press) => press.start_press,
                            _ => return EventStatus::Free,
                        };

                        EventStatus::Used(Some(MergeEvent::Click(Click {
                            start_press: start_press.clone(),
                            end_press,
                            button: mouse_button.clone(),
                        })))
                    }
                    None => return EventStatus::Free,
                };

                self.past_events.retain(|event| match event {
                    MergeEvent::Mousedown(press) if mouse_button == press.button => false,
                    _ => true,
                });
                rtn
            }
            mouse::Event::WheelScrolled { delta } => {
                if let Some(cursor_position) = cursor_position {
                    EventStatus::Used(Some(MergeEvent::Scroll(Scroll {
                        coord: cursor_position,
                        movement_type: delta,
                        movement: match delta {
                            mouse::ScrollDelta::Lines { x, y } => Point::new(x, y),
                            mouse::ScrollDelta::Pixels { x, y } => Point::new(x, y),
                        },
                    })))
                } else {
                    EventStatus::Free
                }
            }
        }
    }
}

//Insert rust test mod here with no exemple
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic_click_event() {
        let mut events_merger = EventsMerger::default();

        let mut cursor_position = Point::new(10.0, 20.0);
        let event = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        let event_status = events_merger.push_event(Some(cursor_position), event);
        assert_eq!(event_status, EventStatus::Used(None));

        let event = Event::Mouse(mouse::Event::CursorMoved {
            position: Point::new(1.0, 1.0),
        });
        let event_status = events_merger.push_event(Some(cursor_position), event);
        assert_eq!(
            event_status,
            EventStatus::Used(Some(MergeEvent::Pressmove(Pressmove {
                start: cursor_position,
                button: mouse::Button::Left,
                current_coord: Point::new(1.0, 1.0)
            })))
        );

        cursor_position = Point::new(1.0, 1.0);
        let event = Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
        let event_status = events_merger.push_event(Some(cursor_position), event);
        assert_eq!(
            event_status,
            EventStatus::Used(Some(MergeEvent::Click(Click {
                start_press: Point::new(10.0, 20.0),
                end_press: cursor_position,
                button: mouse::Button::Left
            })))
        );

        assert!(events_merger.past_events.is_empty());
    }

    #[test]
    fn left_and_right_click_at_same() {
        let mut events_merger = EventsMerger::default();

        let cursor_position = Point::new(10.0, 20.0);
        let event = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        let event_status = events_merger.push_event(Some(cursor_position), event);
        assert_eq!(event_status, EventStatus::Used(None));

        let cursor_position = Point::new(1.0, 2.0);
        let event = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right));
        let event_status = events_merger.push_event(Some(cursor_position), event);
        assert_eq!(event_status, EventStatus::Used(None));

        let cursor_position = Point::new(11.0, 21.0);
        let event = Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
        let event_status = events_merger.push_event(Some(cursor_position), event);
        assert_eq!(
            event_status,
            EventStatus::Used(Some(MergeEvent::Click(Click {
                start_press: Point::new(10.0, 20.0),
                end_press: cursor_position,
                button: mouse::Button::Left
            })))
        );

        assert!(!events_merger.past_events.is_empty());

        let cursor_position = Point::new(2.0, 1.0);
        let event = Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right));
        let event_status = events_merger.push_event(Some(cursor_position), event);
        assert_eq!(
            event_status,
            EventStatus::Used(Some(MergeEvent::Click(Click {
                start_press: Point::new(1.0, 2.0),
                end_press: cursor_position,
                button: mouse::Button::Right
            })))
        );

        assert!(events_merger.past_events.is_empty());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scroll {
    pub coord: Point,
    pub movement_type: mouse::ScrollDelta, //Separate value from type line or pixel because we can decide to ignore them
    pub movement: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mousedown {
    pub start_press: Point,
    pub button: mouse::Button,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Pressmove {
    pub start: Point,
    pub button: mouse::Button,
    pub current_coord: Point,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Click {
    pub start_press: Point,
    pub end_press: Point,
    pub button: mouse::Button,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mousemove{
    pub current_coord: Point,
}