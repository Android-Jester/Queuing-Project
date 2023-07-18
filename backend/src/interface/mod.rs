pub mod teller;
pub mod user;
pub mod prelude {
    pub use super::teller::prelude::*;
    pub use super::user::prelude::*;
    pub use actix_web::{get, post, web::*, HttpResponse, Responder};
}
