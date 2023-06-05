// @generated automatically by Diesel CLI.

diesel::table! {
    users (account_number) {
        #[max_length = 255]
        account_number -> Varchar,
        password -> Text,
    }
}
