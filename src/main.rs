use actix_web::{web, App, HttpServer};
use routes::ships::{get_ships, update_ship, add_ship, drop_ships_table, get_ship};
mod routes;
mod models;
use std::time::Duration;
use sqlx::{postgres::PgPoolOptions};

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url: String = std::env::var("DATABASE_URL").expect("Could not find database URL");

    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::new(5,0))
        .connect(&database_url)
        .await?;

    HttpServer::new(move || {
        App::new()
            .route("/ships", web::get().to(get_ships::get_ships))
            .route("/ships", web::post().to(add_ship::add_ship))
            .route("/ships", web::delete().to(drop_ships_table::drop_ships_table))
            .route("/ships/{name}", web::get().to(get_ship::get_ship))
            .route("/ships/{name}", web::patch().to(update_ship::update_ship))
            .route("/database", web::post().to(routes::database::add_column::add_column))
            .app_data(web::Data::new(pool.clone()))
    })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await?;

    Ok(())
        
}