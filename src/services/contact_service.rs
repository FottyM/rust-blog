use sqlx::{sqlite::SqliteQueryResult, Pool, Sqlite};

use crate::models::contact_message::CreateContactMessageDto;

pub async fn create_contact_message(
    dto: CreateContactMessageDto,
    pool: &Pool<Sqlite>,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO contact_messages (name, email, message)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(dto.name)
    .bind(dto.email)
    .bind(dto.message)
    .execute(pool)
    .await
    .map_err(|err| {
        tracing::error!("Error creating contact: {}", err);
        err
    })
}
