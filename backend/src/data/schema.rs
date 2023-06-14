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
