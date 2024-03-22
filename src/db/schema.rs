// @generated automatically by Diesel CLI.

diesel::table! {
    mac_addresses (id) {
        id -> Nullable<Integer>,
        ssh_key -> Text,
        address_mac -> Text,
    }
}

diesel::table! {
    ports (id) {
        id -> Nullable<Integer>,
        mac_id -> Integer,
        port -> Integer,
        protocol -> Text,
        created -> Bool,
    }
}

diesel::joinable!(ports -> mac_addresses (mac_id));

diesel::allow_tables_to_appear_in_same_query!(
    mac_addresses,
    ports,
);
