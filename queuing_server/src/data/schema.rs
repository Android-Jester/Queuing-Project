// @generated automatically by Diesel CLI.

diesel::table! {
    transaction (id) {
        id -> Integer,
        #[max_length = 255]
        transaction_detail -> Nullable<Varchar>,
        #[max_length = 255]
        server_id -> Nullable<Varchar>,
        #[max_length = 255]
        user_account_number -> Nullable<Varchar>,
        duration -> Nullable<Float>,
        transaction_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (account_number) {
        #[max_length = 255]
        account_number -> Varchar,
    }
}

diesel::joinable!(transaction -> users (user_account_number));

diesel::allow_tables_to_appear_in_same_query!(
    transaction,
    users,
);
