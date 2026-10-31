CREATE TABLE revoked_tokens (
    jti        UUID PRIMARY KEY,
    expires_at TIMESTAMPTZ NOT NULL
);
CREATE INDEX ON revoked_tokens (expires_at);