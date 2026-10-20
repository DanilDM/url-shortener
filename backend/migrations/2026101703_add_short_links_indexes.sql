-- Усі посилання користувача
CREATE INDEX short_links_user_created_idx
    ON short_links (user_id, created_at DESC);

-- Постійні посилання користувача
CREATE INDEX short_links_user_permanent_idx
    ON short_links (user_id, created_at DESC)
    WHERE expires_at IS NULL;