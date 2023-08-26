use crate::prelude::*;

#[get("/daily")]
pub async fn cancelled_daily_report() -> impl Responder {
    let report = get_daily_cancelled_report();

    match report {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/weekly")]
pub async fn cancelled_weekly_report() -> impl Responder {
    let report = get_weekly_cancelled_report();

    match report {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/monthly")]
pub async fn cancelled_monthly_report() -> impl Responder {
    let report = get_monthly_cancelled_report();

    match report {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/yearly")]
pub async fn cancelled_yearly_report() -> impl Responder {
    let report = get_year_cancelled_report();

    match report {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}
