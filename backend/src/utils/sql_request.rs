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

pub struct UserRow{
    pub id: Uuid,
    pub password_hash: String,
}

pub async fn find_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<UserRow>, sqlx::Error> {
    let rec = sqlx::query_as!(
        UserRow,
        r#"
        SELECT id, password_hash
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?;

    Ok(rec)
}