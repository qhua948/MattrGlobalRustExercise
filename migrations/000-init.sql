CREATE TABLE cryptographic_keys
(
    id         INTEGER NOT NULL
        PRIMARY KEY AUTOINCREMENT
        UNIQUE,
    public_key BLOB
);

CREATE TABLE schemas
(
    id     INTEGER NOT NULL
        PRIMARY KEY AUTOINCREMENT
        UNIQUE,
    schema BLOB
);

CREATE TABLE credentials
(
    id            INTEGER NOT NULL
        PRIMARY KEY AUTOINCREMENT
        UNIQUE,
    schema_id     INTEGER NOT NULL,
    public_key_id INTEGER,
    data          BLOB,
    finger_print  BLOB,
    FOREIGN KEY (schema_id) REFERENCES schemas (id),
    FOREIGN KEY (public_key_id) REFERENCES cryptographic_keys (id)
);



