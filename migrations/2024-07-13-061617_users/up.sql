CREATE TABLE
    IF NOT EXISTS user_types (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        name VARCHAR NOT NULL,
        slug VARCHAR NOT NULL
    );

INSERT INTO
    user_types
VALUES
    (1, "Public", "public"),
    (2, "Private", "private"),
    (3, "Admin", "admin"),
    (4, "Shadow Admin", "priv_admin");

CREATE TABLE
    IF NOT EXISTS account_statuss (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        name VARCHAR NOT NULL,
        slug VARCHAR NOT NULL
    );

INSERT INTO
    account_statuss
VALUES
    (1, "Unverified", "unverified"),
    (2, "verfified", "verified"),
    (3, "Suspended", "suspended"),
    (4, "Banned", "banned"),
    (5, "Deleted", "deleted");

CREATE TABLE
    IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        username VARCHAR NOT NULL UNIQUE,
        pswd VARCHAR NOT NULL,
        user_type_id INTEGER NOT NULL DEFAULT 1,
        account_status_id INTEGER NOT NULL DEFAULT 1,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (user_type_id) REFERENCES user_types (id),
        FOREIGN KEY (account_status_id) REFERENCES account_statuss (id)
    );