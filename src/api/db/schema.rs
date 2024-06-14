// @generated automatically by Diesel CLI.

diesel::table! {
    mac_addresses (id) {
        id -> Int4,
        ssh_key -> Text,
        address_mac -> Text,
    }
}

diesel::table! {
    ports (id) {
        id -> Int4,
        mac_id -> Int4,
        port -> Int4,
        protocol -> Text,
        created -> Bool,
    }
}

diesel::joinable!(ports -> mac_addresses (mac_id));

diesel::allow_tables_to_appear_in_same_query!(
    mac_addresses,
    ports,
);
