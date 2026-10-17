CREATE TABLE short_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    user_id UUID NOT NULL,

    code VARCHAR(8) NOT NULL,
    original_url TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,

    click_count BIGINT NOT NULL DEFAULT 0,
    last_clicked_at TIMESTAMPTZ,

    CONSTRAINT short_links_user_fk
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE,

    CONSTRAINT short_links_code_length_check
        CHECK (char_length(code) = 8),

    CONSTRAINT short_links_click_count_check
        CHECK (click_count >= 0),

    CONSTRAINT short_links_expiration_check
        CHECK (expires_at IS NULL OR expires_at > created_at)
);

CREATE UNIQUE INDEX short_links_code_unique_idx
    ON short_links (code);