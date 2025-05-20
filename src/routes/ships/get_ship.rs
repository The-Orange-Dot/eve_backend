use sqlx::Row;
use actix_web::{web, HttpResponse};
use actix_web::error;
use crate::models::ship::{Ship, Variant};


pub async fn get_ship(pool: web::Data<sqlx::Pool<sqlx::Postgres>>, path: web::Path<(String,)>) -> Result<HttpResponse, actix_web::Error> {
  let name = path.into_inner().0;

  let ship = sqlx::query("SELECT * FROM ships WHERE name = $1")
    .bind(&name)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|err| {
      eprint!("Error fetching ship: {}", err);
      match err {
          sqlx::Error::RowNotFound => error::ErrorNotFound("Ship not found"),
          _ => error::ErrorInternalServerError("Internal database error"),
      }
    })?;

    let ship_id: i32 = ship.get("id");

    let variants= sqlx::query_as::<_, Variant>("SELECT * FROM variants WHERE base_ship_id = $1")
      .bind(ship_id)
      .fetch_all(pool.get_ref())
      .await
      .map_err(|err| {
        eprint!("Error fetching ship variations: {}", err);
        actix_web::error::ErrorInternalServerError("Error fetching ship variations.")
      })?;

    let res = Ship {
      id: Some(ship.get::<i32, _>("id")),
      name: ship.get::<String, _>("name"),
      info: serde_json::from_value(ship.get::<serde_json::Value, _>("info"))?,
      attributes: serde_json::from_value(ship.get::<serde_json::Value, _>("attributes"))?,
      fitting: serde_json::from_value(ship.get::<serde_json::Value, _>("fitting"))?,
      capacitor: serde_json::from_value(ship.get::<serde_json::Value, _>("capacitor"))?,
      targeting: serde_json::from_value(ship.get::<serde_json::Value, _>("targeting"))?,
      shields: serde_json::from_value(ship.get::<serde_json::Value, _>("shields"))?,
      armor: serde_json::from_value(ship.get::<serde_json::Value, _>("armor"))?,
      hull: serde_json::from_value(ship.get::<serde_json::Value, _>("hull"))?,
      inventory: serde_json::from_value(ship.get::<serde_json::Value, _>("inventory"))?,
      navigation: serde_json::from_value(ship.get::<serde_json::Value, _>("navigation"))?,
      variants // Adding this as is for relational table many-to-many
    };


    Ok(HttpResponse::Ok().json(res))
}
