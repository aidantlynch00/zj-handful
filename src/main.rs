use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[cfg(debug_assertions)]
static DEBUG: bool = true;

#[cfg(not(debug_assertions))]
static DEBUG: bool = false;

#[derive(Default)]
struct Plugin {
    permission_granted: bool,
    buffered_events: Vec<Event>,
}

register_plugin!(Plugin);

impl ZellijPlugin for Plugin {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        subscribe(&[
            EventType::PermissionRequestResult,
        ]);

        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::PermissionRequestResult(status) => {
                self.permission_granted = matches!(status, PermissionStatus::Granted);
            },
            _ => self.buffered_events.push(event),
        }

        if self.permission_granted {
            while self.buffered_events.len() > 0 {
                let event = self.buffered_events.pop().unwrap();
                self.handle_event(event);
            }
        }

        return DEBUG;
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        let _payload = match pipe_message.payload {
            Some(payload) => payload,
            None => return false
        };

        return DEBUG;
    }

    fn render(&mut self, _rows: usize, _cols: usize) { }
}

impl Plugin {
    fn handle_event(&mut self, _event: Event) { }
}
