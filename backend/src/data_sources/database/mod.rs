pub mod guests;
pub mod tellers;
pub mod transactions;
pub mod users;

pub mod prelude {
    pub use diesel::prelude::*;
    #[allow(clippy::expect_used)]
    pub fn establish_connection() -> MysqlConnection {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("Database URL not Provided");
        MysqlConnection::establish(&database_url).expect("Unable to connect to DB")
    }

    pub use super::guests::prelude::*;
    pub use super::tellers::prelude::*;
    pub use super::transactions::prelude::*;
    pub use super::users::prelude::*;
}