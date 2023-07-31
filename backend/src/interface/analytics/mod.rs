pub mod actions;
pub mod prelude {
    pub use super::actions::*;
    use crate::prelude::*;
    pub fn analytics_config(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/analytics")
                .service(daily_report)
                .service(weekly_report)
                .service(monthly_report)
                .service(yearly_report),
        );
    }
}
