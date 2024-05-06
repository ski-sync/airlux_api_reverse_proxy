-- Your SQL goes here
-- port is list of ports that are allowed to be used by the user

-- CREATE TABLE IF NOT EXISTS port (
--         id INTEGER PRIMARY KEY,
--         port INTEGER NOT NULL,
--         protocol TEXT NOT NULL,
--         created BOOLEAN NOT NULL
-- );

-- CREATE TABLE IF NOT EXISTS ports (
--         id INTEGER PRIMARY KEY,
--         address_mac TEXT NOT NULL,
--         port INTEGER NOT NULL,
--         protocol TEXT NOT NULL,
--         created BOOLEAN NOT NULL
-- );

CREATE TABLE IF NOT EXISTS mac_addresses (
    id SERIAL PRIMARY KEY,
    ssh_key TEXT NOT NULL,
    address_mac TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS ports (
    id SERIAL PRIMARY KEY,
    mac_id INTEGER NOT NULL,
    port INTEGER NOT NULL UNIQUE,
    protocol TEXT NOT NULL,
    created BOOLEAN NOT NULL,
    FOREIGN KEY(mac_id) REFERENCES mac_addresses(id),
    UNIQUE(mac_id, port, protocol)
);
