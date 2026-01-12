use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[cfg(debug_assertions)]
static DEBUG: bool = true;

#[cfg(not(debug_assertions))]
static DEBUG: bool = false;

#[derive(Default)]
struct Plugin {
    permission_granted: bool,
    setup: bool,
    buffered_events: Vec<Event>,
    clients: Vec<ClientInfo>,
    tabs: Vec<TabInfo>,
    manifest: PaneManifest,
    picked: Vec<PaneId>,
}

register_plugin!(Plugin);

enum Command {
    Pick,
    Place,
    Chuck,
}

impl TryFrom<String> for Command {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "pick" => Ok(Command::Pick),
            "place" => Ok(Command::Place),
            "chuck" => Ok(Command::Chuck),
            _ => Err(format!("unknown command '{}'", value))
        }
    }
}

impl ZellijPlugin for Plugin {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        subscribe(&[
            EventType::PermissionRequestResult,
            EventType::ListClients,
            EventType::TabUpdate,
            EventType::PaneUpdate,
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
            // hide the plugin pane if we have permissions
            if !DEBUG && !self.setup {
                hide_self();
                self.setup = true;
            }

            while self.buffered_events.len() > 0 {
                let event = self.buffered_events.pop().unwrap();
                self.handle_event(event);
            }
        }

        return DEBUG;
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        // ignore messages with no payload
        let payload = match pipe_message.payload {
            Some(payload) => payload,
            None => return false
        };

        // parse command from payload
        let command = match Command::try_from(payload) {
            Ok(command) => command,
            Err(parse_err) => {
                eprintln!("{}", parse_err);
                return false;
            }
        };

        match command {
            Command::Pick => self.pick(),
            Command::Place => self.place(),
            Command::Chuck => self.chuck(),
        }

        return DEBUG;
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        println!("Picked panes: {:?}", self.picked);
        println!();
        if let Some(pane) = self.get_focused_pane() {
            println!("Focused Pane: {:?}", pane);
        }
    }
}

impl Plugin {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::ListClients(clients) => self.clients = clients,
            Event::TabUpdate(tabs) => {
                self.tabs = tabs;
                list_clients();
            },
            Event::PaneUpdate(manifest) => {
                self.manifest = manifest;
                list_clients();
            },
            _ => {}
        }
    }

    fn pick(&mut self) {
        if let Some(pane) = self.get_focused_pane() {
            if !self.picked.contains(&pane) {
                self.picked.push(pane);
                hide_pane_with_id(pane);
            }
        }
    }

    fn place(&mut self) {
        if let Some(tab) = get_focused_tab(&self.tabs) {
            for pane in &self.picked {
                show_pane_with_id(*pane, false);
            }

            break_panes_to_tab_with_index(self.picked.as_slice(), tab.position, true);
            self.picked.clear();
        }

        if !DEBUG { close_self(); }
    }

    fn chuck(&mut self) {
        if self.picked.len() > 0 {
            for pane in &self.picked {
                show_pane_with_id(*pane, false);
            }

            break_panes_to_new_tab(self.picked.as_slice(), None, true);
            self.picked.clear();
        }

        if !DEBUG { close_self(); }
    }

    fn get_focused_pane(&self) -> Option<PaneId> {
        for client in &self.clients {
            if client.is_current_client {
                return Some(client.pane_id);
            }
        }

        None
    }
}
