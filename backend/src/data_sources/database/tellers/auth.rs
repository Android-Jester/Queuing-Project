use crate::prelude::*;
// TODO: Change Models to be returnable
pub fn db_auth_teller(teller_login: ServerLogin) -> Result<(String, i32, i32), &'static str> {
    let conn = &mut establish_connection();
    match conn.transaction(|connection| {
        Servers::dsl::Servers
            .select(ServerQuery::as_select())
            .find(teller_login.server_id)
            .first(connection)
    }) {
        Ok(teller) => {
            if teller.password.eq(&teller_login.password) {
                // TODO: Convert Data to Struct
                Ok((teller.server_id, teller.station, teller.service_time))
            } else {
                Err("Unable to login Teller")
            }
        }
        Err(_) => Err("Unable to Find Teller"),
    }
}


pub fn db_teller_service_time(teller_id: String) -> i32 {
    let connection = &mut establish_connection();
    let user = connection.transaction(|conn| {
        Servers::dsl::Servers.find(teller_id).select(ServerQuery::as_select()).load(conn)
    });
    match user {
        Ok(teller) => teller[0].service_time,
        Err(_) => 0
    }
}
