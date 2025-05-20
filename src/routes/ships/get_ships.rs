use actix_web::{HttpResponse, web};
use sqlx::Row;
use crate::models::ship::{Ship};

pub async fn get_ships(pool: web::Data<sqlx::Pool<sqlx::Postgres>>) -> Result<HttpResponse, actix_web::Error> {
    let rows = sqlx::query("SELECT * FROM ships ORDER BY id")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Error fetching ships: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let ships: Result<Vec<Ship>, _> = rows.iter().map(|row| {
        Ok(Ship {
            id: Some(row.get::<i32, _>("id")),
            name: row.get::<String, _>("name"),
            info: serde_json::from_value(row.get::<serde_json::Value, _>("info"))?,
            attributes: serde_json::from_value(row.get::<serde_json::Value, _>("attributes"))?,
            fitting: serde_json::from_value(row.get::<serde_json::Value, _>("fitting"))?,
            capacitor: serde_json::from_value(row.get::<serde_json::Value, _>("capacitor"))?,
            targeting: serde_json::from_value(row.get::<serde_json::Value, _>("targeting"))?,
            shields: serde_json::from_value(row.get::<serde_json::Value, _>("shields"))?,
            armor: serde_json::from_value(row.get::<serde_json::Value, _>("armor"))?,
            hull: serde_json::from_value(row.get::<serde_json::Value, _>("hull"))?,
            inventory: serde_json::from_value(row.get::<serde_json::Value, _>("inventory"))?,
            navigation: serde_json::from_value(row.get::<serde_json::Value, _>("navigation"))?,
            variants: Vec::new()

        })
    }).collect();

    let ships = ships.map_err(|e: Box<dyn std::error::Error>| {
        eprintln!("Error deserializing ships: {}", e);
        actix_web::error::ErrorInternalServerError("Data processing error")
    })?;

    Ok(HttpResponse::Ok().json(ships))
}