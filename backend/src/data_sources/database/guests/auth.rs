use crate::prelude::*;
/// Checks if user is authorized to access his account
pub fn db_add_guest(insert_data: GuestQuery) -> Result<GuestQuery, &'static str> {
    let conn = &mut establish_connection();
    match conn.transaction(|conn| {
        diesel::insert_into(Guests::dsl::Guests)
            .values(insert_data.clone())
            .execute(conn)
    }) {
        Ok(_) => Ok(insert_data),
        Err(_) => Err("Unable to add guests"),
    }
}
