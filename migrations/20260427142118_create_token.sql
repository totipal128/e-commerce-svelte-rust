-- Add migration script here
CREATE TABLE tokens
(
    id                    SERIAL PRIMARY KEY,
    user_id               INTEGER REFERENCES users (id) ON DELETE CASCADE      NOT NULL,
    access_token          VARCHAR(255)                                         DEFAULT NULL,
    refresh_token         VARCHAR(255)                                         DEFAULT NULL,
    expired_access_token  TIMESTAMPTZ                                          DEFAULT NOW(),
    expired_refresh_token TIMESTAMPTZ                                          DEFAULT NOW(),
    otp                   VARCHAR(6)                                              DEFAULT NULL,
    expired_otp           TIMESTAMPTZ                                          DEFAULT NOW(),
    user_agent            VARCHAR(255)                                         DEFAULT NULL, -- untuk mobile atau desktop
    ip_address            VARCHAR(255)                                         DEFAULT NULL,
    latitude              FLOAT                                                DEFAULT NULL,
    longitude             FLOAT                                                DEFAULT NULL,


    is_active             BOOLEAN                                              DEFAULT TRUE,
    is_mobile             BOOLEAN                                              DEFAULT FALSE, 
    created_at            TIMESTAMPTZ                                          DEFAULT NOW()
);