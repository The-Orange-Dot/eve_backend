use actix_web::{HttpResponse, web};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Column {
  table: String,
  column_name: String,
  column_type: String
}

pub async fn add_column(payload: web::Json<Column>, pool: web::Data<sqlx::Pool<sqlx::Postgres>>) -> Result<HttpResponse, actix_web::Error> {
  let new_column: Column = payload.into_inner();

  sqlx::query(
    &format!("ALTER TABLE {} ADD COLUMN IF NOT EXISTS {} {}",
      new_column.table, new_column.column_name, new_column.column_type
    )
  ).execute(pool.get_ref())
  .await
  .map_err(|err| {
    eprint!("Error adding column {} to table {}: {}", new_column.column_name, new_column.table, err);
    actix_web::error::ErrorInternalServerError("Error adding column to table.")

  })?;

  Ok(HttpResponse::Ok().json(serde_json::json!({
    "status": "success",
    "message": format!("Added column '{}' to table '{}'.", new_column.column_name, new_column.table)
  })))

}
