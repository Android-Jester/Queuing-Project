pub mod actions;
pub mod cancelled_actions;
pub mod prelude {
    pub use super::actions::*;
    pub use super::cancelled_actions::*;
    use crate::prelude::*;
    pub fn analytics_config(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/analytics")
                .service(analytics_display)
                .service(get_report)
                .service(
                    scope("/transactions")
                        .service(daily_report)
                        .service(weekly_report)
                        .service(monthly_report)
                        .service(yearly_report),
                )
                .service(
                    scope("/cancelled")
                        .service(cancelled_daily_report)
                        .service(cancelled_weekly_report)
                        .service(cancelled_monthly_report)
                        .service(cancelled_yearly_report),
                ),
        );
    }
}
