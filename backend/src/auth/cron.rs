pub fn revoked_token_cleanup(db: sqlx::PgPool) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1800));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        loop {
            interval.tick().await;
            if let Err(e) = sqlx::query!("DELETE FROM revoked_tokens WHERE expires_at < now()")
                .execute(&db)
                .await
            {
                eprintln!("revoked_tokens cleanup failed: {e}");
            }
        }
    });
}