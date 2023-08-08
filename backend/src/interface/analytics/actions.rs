use crate::prelude::*;
#[get("/daily")]
pub async fn daily_report() -> impl Responder {
    let report = get_daily_report();

    match report {
        Ok(transactions) => HttpResponse::Ok()
            // .content_type("application/octet-stream")
            .json(transactions),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/weekly")]
pub async fn weekly_report() -> impl Responder {
    let report = get_weekly_report();

    match report {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/monthly")]
pub async fn monthly_report() -> impl Responder {
    let report = get_monthly_report();

    match report {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/yearly")]
pub async fn yearly_report() -> impl Responder {
    let report = get_year_report();

    match report {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}
