extern crate log;
pub mod consts;
pub mod data;
pub mod data_sources;
pub mod interface;
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
        self, middleware,
        web::{self, *},
        App, HttpServer,
    };
    pub use log::*;
    pub use serde::{self, *};
}
