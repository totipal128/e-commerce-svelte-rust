-- Add migration script here
CREATE TABLE group_message
(
    id                  SERIAL PRIMARY KEY, 
    name                VARCHAR(255)                                         DEFAULT NULL,
    description         VARCHAR(255)                                         DEFAULT NULL,
    creted_by           INTEGER REFERENCES users (id) ON DELETE SET NULL     DEFAULT NULL,
    updated_by          INTEGER REFERENCES users (id) ON DELETE SET NULL     DEFAULT NULL,


    created_at          TIMESTAMPTZ                                          DEFAULT NOW(),
    updated_at          TIMESTAMPTZ                                          DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ                                          DEFAULT NULL
);

CREATE TABLE user_group_message
(
    id                  SERIAL PRIMARY KEY,
    user_id             INTEGER REFERENCES users (id) ON DELETE CASCADE NOT NULL,
    group_message_id    INTEGER REFERENCES group_message (id) ON DELETE CASCADE NOT NULL,


    created_at    TIMESTAMPTZ                                          DEFAULT NOW(),
    updated_at    TIMESTAMPTZ                                          DEFAULT NOW(),
    deleted_at    TIMESTAMPTZ                                          DEFAULT NULL
);

CREATE TABLE messages
 (   id                  SERIAL PRIMARY KEY, 
    group_message_id    INTEGER REFERENCES group_message (id) ON DELETE CASCADE NOT NULL,
    created_by          INTEGER REFERENCES users (id) ON DELETE CASCADE NOT NULL,
    message             TEXT                                                 DEFAULT NULL,


    created_at          TIMESTAMPTZ                                          DEFAULT NOW(),
    updated_at          TIMESTAMPTZ                                          DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ                                          DEFAULT NULL
);

CREATE TABLE message_read
(
    id                  SERIAL PRIMARY KEY, 
    message_id          INTEGER REFERENCES messages (id) ON DELETE CASCADE NOT NULL,
    user_id             INTEGER REFERENCES users (id) ON DELETE CASCADE NOT NULL,


    created_at          TIMESTAMPTZ                                          DEFAULT NOW(),
    updated_at          TIMESTAMPTZ                                          DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ                                          DEFAULT NULL
);

CREATE TABLE message_file 
(
    id                  SERIAL PRIMARY KEY, 
    message_id          INTEGER REFERENCES messages (id) ON DELETE CASCADE NOT NULL,
    file_path           VARCHAR(255)                                         DEFAULT NULL,
    file_name           VARCHAR(255)                                         DEFAULT NULL,
    file_type           VARCHAR(255)                                         DEFAULT NULL,
    file_size           INTEGER                                              DEFAULT NULL,    


    created_at          TIMESTAMPTZ                                          DEFAULT NOW(),
    updated_at          TIMESTAMPTZ                                          DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ                                          DEFAULT NULL
);