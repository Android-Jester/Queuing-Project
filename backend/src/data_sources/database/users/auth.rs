use crate::prelude::*;
// TODO: Use only one query object
pub fn db_login_user(login_data: ClientLoginData) -> Result<ClientDataQuery, &'static str> {
    let conn = &mut establish_connection();
    match conn.transaction(|connection| {
        Clients::dsl::Clients
            .select(ClientQuery::as_select())
            .filter(Clients::account_number.eq(login_data.account_number))
            .first(connection)
    }) {
        Ok(user) => {
            if user.password.eq(&login_data.password) {
                Ok(ClientDataQuery::new(
                    user.name,
                    user.account_number,
                    user.national_id,
                ))
            } else {
                Err("Unable to login User")
            }
        }
        Err(_) => Err("Unable to Find User"),
    }
}
