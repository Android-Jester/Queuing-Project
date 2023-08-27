pub mod calc;
pub mod logic;
pub mod prelude {
    use chrono::DateTime;
    use genpdf::*;

    use crate::prelude::get_daily_cancelled_report;
    use crate::prelude::get_daily_report;
    use crate::prelude::get_monthly_cancelled_report;
    use crate::prelude::get_monthly_report;
    use crate::prelude::get_weekly_cancelled_report;
    use crate::prelude::get_weekly_report;
    use crate::prelude::get_year_cancelled_report;
    use crate::prelude::get_year_report;
    use crate::prelude::CancelStruct;
    use crate::prelude::Transaction;

    pub use super::calc::*;
    pub use super::logic::*;

    pub fn generate_pdf() {
        let font_family = genpdf::fonts::from_files("./fonts/JetBrainsMono", "JetBrainsMono", None)
            .expect("Failed to load font family");
        let mut doc = genpdf::Document::new(font_family);
        doc.set_title("Smart Queue Queue System");
        let mut decorator = genpdf::SimplePageDecorator::new();
        // decorator.set_margins(10);
        doc.set_page_decorator(decorator);
        let now: DateTime<chrono::Utc> = chrono::Utc::now();
        let _ = now.format("YYYY-MM-DD HH:mm::ss");
        // Title Header
        let mut title = elements::Paragraph::new("Smart Queue Queue System");
        title.set_alignment(Alignment::Center);
        let title = title.padded(Margins::all(5));
        doc.push(title);

        // Time Header
        let mut time = elements::Paragraph::new(format!("{}", now));
        time.set_alignment(Alignment::Center);
        doc.push(time);

        // Yearly
        add_transaction_table("Yearly", get_year_report().unwrap(), &mut doc);

        // Monthly
        add_transaction_table("Monthly", get_monthly_report().unwrap(), &mut doc);

        // Weekly
        add_transaction_table("Weekly", get_weekly_report().unwrap(), &mut doc);

        // Daily
        add_transaction_table("Daily", get_daily_report().unwrap(), &mut doc);

        // Cancelled Table
        // Yearly
        add_cancelled_table("Yearly", get_year_cancelled_report().unwrap(), &mut doc);

        // Monthly
        add_cancelled_table("Monthly", get_monthly_cancelled_report().unwrap(), &mut doc);

        // Weekly
        add_cancelled_table("Weekly", get_weekly_cancelled_report().unwrap(), &mut doc);

        // Daily
        add_cancelled_table("Daily", get_daily_cancelled_report().unwrap(), &mut doc);

        doc.render_to_file("report.pdf")
            .expect("Failed to write PDF");
    }

    fn add_transaction_table(title_head: &str, transactions: Vec<Transaction>, doc: &mut Document) {
        let mut title = elements::Paragraph::new("");
        title.set_alignment(Alignment::Center);
        let mut style = style::Style::new();
        style.set_font_size(16);
        style.set_bold();
        let title = title.styled_string(format!("Transactions Page ({})", title_head), style);
        let title = title.padded(Margins::all(5));
        doc.push(title);
        let mut table = elements::TableLayout::new(vec![1, 1, 1, 1, 1, 1]);
        let mut row_table = table
            .row()
            .element(elements::Paragraph::new("ID"))
            .element(elements::Paragraph::new("detail"))
            .element(elements::Paragraph::new("server_id"))
            .element(elements::Paragraph::new("client_national_id"))
            .element(elements::Paragraph::new("duration"))
            .element(elements::Paragraph::new("created_date"));
        row_table.push();
        let mut count = 0;
        for transaction in transactions {
            count += 1;
            let table_row2 = table
                .row()
                .element(elements::Paragraph::new(count.to_string()))
                .element(elements::Paragraph::new(transaction.detail))
                .element(elements::Paragraph::new(transaction.server_id))
                .element(elements::Paragraph::new(transaction.client_national_id))
                .element(elements::Paragraph::new(transaction.duration.to_string()))
                .element(elements::Paragraph::new(
                    transaction.created_date.to_string(),
                ));
            table_row2.push().unwrap();
        }
        doc.push(table);
    }

    fn add_cancelled_table(title_head: &str, cancelled: Vec<CancelStruct>, doc: &mut Document) {
        let mut title = elements::Paragraph::new("");
        title.set_alignment(Alignment::Center);
        let mut style = style::Style::new();
        style.set_font_size(16);
        style.set_bold();
        let title = title.styled_string(format!("Cancelled Transactions({})", title_head), style);
        let title = title.padded(Margins::all(5));
        doc.push(title);
        let mut table = elements::TableLayout::new(vec![1, 1, 1, 1, 1, 1]);
        let mut table_row = table
            .row()
            .element(elements::Paragraph::new("ID"))
            .element(elements::Paragraph::new("detail"))
            .element(elements::Paragraph::new("server_id"))
            .element(elements::Paragraph::new("client_national_id"))
            .element(elements::Paragraph::new("duration"))
            .element(elements::Paragraph::new("created_date"));
        table_row.push().unwrap();
        let mut count = 0;
        for transaction in cancelled {
            count += 1;
            let table_row2 = table
                .row()
                .element(elements::Paragraph::new(count.to_string()))
                .element(elements::Paragraph::new(transaction.detail))
                .element(elements::Paragraph::new(transaction.server_id))
                .element(elements::Paragraph::new(transaction.client_national_id))
                .element(elements::Paragraph::new(
                    transaction.created_date.to_string(),
                ));
            table_row2.push().unwrap();
        }
        doc.push(table);
    }
}
