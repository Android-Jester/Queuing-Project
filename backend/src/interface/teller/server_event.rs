use std::{sync::Arc, time::Duration};

use actix_web::rt::time::interval;
use actix_web_lab::sse::{self, ChannelStream, Sse};
use futures_util::future;
use parking_lot::Mutex;

use crate::prelude::*;

pub struct ServerBroadcaster {
    inner: Mutex<ServerBroadcasterInner>,
}
#[derive(Debug, Clone)]
pub struct ServerClients {
    // ip: String,
    sender: sse::Sender,
}

impl ServerClients {
    fn new(sender: sse::Sender) -> Self {
        Self { sender }
    }
}

#[derive(Debug, Clone, Default)]
struct ServerBroadcasterInner {
    clients: Vec<ServerClients>,
}

impl ServerBroadcaster {
    /// Constructs new broadcaster and spawns ping loop.
    pub fn create() -> Arc<Self> {
        let this = Arc::new(ServerBroadcaster {
            inner: Mutex::new(ServerBroadcasterInner::default()),
        });

        ServerBroadcaster::spawn_ping(Arc::clone(&this));

        this
    }

    /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast
    /// list if not.
    fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    /// Removes all non-responsive clients from broadcast list.
    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();

        let mut ok_clients = Vec::new();

        for client in clients {
            if client
                .sender
                .send(sse::Event::Comment("ping".into()))
                .await
                .is_ok()
            {
                ok_clients.push(client.clone());
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    /// Registers client with broadcaster, returning an SSE response body.
    pub async fn new_client(&self, data: &Vec<ClientQueueData>) -> Sse<ChannelStream> {
        let (tx, rx) = sse::channel(1);
        tx.send(sse::Data::new_json(data.clone()).unwrap())
            .await
            .unwrap();
        self.inner.lock().clients.push(ServerClients::new(tx));
        rx
    }

    /// Broadcasts `msg` to all clients.
    pub async fn user_update(&self, sub_queue: &mut SubQueues, service_location: usize) {
        let clients = self.inner.lock().clients.clone();
        let send_futures = clients.iter().map(|client| {
            let results = sub_queue.teller_show_queue(service_location);
            let json = sse::Data::new_json(results).unwrap();
            info!("JSON: {:?}", json);
            client.sender.send(json)
        });

        // try to send to all clients, ignoring failures
        // disconnected clients will get swept up by `remove_stale_clients`
        let _ = future::join_all(send_futures).await;
    }
}
