// @generated automatically by Diesel CLI.

diesel::table! {
    Transaction (transaction) {
        #[max_length = 255]
        transaction -> Varchar,
        #[max_length = 255]
        server -> Nullable<Varchar>,
    }
}

diesel::table! {
    Users (account_number) {
        #[max_length = 255]
        account_number -> Varchar,
        #[max_length = 255]
        transaction -> Nullable<Varchar>,
        service_time -> Float,
    }
}

diesel::table! {
    teller (teller_id) {
        #[max_length = 255]
        teller_id -> Varchar,
        #[max_length = 255]
        transaction -> Nullable<Varchar>,
        service_time -> Nullable<Float>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    Transaction,
    Users,
    teller,
);
