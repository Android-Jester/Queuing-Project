pub mod teller;
pub mod transaction;
pub mod user;

pub mod prelude {
    use diesel::{Connection, MysqlConnection};
    use dotenvy::dotenv;
    use std::env;

    pub fn establish_connection() -> MysqlConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        MysqlConnection::establish(&database_url).expect("Unable to connect to DB")
    }

    #[doc(inline)]
    pub use super::teller::*;

    #[doc(inline)]
    pub use super::user::*;

    #[doc(inline)]
    pub use super::transaction::*;
}
