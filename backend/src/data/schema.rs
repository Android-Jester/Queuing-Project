// @generated automatically by Diesel CLI.

diesel::table! {
    Clients (id) {
        id -> Integer,
        name -> Text,
        #[max_length = 255]
        account_number -> Varchar,
        #[max_length = 16]
        national_id -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::table! {
    Guests_Clients (national_id) {
        #[max_length = 15]
        national_id -> Varchar,
        name -> Text,
        transaction_detail -> Text,
        telephone_num -> Text,
    }
}

diesel::table! {
    MainQueue (national_id) {
        #[max_length = 16]
        national_id -> Varchar,
        position -> Integer,
        name -> Text,
        sub_queue_position -> Integer,
        #[max_length = 255]
        assigned_server -> Varchar,
        server_location -> Integer,
        #[max_length = 255]
        activity -> Varchar,
        time_duration -> Integer,
        time_joined -> Timestamp,
    }
}

diesel::table! {
    Servers (server_id) {
        #[max_length = 255]
        server_id -> Varchar,
        station -> Integer,
        service_time -> Integer,
        #[max_length = 255]
        password -> Varchar,
        active -> Bool,
    }
}

diesel::table! {
    Transactions (id) {
        id -> Integer,
        detail -> Text,
        #[max_length = 255]
        server_id -> Varchar,
        #[max_length = 15]
        client_national_id -> Varchar,
        duration -> Float,
        created_date -> Timestamp,
    }
}

diesel::joinable!(MainQueue -> Servers (assigned_server));
diesel::joinable!(Transactions -> Servers (server_id));

diesel::allow_tables_to_appear_in_same_query!(
    Clients,
    Guests_Clients,
    MainQueue,
    Servers,
    Transactions,
);
