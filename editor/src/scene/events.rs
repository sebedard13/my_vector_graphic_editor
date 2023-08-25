#![allow(clippy::single_match)]
use iced::{keyboard, mouse, widget::canvas::Event, Point};
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
    Mousemove(Mousemove),
    KeysDown(KeysChange),
    KeysUp(KeysChange),
    KeyDown(keyboard::KeyCode),
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
            Event::Touch(_) => EventStatus::Free, //TODO: Handle touch event, maybe like mouse event
            Event::Keyboard(keyboard_event) => {
                self.match_keyboard_event(cursor_position, keyboard_event)
            }
        }
    }

    pub fn match_keyboard_event(
        &mut self,
        cursor_position: Option<Point>,
        event: keyboard::Event,
    ) -> EventStatus {
        match event {
            keyboard::Event::KeyPressed {
                key_code,
                modifiers,
            } => {
                let active_keys = self.get_all_keydown(modifiers);

                let keydown = MergeEvent::KeyDown(key_code);
                self.past_events.push(keydown.clone());

                EventStatus::Used(Some(MergeEvent::KeysDown(KeysChange {
                    current_coord: cursor_position,
                    new_keys: key_code,
                    active_keys,
                })))
            }
            keyboard::Event::KeyReleased {
                key_code,
                modifiers,
            } => {
                self.past_events.retain(
                    |event| !matches!(event, MergeEvent::KeyDown(keydown) if key_code == *keydown),
                );

                let active_keys = self.get_all_keydown(modifiers);

                EventStatus::Used(Some(MergeEvent::KeysUp(KeysChange {
                    current_coord: cursor_position,
                    new_keys: key_code,
                    active_keys,
                })))
            }
            _ => EventStatus::Free,
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
                    .filter(|event| matches!(event, MergeEvent::Mousedown { .. }))
                    .collect_vec();

                match valid_event.first() {
                    Some(event) => match event {
                        MergeEvent::Mousedown(press) => {
                            EventStatus::Used(Some(MergeEvent::Pressmove(Pressmove {
                                start: press.start_press,
                                button: press.button,
                                current_coord: position,
                            })))
                        }
                        _ => EventStatus::Free,
                    },
                    None => EventStatus::Used(Some(MergeEvent::Mousemove(Mousemove {
                        current_coord: position,
                    }))),
                }
            }
            mouse::Event::ButtonPressed(mouse_button) => {
                if let Some(cursor_position) = cursor_position {
                    let v = MergeEvent::Mousedown(Mousedown {
                        start_press: cursor_position,
                        button: mouse_button,
                    });
                    self.past_events.push(v.clone());
                    EventStatus::Used(Some(v))
                } else {
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
                    .filter(|event| {
                        matches!(event,
                        MergeEvent::Mousedown(press) if mouse_button == press.button)
                    })
                    .collect_vec();

                let rtn = match valid_event.first() {
                    Some(event) => {
                        let start_press = match event {
                            MergeEvent::Mousedown(press) => press.start_press,
                            _ => return EventStatus::Free,
                        };

                        EventStatus::Used(Some(MergeEvent::Click(Click {
                            start_press,
                            end_press,
                            button: mouse_button,
                        })))
                    }
                    None => return EventStatus::Free,
                };

                self.past_events.retain(|event| {
                    !matches!(event,
                    MergeEvent::Mousedown(press) if mouse_button == press.button)
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

    #[allow(clippy::single_match)]
    fn get_all_keydown(&self, _: keyboard::Modifiers) -> Vec<keyboard::KeyCode> {
        //TODO: Use modifiers to get all active keys with valid control, shift, etc
        let mut active_keys = vec![];
        for event in &self.past_events {
            match event {
                MergeEvent::KeyDown(keydown) => {
                    active_keys.push(*keydown);
                }
                _ => {}
            }
        }
        active_keys
    }
}

//Insert rust test mod here with no exemple
#[cfg(test)]
mod tests {

    use iced::keyboard::{KeyCode, Modifiers};

    use super::*;

    #[test]
    fn basic_click_event() {
        let mut events_merger = EventsMerger::default();

        let mut cursor_position = Point::new(10.0, 20.0);
        let event = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        let event_status = events_merger.push_event(Some(cursor_position), event);
        assert_eq!(
            event_status,
            EventStatus::Used(Some(MergeEvent::Mousedown(Mousedown {
                start_press: cursor_position,
                button: mouse::Button::Left
            })))
        );

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
        assert_eq!(
            event_status,
            EventStatus::Used(Some(MergeEvent::Mousedown(Mousedown {
                start_press: cursor_position,
                button: mouse::Button::Left
            })))
        );

        let cursor_position = Point::new(1.0, 2.0);
        let event = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right));
        let event_status = events_merger.push_event(Some(cursor_position), event);
        assert_eq!(
            event_status,
            EventStatus::Used(Some(MergeEvent::Mousedown(Mousedown {
                start_press: cursor_position,
                button: mouse::Button::Right
            })))
        );

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

    #[test]
    fn basic_keyboard_copy_paste() {
        let mut events_merger = EventsMerger::default();

        //Not using modifiers for now, maybe iced don't send LControl
        let event = Event::Keyboard(keyboard::Event::KeyPressed {
            key_code: KeyCode::LControl,
            modifiers: Modifiers::empty(),
        });
        let event_status = events_merger.push_event(None, event);
        assert_eq!(
            event_status,
            EventStatus::Used(Some(MergeEvent::KeysDown(KeysChange {
                current_coord: None,
                new_keys: KeyCode::LControl,
                active_keys: vec![],
            })))
        );

        let event = Event::Keyboard(keyboard::Event::KeyPressed {
            key_code: KeyCode::C,
            modifiers: Modifiers::empty(),
        });
        let event_status = events_merger.push_event(None, event);
        assert_eq!(
            event_status,
            EventStatus::Used(Some(MergeEvent::KeysDown(KeysChange {
                current_coord: None,
                new_keys: KeyCode::C,
                active_keys: vec![KeyCode::LControl],
            })))
        );

        let event = Event::Keyboard(keyboard::Event::KeyReleased {
            key_code: KeyCode::C,
            modifiers: Modifiers::empty(),
        });
        let event_status = events_merger.push_event(None, event);
        assert_eq!(
            event_status,
            EventStatus::Used(Some(MergeEvent::KeysUp(KeysChange {
                current_coord: None,
                new_keys: KeyCode::C,
                active_keys: vec![KeyCode::LControl],
            })))
        );

        let event = Event::Keyboard(keyboard::Event::KeyPressed {
            key_code: KeyCode::V,
            modifiers: Modifiers::empty(),
        });
        let event_status = events_merger.push_event(None, event);
        assert_eq!(
            event_status,
            EventStatus::Used(Some(MergeEvent::KeysDown(KeysChange {
                current_coord: None,
                new_keys: KeyCode::V,
                active_keys: vec![KeyCode::LControl],
            })))
        );

        let event = Event::Keyboard(keyboard::Event::KeyReleased {
            key_code: KeyCode::V,
            modifiers: Modifiers::empty(),
        });
        let event_status = events_merger.push_event(None, event);
        assert_eq!(
            event_status,
            EventStatus::Used(Some(MergeEvent::KeysUp(KeysChange {
                current_coord: None,
                new_keys: KeyCode::V,
                active_keys: vec![KeyCode::LControl],
            })))
        );

        let event = Event::Keyboard(keyboard::Event::KeyReleased {
            key_code: KeyCode::LControl,
            modifiers: Modifiers::empty(),
        });
        let event_status = events_merger.push_event(None, event);
        assert_eq!(
            event_status,
            EventStatus::Used(Some(MergeEvent::KeysUp(KeysChange {
                current_coord: None,
                new_keys: KeyCode::LControl,
                active_keys: vec![],
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
pub struct Mousemove {
    pub current_coord: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeysChange {
    pub current_coord: Option<Point>,
    pub new_keys: keyboard::KeyCode,
    pub active_keys: Vec<keyboard::KeyCode>,
}
