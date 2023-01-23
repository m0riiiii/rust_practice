use tracing::{debug, info, instrument};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::Config;
use crate::event::Event;
use crate::event_queue::EventQueue;
use crate::state::State;
use crate::connection::Connection;
use crate::packets::{message::Message, update::UpdateMessage};
use crate::routing::{AdjRibIn, AdjRibOut, LocRib};

#[derive(Debug)]
pub struct Peer {
    state: State,
    event_queue: EventQueue,
    tcp_connection: Option<Connection>,
    config: Config,
    loc_rib: Arc<Mutex<LocRib>>,
    adj_rib_out: AdjRibOut,
    adj_rib_in: AdjRibIn,
}

impl Peer {
    pub fn new(config: Config, loc_rib: Arc<Mutex<LocRib>>) -> Self {
        let state = State::Idle;
        let event_queue = EventQueue::new();
        let adj_rib_out = AdjRibOut::new();
        let adj_rib_in = AdjRibIn::new();
        Self {
            state,
            event_queue,
            tcp_connection: None::<Connection>,
            config,
            loc_rib,
            adj_rib_out,
            adj_rib_in,
        }
    }

    #[instrument]
    pub fn start(&mut self) {
        info!("peer is started.");
        self.event_queue.enqueu(Event::ManualStart);
    }

    #[instrument]
    pub async fn next(&mut self) {
        if let Some(event) = self.event_queue.dequeue() {
            info!("event is occuered, event={:?}.", event);
            self.handle_event(event).await;
        }

        if let Some(conn) = &mut self.tcp_connection {
            if let Some(message) = conn.get_message().await {
                info!("message is received, message={:?}", message);
                self.handle_message(message);
            }
        }
    }

    fn handle_message(&mut self, message: Message) {
        match message {
            Message::Open(open) => {
                self.event_queue.enqueu(Event::BgpOpen(open))
            },
            Message::Keepalive(keepalive) => {
                self.event_queue.enqueu(Event::KeepAliveMsg(keepalive))
            },
            Message::Update(update) => {
                self.event_queue.enqueu(Event::UpdateMsg(update))
            },
        }
    }

    async fn handle_event(&mut self, event: Event) {
        match &self.state {
            State::Idle => {
                match event {
                    Event::ManualStart => {
                        self.tcp_connection =
                            Connection::connect(&self.config).await.ok();
                        if self.tcp_connection.is_some() {
                            self.event_queue
                                .enqueu(Event::TcpConnectionConfirmed);
                        } else {
                            panic!(
                                "cannot establish TCP Connection. {:?}",
                                self.config
                            )
                        }
                        self.state = State::Connect;
                    }
                    _ => {}
                }
            },
            State::Connect => {
                match event {
                    Event::TcpConnectionConfirmed => {
                        self.tcp_connection
                            .as_mut()
                            .expect("no tcp connection.")
                            .send(Message::new_open(
                                self.config.local_as,
                                self.config.local_ip))
                                .await;
                        self.state = State::OpenSent
                    },
                    _ => {}
                }
            },
            State::OpenSent => {
                match event {
                    Event::BgpOpen(open) => {
                        self.tcp_connection
                            .as_mut()
                            .expect("no tcp connection.")
                            .send(Message::new_keepalive())
                            .await;
                        self.state = State::OpenConfirm;
                    },
                    _ => {}
                }
            },
            State::OpenConfirm => {
                match event {
                    Event::KeepAliveMsg(_) => {
                        self.state = State::Established;
                        self.event_queue.enqueu(Event::Established);
                    },
                    _ => {}
                }
            },
            State::Established => {
                match event {
                    Event::Established | Event::LocRibChanged => {
                        let loc_rib = self.loc_rib.lock().await;
                        self.adj_rib_out
                            .install_from_loc_rib(&loc_rib, &self.config);
                        if self.adj_rib_out.does_contain_new_route() {
                            self.event_queue.enqueu(Event::AdjRibOutChanged);
                            self.adj_rib_out.update_to_all_unchanged();
                        }
                    },
                    Event::AdjRibOutChanged => {
                        let updates: Vec<UpdateMessage> =
                            self.adj_rib_out.create_update_messages(
                                self.config.local_ip,
                                self.config.local_as
                            );
                            for update in updates {
                                self.tcp_connection
                                    .as_mut()
                                    .expect("cannot establish TCP Connection.")
                                    .send(Message::Update(update))
                                    .await;
                            }
                    },
                    Event::UpdateMsg(update) => {
                        self.adj_rib_in.install_from_update(update, &self.config);
                        if self.adj_rib_in.does_contain_new_route() {
                            debug!("adj_rib in is updated.");
                            self.event_queue.enqueu(Event::AdjRibInChanged);
                            self.adj_rib_in.update_to_all_unchanged();
                        }
                    },
                    Event::AdjRibInChanged => {
                        self.loc_rib
                            .lock()
                            .await
                            .install_from_adj_rib_in(&self.adj_rib_in);
                        if self.loc_rib.lock().await.does_contain_new_route() {
                            self.loc_rib
                                .lock()
                                .await
                                .write_to_kernel_routing_table()
                                .await;
                            self.event_queue.enqueu(Event::LocRibChanged);
                            self.loc_rib
                                .lock()
                                .await
                                .update_to_all_unchanged();
                        }
                    }
                    _ => {}
                }
            }
            _ => {},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn peer_can_transition_to_connect_state() {
        let config: Config = 
            "64512 127.0.0.1 65413 127.0.0.2 active".parse().unwrap();
        let loc_rib = Arc::new(Mutex::new(LocRib::new(&config).await.unwrap()));
        let mut peer = Peer::new(config, Arc::clone(&loc_rib));
        peer.start();

        tokio::spawn(async move {
            let remote_config = 
                "64513 127.0.0.2 64512 127.0.0.1 passive".parse().unwrap();
            let remote_loc_rib = 
                Arc::new(Mutex::new(LocRib::new(&remote_config).await.unwrap()));
            let mut remote_peer = Peer::new(remote_config, Arc::clone(&remote_loc_rib));
            remote_peer.start();
            remote_peer.next().await;

            assert_eq!(remote_peer.state, State::Connect);
        });

        sleep(Duration::from_secs(1)).await;
        peer.next().await;
        assert_eq!(peer.state, State::Connect);
    }

    #[tokio::test]
    async fn peer_can_transition_to_open_sent_state() {
        let config: Config = 
            "64512 127.0.0.1 65413 127.0.0.2 active".parse().unwrap();
            let loc_rib = Arc::new(Mutex::new(LocRib::new(&config).await.unwrap()));
            let mut peer = Peer::new(config, Arc::clone(&loc_rib));
        peer.start();

        tokio::spawn(async move {
            let remote_config = 
                "64513 127.0.0.2 64512 127.0.0.1 passive".parse().unwrap();
            let remote_loc_rib = 
                Arc::new(Mutex::new(LocRib::new(&remote_config).await.unwrap()));
            let mut remote_peer = Peer::new(remote_config, Arc::clone(&remote_loc_rib));
            remote_peer.start();
            remote_peer.next().await;
            remote_peer.next().await;

            assert_eq!(remote_peer.state, State::OpenSent);
        });

        sleep(Duration::from_secs(1)).await;
        peer.next().await;
        peer.next().await;
        assert_eq!(peer.state, State::OpenSent);
    }

    #[tokio::test]
    async fn peer_can_transition_to_open_confirm_state() {
        let config: Config = 
            "64512 127.0.0.1 65413 127.0.0.2 active".parse().unwrap();
        let loc_rib = Arc::new(Mutex::new(LocRib::new(&config).await.unwrap()));
        let mut peer = Peer::new(config, Arc::clone(&loc_rib));
        peer.start();

        let max_step = 50;

        tokio::spawn(async move {
            let remote_config = 
                "64513 127.0.0.2 64512 127.0.0.1 passive".parse().unwrap();
            let remote_loc_rib = 
                Arc::new(Mutex::new(LocRib::new(&remote_config).await.unwrap()));
            let mut remote_peer = Peer::new(remote_config, Arc::clone(&remote_loc_rib));
            remote_peer.start();
            for _ in 0..max_step {
                remote_peer.next().await;
                if remote_peer.state == State::OpenConfirm {
                    break;
                };
                sleep(Duration::from_secs_f32(0.1)).await;
            }
        });

        sleep(Duration::from_secs(1)).await;
        for _ in 0..max_step {
            peer.next().await;
            if peer.state == State::OpenConfirm {
                break;
            }
            sleep(Duration::from_secs_f32(0.1)).await;
        };
        assert_eq!(peer.state, State::OpenConfirm);
    }

    #[tokio::test]
    async fn peer_can_transition_to_established_state() {
        let config: Config = 
            "64512 127.0.0.1 65413 127.0.0.2 active".parse().unwrap();
        let loc_rib = Arc::new(Mutex::new(LocRib::new(&config).await.unwrap()));
        let mut peer = Peer::new(config, Arc::clone(&loc_rib));
        peer.start();

        let max_step = 50;

        tokio::spawn(async move {
            let remote_config = 
                "64513 127.0.0.2 64512 127.0.0.1 passive".parse().unwrap();
            let remote_loc_rib = 
                Arc::new(Mutex::new(LocRib::new(&remote_config).await.unwrap()));
            let mut remote_peer = Peer::new(remote_config, Arc::clone(&remote_loc_rib));
            remote_peer.start();
            for _ in 0..max_step {
                remote_peer.next().await;
                if remote_peer.state == State::Established {
                    break;
                };
                sleep(Duration::from_secs_f32(0.1)).await;
            }
        });

        sleep(Duration::from_secs(1)).await;
        for _ in 0..max_step {
            peer.next().await;
            if peer.state == State::Established {
                break;
            }
            sleep(Duration::from_secs_f32(0.1)).await;
        };
        assert_eq!(peer.state, State::Established);
    }
}