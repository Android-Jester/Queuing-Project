use crate::prelude::*;
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

#[derive(
    Selectable, Queryable, Insertable, Deserialize, QueryableByName, Serialize, Clone, Debug,
)]
#[diesel(table_name = Servers)]
#[diesel(check_for_backend(Mysql))]
struct ServerServiceRates {
    service_time: i32,
}

pub fn combined_service_rate() -> Result<f64, String> {
    let conn = &mut establish_connection();
    let res = diesel::sql_query("SELECT service_time FROM Servers")
        .get_results::<ServerServiceRates>(conn);

    match res {
        Ok(times) => {
            let mut sum = 0.0;
            let server_length = times.len();
            for time in times {
                let trac: f64 = time.service_time as f64 / 60.0;
                sum += trac;
            }
            let res = sum / server_length as f64;
            Ok(res)
        }
        Err(_) => Err("Unable to get combined service rates".to_string()),
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
            Err("Unable to Find Teller")
        }
    }
}
