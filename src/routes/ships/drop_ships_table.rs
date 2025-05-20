use actix_web::{HttpResponse, web, Error};

pub async fn drop_ships_table (pool: web::Data<sqlx::Pool<sqlx::Postgres>>) -> Result<HttpResponse, Error>{
  sqlx::query("DROP TABLE ships").execute(pool.get_ref()).await.expect("Error dropping table: ships");

  Ok(HttpResponse::Ok().body("table dropped: ships"))
}