use crate::prelude::*;
use actix_web::rt::time::interval;
use actix_web_lab::sse::{self, ChannelStream, Sse};

pub struct BroadcasterUser {
    inner: Mutex<BroadcasterInner>,
}

#[derive(Debug, Clone)]
struct Client {
    sender: sse::Sender,
    ip: String,
}

impl Client {
    fn new(ip: String, sender: sse::Sender) -> Self {
        Self { ip, sender }
    }
}

#[derive(Debug, Clone, Default)]
struct BroadcasterInner {
    clients: Vec<Client>,
}

impl BroadcasterUser {
    /// Constructs new broadcaster and spawns ping loop.
    pub fn create() -> Arc<Self> {
        let this = Arc::new(BroadcasterUser {
            inner: Mutex::new(BroadcasterInner::default()),
        });

        BroadcasterUser::spawn_ping(Arc::clone(&this));

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
                .send(sse::Event::Comment(bytestring::ByteString::from(
                    "ping".to_string(),
                )))
                .await
                .is_ok()
            {
                ok_clients.push(client.clone());
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    /// Registers client with broadcaster, returning an SSE response body.
    pub async fn new_client(
        &self,
        added_user: &UserQueuePos,
        ip: String,
        // queue: &mut SubQueues,
        // broadcaster: &Data<BroadcasterUser>,
    ) -> Sse<ChannelStream> {
        let (tx, rx) = sse::channel(10);
        info!("CALLED: {:?}", added_user);
        if let Ok(user_data) = sse::Data::new_json(added_user) {
            tx.send(user_data).await.unwrap();
            let client = Client::new(ip.clone(), tx);
            self.inner.lock().clients.push(client);
        };
        rx
    }

    pub async fn error(&self) -> Sse<ChannelStream> {
        let (tx, rx) = sse::channel(10);
        tx.send(sse::Data::new("Error")).await.unwrap();
        // self.inner.lock().clients.push(tx);
        rx.with_keep_alive(Duration::from_secs(0))
    }

    // Broadcasts `msg` to all clients.
    pub async fn broadcast_countdown(&self, user: &UserQueuePos, ip: String) {
        info!("DD");
        let clients = self.inner.lock().clients.clone();
        let send_futures = clients.iter().map(|client| {
            if client.ip == ip {
                let user_channel_data = sse::Data::new_json(user.clone()).unwrap();
                info!("SSS JSON: {:?}", user_channel_data);
                client.sender.send(user_channel_data)
            } else {
                client.sender.send(sse::Data::new(""))
            }
        });

        // try to send to all clients, ignoring failures
        // disconnected clients will get swept up by `remove_stale_clients`
        let _ = futures_util::future::join_all(send_futures).await;
    }
}
