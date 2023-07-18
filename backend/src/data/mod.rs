pub mod guests;
pub mod schema;
pub mod teller;
pub mod transaction;
pub mod user;
pub mod prelude {
    pub use super::guests::prelude::*;
    pub use super::schema::*;
    pub use super::teller::prelude::*;
    pub use super::transaction::prelude::*;
    pub use super::user::prelude::*;
    pub use diesel::mysql::Mysql;
}
