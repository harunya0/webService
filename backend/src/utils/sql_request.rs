use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user(
    pool: &PgPool,
    username: &str,
    password_hash: &str,
) -> Result<Uuid, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        RETURNING id
        "#,
        username,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}