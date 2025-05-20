use actix_web::{HttpResponse, web, Error};
use crate::models::ship::Ship;

pub async fn add_ship(
    payload: web::Json<Ship>,
    pool: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> Result<HttpResponse, Error> {
    let new_ship: Ship = payload.into_inner();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS ships(
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL UNIQUE,
            info JSONB NOT NULL,
            attributes JSONB NOT NULL,
            fitting JSONB NOT NULL,
            capacitor JSONB NOT NULL,
            targeting JSONB NOT NULL,
            shields JSONB NOT NULL,
            armor JSONB NOT NULL,
            hull JSONB NOT NULL,
            inventory JSONB NOT NULL,
            navigation JSONB NOT NULL,
            variants JSONB[] DEFAULT '[]'::JSONB[]
        )
        "#
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Error creating table: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let _result = sqlx::query(
        r#"
        INSERT INTO ships (name, info, attributes, fitting, capacitor, targeting, shields, armor, hull, inventory, navigation, variants)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING id
        "#
    )
    .bind(&new_ship.name)
    .bind(serde_json::to_value(new_ship.info).unwrap())
    .bind(serde_json::to_value(new_ship.attributes).unwrap())
    .bind(serde_json::to_value(new_ship.fitting).unwrap())
    .bind(serde_json::to_value(new_ship.capacitor).unwrap())
    .bind(serde_json::to_value(new_ship.targeting).unwrap())
    .bind(serde_json::to_value(new_ship.shields).unwrap())
    .bind(serde_json::to_value(new_ship.armor).unwrap())
    .bind(serde_json::to_value(new_ship.hull).unwrap())
    .bind(serde_json::to_value(new_ship.inventory).unwrap())
    .bind(serde_json::to_value(new_ship.navigation).unwrap())
    .bind(serde_json::to_value(new_ship.variants).unwrap())
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Error inserting ship: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Created().body("Added ship to database"))
}