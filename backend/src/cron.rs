pub fn run_all(db: sqlx::PgPool) {
    crate::auth::cron::revoked_token_cleanup(db);
}