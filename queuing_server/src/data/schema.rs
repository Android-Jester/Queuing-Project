// @generated automatically by Diesel CLI.

diesel::table! {
    guests (national_id) {
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        action -> Nullable<Varchar>,
        #[max_length = 255]
        national_id -> Varchar,
    }
}

diesel::table! {
    teller (server_id) {
        #[max_length = 255]
        server_id -> Varchar,
        server_station -> Integer,
        service_time -> Float,
        active -> Nullable<Bool>,
    }
}

diesel::table! {
    transaction (id) {
        id -> Integer,
        #[max_length = 255]
        transaction_detail -> Varchar,
        #[max_length = 255]
        server_id -> Varchar,
        #[max_length = 255]
        user_account_number -> Varchar,
        duration -> Float,
        transaction_time -> Timestamp,
    }
}

diesel::table! {
    users (account_number) {
        #[max_length = 255]
        account_number -> Varchar,
    }
}

diesel::joinable!(transaction -> teller (server_id));
diesel::joinable!(transaction -> users (user_account_number));

diesel::allow_tables_to_appear_in_same_query!(
    guests,
    teller,
    transaction,
    users,
);
