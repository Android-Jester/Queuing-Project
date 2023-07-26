use std::ops::Deref;

use actix_web_lab::sse::{self, ChannelStream, Sse};
use tokio::time::interval;

use crate::prelude::*;
pub struct ClientBroadcaster {
    inner: Mutex<ClientBroadcasterInner>,
}

#[derive(Debug, Clone)]
pub struct ThreadClients {
    ip: String,
    sender: sse::Sender,
}

impl ThreadClients {
    fn new(sender: sse::Sender, ip: String) -> Self {
        Self { sender, ip }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ClientBroadcasterInner {
    clients: Vec<ThreadClients>,
}

impl ClientBroadcaster {
    pub fn create() -> Arc<Self> {
        let client_builder = Arc::new(Self {
            inner: Mutex::new(ClientBroadcasterInner::default()),
        });
        ClientBroadcaster::spawn_ping(Arc::clone(&client_builder));
        client_builder
    }

    fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();
        let mut ok_clients = vec![];

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

    pub async fn new_client(
        &self,
        ip: String,
        data: &mut ClientQueueData,
        sub_queue: &mut SubQueues,
        service_location: usize,
        server_broadcaster: Arc<ServerBroadcaster>,
        client_broadcaster: Arc<ClientBroadcaster>,
    ) -> Sse<ChannelStream> {
        let (tx, rx) = sse::channel(10*1024*1024);
        tx.send(sse::Data::new_json(data.clone()).unwrap()).await.unwrap();
        self.inner
            .lock()
            .clients
            .push(ThreadClients::new(tx, ip.clone()));
        data.timer_countdown(ip, client_broadcaster).await;
        server_broadcaster
            .user_update(sub_queue, service_location)
            .await;
        rx
    }

    pub async fn countdowning(&self, user: ClientQueueData, ip: String) {
        warn!("Called");
        let clients = self.inner.lock().clients.clone();
        let send_futures = clients.iter().map(|client| {
            if client.ip == ip {
                let json = sse::Data::new_json(user.clone()).unwrap();
                client.sender.send(json)
            } else {
                client.sender.send(sse::Data::new(""))
            }
        });

        futures_util::future::join_all(send_futures).await;
    }
}
