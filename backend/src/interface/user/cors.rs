// use std::pin::Pin;
// use std::sync::Mutex;
// use std::task::{Context, Poll};
// use std::time::{Duration, Instant};

// use actix::{Actor, ActorContext, AsyncContext, Handler, Message, StreamHandler};
// use actix_web::web::Bytes;
// use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
// use actix_web_actors::ws;
// use futures::Stream;
// use log::info;
// use std::sync::MutexGuard;

// const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
// const CLIENT_TIMEOUT: Duration = Duration::from_secs(15);
// #[derive(Clone, Debug, PartialEq)]
// // #[rtype(result = "WebState")]
// pub enum WebState {
//     Update,
//     NoUpdate,
// }

// impl WebState {
//     pub fn show(&self) -> &Self {
//         self
//     }
//     pub fn update_state(&mut self) {
//         info!("Updating State");
//         *self = WebState::Update;
//     }
//     pub fn back_to_normal(&mut self) {
//         info!("Back to Normal");
//         *self = WebState::NoUpdate;
//     }
//     pub fn show_state(&self) -> WebState {
//         self.clone()
//     }
//     pub fn check(this: &WebState) -> bool {
//         let matched_data = matches!(this, WebState::Update);
//         info!("Match: {matched_data}");
//         matched_data
//     }
// }

// pub struct Statementss {}

// impl actix::Message for Statementss {
//     type Result = WebState;
// }

// pub struct MyWS {
//     hb: Instant,
// }

// impl MyWS {
//     fn new() -> Self {
//         Self { hb: Instant::now() }
//     }
// }
// //
// impl Actor for MyWS {
//     type Context = ws::WebsocketContext<Self>;
//     fn started(&mut self, ctx: &mut Self::Context) {
//         self.hb(ctx);
//     }
// }
// impl MyWS {
//     #[allow(non_snake_case)]
//     fn collect_webState(&mut self, state: WebState) {
//         self.state = state;
//     }

//     fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
//         ctx.run_interval(
//             HEARTBEAT_INTERVAL,
//             |act: &mut MyWS, ctx: &mut ws::WebsocketContext<MyWS>| {
//                 // check client heartbeats
//                 if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
//                     // heartbeat timed out
//                     println!("Websocket Client heartbeat failed, disconnecting!");

//                     // stop actor
//                     ctx.stop();

//                     // don't try to send a ping
//                     return;
//                 }

//                 ctx.ping(b"");
//             },
//         );
//     }
// }

// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWS {
//     fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         let message = match item {
//             Ok(ws_data) => ws_data,
//             Err(_) => {
//                 println!("Error");
//                 return;
//             }
//         };
//         match message {
//             ws::Message::Text(_) => {}
//             ws::Message::Binary(data) => ctx.binary(data),
//             ws::Message::Continuation(_) => ctx.stop(),
//             ws::Message::Ping(data) => {
//                 self.hb = Instant::now();
//                 info!("Ping");
//                 ctx.pong(&data)
//             }
//             ws::Message::Pong(_) => {
//                 self.hb = Instant::now();
//             }
//             ws::Message::Close(reason) => {
//                 ctx.close(reason);
//                 ctx.stop();
//             }
//             ws::Message::Nop => (),
//         }
//     }
// }

// #[get("/ws")]
// pub async fn ws_start(
//     req: HttpRequest,
//     stream: web::Payload,
//     web_state: web::Data<Mutex<WebState>>,
// ) -> Result<HttpResponse, Error> {
//     let mut my_web_socket = MyWS::new();
//     my_web_socket.collect_webState(web_state.lock().unwrap().clone());
//     ws::start(my_web_socket, &req, stream)
// }
