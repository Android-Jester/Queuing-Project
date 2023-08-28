use std::io::Read;

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

#[get("")]
pub async fn analytics_display() -> impl Responder {
    let analytics = crate::data::analytics::Analytics::new();
    HttpResponse::Ok().json(analytics)
}

#[get("/get_report")]
pub async fn get_report() -> impl Responder {
    generate_pdf();
    let mut file = std::fs::File::open("report.pdf").unwrap();
    let mut buffer = Vec::new();
    if let Ok(_) = file.read_to_end(&mut buffer) {
        #[allow(deprecated)]
        HttpResponse::Ok()
            .content_type("application/pdf")
            .header(
                "Content-Disposition",
                format!("attachment; filename=report.pdf"),
            )
            .body(buffer)
    } else {
        HttpResponse::NotAcceptable().body("Unable to convert buffer")
    }
}
