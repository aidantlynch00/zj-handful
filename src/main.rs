use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[cfg(feature = "tracing")]
pub fn init_tracing() {
    use std::fs::File;
    use std::sync::Arc;
    use tracing_subscriber::layer::SubscriberExt;

    let file = File::create("/host/zj-handful.log");
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("error creating log file: {:?}", error)
    };

    let writer = tracing_subscriber::fmt::layer()
        .with_writer(Arc::new(file));

    let subscriber = tracing_subscriber::registry()
        .with(writer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to init tracing");
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
    Toss,
    Spike,
}

impl TryFrom<String> for Command {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "pick" => Ok(Command::Pick),
            "place" => Ok(Command::Place),
            "chuck" => Ok(Command::Chuck),
            "toss" => Ok(Command::Toss),
            "spike" => Ok(Command::Spike),
            _ => Err(format!("unknown command '{}'", value))
        }
    }
}

impl ZellijPlugin for Plugin {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        #[cfg(feature = "tracing")]
        init_tracing();
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
            },
            (_, Event::PermissionRequestResult(PermissionStatus::Denied)) => {
                self.permission_granted = Some(false);
            },
            (None, _) => {
                self.buffered_events.push(event);
            },
            (Some(true), _) => {
                self.handle_event(event);
            },
            (Some(false), _) => { }
        }

        false
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
        self.handle_command();

        false
    }

    fn render(&mut self, _rows: usize, _cols: usize) { }
}

impl Plugin {
    #[tracing::instrument(skip_all)]
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::ListClients(clients) => {
                tracing::trace!("got clients");
                self.clients = Some(clients);
                self.handle_command();
            },
            Event::TabUpdate(tabs) => {
                tracing::trace!("got tabs");
                self.tabs = Some(tabs);
                self.clients = None;
                list_clients();
            },
            _ => { }
        }
    }

    #[tracing::instrument(skip_all)]
    fn handle_command(&mut self) {
        match (&self.tabs, &self.clients) {
            (Some(_), Some(_)) => { },
            _ => {
                tracing::debug!("cannot handle command yet");
                return;
            }
        };

        if let Some(command) = &self.buffered_command {
            match command {
                Command::Chuck => {
                    tracing::trace!("handling chuck");

                    // create a new tab and wait to place on it
                    new_tab::<&str>(None, None);
                    self.tabs = None;
                    self.buffered_command = Some(Command::Place);
                    return;
                },
                Command::Pick => self.pick(),
                Command::Place => self.place(),
                Command::Toss => self.toss(),
                Command::Spike => self.spike(),
            }

            self.buffered_command = None;
        }
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

        let tabs = self.tabs.as_ref().unwrap();
        let focused_tab = get_focused_tab(tabs);

        if let Some(tab) = focused_tab {
            for pane in &self.picked {
                tracing::debug!("showing pane {:?}", pane);
                show_pane_with_id(*pane, false);
            }

            tracing::info!("placing {:?}", self.picked);
            break_panes_to_tab_with_index(self.picked.as_slice(), tab.position, true);
            self.picked.clear();
        }

        close_self();
    }

    #[tracing::instrument(skip_all)]
    fn toss(&mut self) {
        tracing::trace!("toss called");

        let picked = std::mem::take(&mut self.picked);
        for pane in &picked {
            tracing::debug!("showing pane {:?}", pane);
            show_pane_with_id(*pane, false);
        }

        float_multiple_panes(picked);
        close_self();
    }

    #[tracing::instrument(skip_all)]
    fn spike(&mut self) {
        tracing::trace!("spike called");

        let picked = std::mem::take(&mut self.picked);
        for pane in &picked {
            tracing::debug!("showing pane {:?}", pane);
            show_pane_with_id(*pane, false);
        }

        embed_multiple_panes(picked);
        close_self();
    }

    #[tracing::instrument(skip_all)]
    fn finish_setup(&mut self) {
        tracing::debug!("hiding plugin pane and making it unselectable");
        hide_self();
        set_selectable(false);

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
