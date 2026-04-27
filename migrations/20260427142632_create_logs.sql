-- Add migration script here
CREATE TABLE logs_activity
(
    id            SERIAL PRIMARY KEY,
    user_id       INTEGER REFERENCES users (id) ON DELETE CASCADE      NOT NULL,
    user_agent    VARCHAR(255)                                         DEFAULT NULL,
    ip_address    VARCHAR(255)                                         DEFAULT NULL,
    latitude      FLOAT                                                DEFAULT NULL,
    longitude     FLOAT                                                DEFAULT NULL,
    action        VARCHAR(255)                                         DEFAULT NULL,
    module        VARCHAR(255)                                         DEFAULT NULL,
    path          VARCHAR(255)                                         DEFAULT NULL,
    method        VARCHAR(10)                                          DEFAULT NULL,
    status        INTEGER                                              DEFAULT NULL,
    messages      TEXT                                                 DEFAULT NULL,
    


    created_at    TIMESTAMPTZ                                          DEFAULT NOW(),
    updated_at    TIMESTAMPTZ                                          DEFAULT NOW(),
    deleted_at    TIMESTAMPTZ                                          DEFAULT NULL
);

   


CREATE TABLE logs_activity_time
(
    id            SERIAL PRIMARY KEY,
    user_id       INTEGER REFERENCES users (id) ON DELETE CASCADE      NOT NULL,
    minutes       INTEGER                                              DEFAULT 0,


    date_time     TIMESTAMPTZ                                          DEFAULT NOW()
);