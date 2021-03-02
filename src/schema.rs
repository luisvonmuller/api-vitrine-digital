table! {
    art (art_id) {
        art_id -> Integer,
        art_delivery_time -> Timestamp,
        art_image -> Text,
        client_demand_client_demand_id -> Integer,
    }
}

table! {
    client (client_id) {
        client_id -> Integer,
        client_name -> Varchar,
        client_mail -> Nullable<Varchar>,
        client_phone -> Nullable<Varchar>,
        client_uf -> Nullable<Varchar>,
        client_city -> Nullable<Varchar>,
        client_address -> Nullable<Varchar>,
    }
}

table! {
    client_demand (client_demand_id) {
        client_demand_id -> Integer,
        client_demand_date -> Timestamp,
        client_demand_status -> Integer,
        client_client_id -> Integer,
    }
}

table! {
    login (login_id) {
        login_id -> Integer,
        login_mail -> Varchar,
        login_password -> Varchar,
        logincol -> Nullable<Varchar>,
        client_client_id -> Integer,
    }
}

table! {
    product (product_id) {
        product_id -> Integer,
        product_description -> Varchar,
        product_value -> Nullable<Float>,
        product_second_value -> Nullable<Float>,
        product_image -> Nullable<Text>,
        client_demand_client_demand_id -> Integer,
    }
}

joinable!(art -> client_demand (client_demand_client_demand_id));
joinable!(client_demand -> client (client_client_id));
joinable!(login -> client (client_client_id));
joinable!(product -> client_demand (client_demand_client_demand_id));

allow_tables_to_appear_in_same_query!(
    art,
    client,
    client_demand,
    login,
    product,
);
