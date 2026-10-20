CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    username VARCHAR(32) NOT NULL,
    password_hash TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT users_username_length_check
        CHECK (char_length(username) BETWEEN 3 AND 32)
);

CREATE UNIQUE INDEX users_username_unique_idx
    ON users (LOWER(username));