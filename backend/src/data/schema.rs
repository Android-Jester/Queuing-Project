// @generated automatically by Diesel CLI.

diesel::table! {
    #[allow(non_snake_case)]
    Guests (national_id) {
        #[max_length = 15]
        national_id -> Varchar,
        name -> Text,
        #[max_length = 255]
        transaction_type -> Varchar,
        #[max_length = 10]
        telephone_num -> Varchar,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Tellers (server_id) {
        #[max_length = 255]
        server_id -> Varchar,
        server_station -> Integer,
        service_time -> Float,
        active -> Bool,
        #[max_length = 16]
        password -> Varchar,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Transactions (id) {
        id -> Integer,
        #[max_length = 255]
        transaction_detail -> Varchar,
        #[max_length = 255]
        server_id -> Varchar,
        #[max_length = 15]
        national_id -> Nullable<Varchar>,
        #[max_length = 15]
        guest_national_id -> Nullable<Varchar>,
        duration -> Float,
        transaction_time -> Timestamp,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Users (user_id) {
        user_id -> Integer,
        name -> Text,
        #[max_length = 255]
        account_number -> Varchar,
        #[max_length = 15]
        national_id -> Varchar,
        #[max_length = 16]
        password -> Varchar,
    }
}

diesel::joinable!(Transactions -> Guests (guest_national_id));
diesel::joinable!(Transactions -> Tellers (server_id));

diesel::allow_tables_to_appear_in_same_query!(Guests, Tellers, Transactions, Users,);
