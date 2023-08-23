use crate::prelude::*;
use diesel::RunQueryDsl;
pub fn db_list_tellers() -> Result<Vec<ServerQuery>, &'static str> {
    let conn = &mut establish_connection();
    match conn.transaction(|connection| {
        Servers::dsl::Servers
            .select(ServerQuery::as_select())
            .load(connection)
    }) {
        Ok(transactions) => Ok(transactions),
        Err(_) => Err("Unable to find transactions"),
    }
}

// TODO: Fix this function to return model that doesn't contain password
pub fn find_teller(teller_id: String) -> Result<ServerQuery, &'static str> {
    let conn = &mut establish_connection();
    let transactions_data = conn.transaction(|connection| {
        Servers::dsl::Servers
            .select(ServerQuery::as_select())
            .find(teller_id)
            .first(connection)
    });
    match transactions_data {
        Ok(teller) => {
            info!("Teller: {:?}", teller);
            Ok(teller)
        }
        Err(_) => {
            error!("Unable to find teller");
            Err("Unable to Find Telle")
        }
    }
}

#[derive(QueryableByName, Selectable)]
#[diesel(table_name = Transactions)]
#[diesel(check_for_backend(Mysql))]
struct STime {
    duration: f32,
}

pub fn average_service_time() -> Result<f32, String> {
    let conn = &mut establish_connection();
    let durations: Result<Vec<STime>, diesel::result::Error> =
        diesel::sql_query("SELECT duration from Transactions")
            .get_results(&mut establish_connection());
    match durations {
        Ok(service_times) => {
            let mut sum = 0.0;
            let count = service_times.len();
            for service_time in service_times {
                sum += service_time.duration;
            }
            let res = sum / count as f32;
            Ok(res)
        }
        Err(er) => Err(format!("Error: {}", er)),
    }
}
