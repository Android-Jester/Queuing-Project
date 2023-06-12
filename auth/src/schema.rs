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
    users (account_number) {
        #[max_length = 255]
        account_number -> Varchar,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    guests,
    users,
);
