use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[cfg(debug_assertions)]
mod env {
    use std::fs::File;
    use std::sync::Arc;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::fmt::layer;

    pub static DEBUG: bool = true;

    pub fn init_tracing() {
        let file = File::create("/host/zj-pnp.log");
        let file = match file {
            Ok(file) => file,
            Err(error) => panic!("error creating log file: {:?}", error)
        };

        let debug_log = layer().with_writer(Arc::new(file));
        tracing_subscriber::registry()
            .with(debug_log)
            .init();
    }
}

#[cfg(not(debug_assertions))]
mod env {
    pub static DEBUG: bool = false;
    pub fn init_tracing() { }
}

#[derive(Debug, Default)]
struct Plugin {
    permission_granted: Option<bool>,
    buffered_events: Vec<Event>,
    buffered_command: Option<Command>,
    clients: Option<Vec<ClientInfo>>,
    tabs: Option<Vec<TabInfo>>,
    picked: Vec<PaneId>,
}

register_plugin!(Plugin);

#[derive(Debug)]
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
        env::init_tracing();
        tracing::debug!("tracing initialized");

        let events = &[
            EventType::PermissionRequestResult,
            EventType::ListClients,
            EventType::TabUpdate,
        ];

        subscribe(events);
        tracing::info!("subscribed to {:?}", events);

        let permissions = &[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ];

        request_permission(permissions);
        tracing::info!("requested permissions {:?}", permissions);
    }

    #[tracing::instrument(skip_all)]
    fn update(&mut self, event: Event) -> bool {
        match (&self.permission_granted, &event) {
            (None, Event::PermissionRequestResult(PermissionStatus::Granted)) => {
                tracing::info!("permission granted");
                self.permission_granted = Some(true);
                self.finish_setup();
                env::DEBUG
            },
            (_, Event::PermissionRequestResult(PermissionStatus::Denied)) => {
                self.permission_granted = Some(false);
                false
            },
            (None, _) => {
                self.buffered_events.push(event);
                false
            },
            (Some(true), _) => {
                env::DEBUG && self.handle_event(event)
            },
            (Some(false), _) => { false }
        }
    }

    #[tracing::instrument(skip_all)]
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
                tracing::error!("{}", parse_err);
                return false;
            }
        };

        self.buffered_command = Some(command);
        env::DEBUG && self.handle_command()
    }

    #[tracing::instrument(skip_all)]
    fn render(&mut self, _rows: usize, _cols: usize) {
        println!("Picked panes: {:?}", self.picked);
        println!();
        if let Some(pane) = self.get_focused_pane() {
            println!("Focused Pane: {:?}", pane);
        }
    }
}

impl Plugin {
    #[tracing::instrument(skip_all)]
    fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::ListClients(clients) => {
                tracing::trace!("got clients");
                self.clients = Some(clients);
                self.handle_command();
                true
            },
            Event::TabUpdate(tabs) => {
                tracing::trace!("got tabs");
                self.tabs = Some(tabs);
                self.clients = None;
                list_clients();
                false
            },
            _ => { false }
        }
    }

    #[tracing::instrument(skip_all)]
    fn handle_command(&mut self) -> bool {
        match (&self.tabs, &self.clients) {
            (Some(_), Some(_)) => { },
            _ => {
                tracing::debug!("cannot handle command yet");
                return false;
            }
        };

        if let Some(command) = &self.buffered_command {
            match command {
                Command::Pick => self.pick(),
                Command::Place => self.place(),
                Command::Chuck => self.chuck(),
            }

            self.buffered_command.take();
            return true;
        }

        false
    }

    #[tracing::instrument(skip_all)]
    fn pick(&mut self) {
        tracing::trace!("pick called");
        if let Some(pane) = self.get_focused_pane() {
            if !self.picked.contains(&pane) {
                tracing::info!("picking pane {:?}", pane);
                self.picked.push(pane);
                tracing::debug!("hiding pane {:?}", pane);
                hide_pane_with_id(pane);
            }
        }
    }

    #[tracing::instrument(skip_all)]
    fn place(&mut self) {
        tracing::trace!("place called");

        let focused_tab = self.tabs
            .as_ref()
            .and_then(get_focused_tab);

        if let Some(tab) = focused_tab {
            for pane in &self.picked {
                tracing::debug!("showing pane {:?}", pane);
                show_pane_with_id(*pane, false);
            }

            tracing::info!("placing {:?}", self.picked);
            break_panes_to_tab_with_index(self.picked.as_slice(), tab.position, true);
            self.picked.clear();
        }

        if !env::DEBUG {
            tracing::debug!("closing plugin pane");
            close_self();
        }
    }

    #[tracing::instrument(skip_all)]
    fn chuck(&mut self) {
        tracing::trace!("chuck called");
        if self.picked.len() > 0 {
            for pane in &self.picked {
                tracing::debug!("showing pane {:?}", pane);
                show_pane_with_id(*pane, false);
            }

            tracing::info!("chucking {:?}", self.picked);
            break_panes_to_new_tab(self.picked.as_slice(), None, true);
            self.picked.clear();
        }

        if !env::DEBUG {
            tracing::debug!("closing plugin pane");
            close_self();
        }
    }

    #[tracing::instrument(skip_all)]
    fn finish_setup(&mut self) {
        if !env::DEBUG { hide_self(); }
        while self.buffered_events.len() > 0 {
            let event = self.buffered_events.pop().unwrap();
            self.handle_event(event);
        }
    }

    fn get_focused_pane(&self) -> Option<PaneId> {
        for client in self.clients.as_ref()? {
            if client.is_current_client {
                return Some(client.pane_id);
            }
        }

        None
    }
}
