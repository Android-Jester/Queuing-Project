pub mod models;
pub mod schema;

pub mod prelude {
    pub const CUSTOMER_COUNT: usize = 30;
    pub const SERVER_COUNT: usize = 4;
    pub use super::models::prelude::*;
    pub use super::schema::*;
}
