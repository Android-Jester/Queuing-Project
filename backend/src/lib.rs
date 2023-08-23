pub mod consts;
pub mod data;
pub mod data_sources;
pub mod interface;
#[cfg(test)]
pub mod test;
pub mod prelude {
    pub use super::consts::*;
    pub use super::data::prelude::*;
    pub use super::data_sources::prelude::*;
    pub use super::interface::prelude::*;
    pub use parking_lot::{Mutex, MutexGuard};
    pub use std::time::Duration;
    pub use std::{net::Ipv4Addr, sync::Arc};

    // Imports from other packages
    pub use actix_web::{
        self, get, middleware, post,
        web::{self, *},
        App, HttpRequest, HttpResponse, HttpServer, Responder,
    };
    pub use log::*;
    pub use serde::{self, *};
}
