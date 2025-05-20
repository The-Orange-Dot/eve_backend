use actix_web::{HttpResponse, web};
use serde::{Deserialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct ShipUpdate {
    name: Option<String>,
    info: Option<Value>,
    attributes: Option<Value>,
    fitting: Option<Value>,
    capacitor: Option<Value>,
    targeting: Option<Value>,
    shields: Option<Value>,
    armor: Option<Value>,
    hull: Option<Value>,
    inventory: Option<Value>,
    navigation: Option<Value>,
    variants: Option<Value>,
}

// Creates a macro for providing the type to sqlx explicitly
macro_rules! bind_if_some {
    ($builder:ident, $field:expr) => {
        if let Some(ref value) = $field {
            $builder = $builder.bind(value);
            true
        } else {
            false
        }
    };
    ($builder:ident, $field:expr, $param_count:ident) => {
        if let Some(_) = $field {
            $param_count += 1;
            bind_if_some!($builder, $field)
        } else {
            false
        }
    };
}

// Allows setting the the array for
enum LoopingValues<'a> {
    String(&'a Option<String>),
    Json(&'a Option<Value>),
}

pub async fn update_ship(ship_name: web::Path<String>, update_data: web::Json<ShipUpdate>,pool: web::Data<sqlx::Pool<sqlx::Postgres>>) -> Result<HttpResponse, actix_web::Error> {
  let ship_name = &ship_name.into_inner();
  let update_data = &update_data.into_inner();

  // Checks to see if ship exists first - Returns error if ship cannot be found
  let ship_exists = sqlx::query_scalar::<_, bool>(
    "SELECT EXISTS(SELECT 1 FROM ships WHERE name = $1)"
  )
    .bind(ship_name)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|err| {
      eprintln!("Ship doesn't exist in database: {}", err);
      actix_web::error::ErrorNotFound("Ship does not exist in database.")
    })?;

    if !ship_exists {
      return Err(actix_web::error::ErrorNotFound("Ship not found."))
    } else {
      println!("Ship found. Updating ship info...");
    }
    
  // Dynamic update query
  let mut query = String::from("UPDATE ships SET ");
  // This enables both String(text) and Value(jsonb) types -> Use Box::new(foo) in push functions.
  let mut params_count = 1;
  let mut has_updates = false;

  let fields = [
    ("name", LoopingValues::String(&update_data.name)),
    ("info", LoopingValues::Json(&update_data.info)),
    ("attributes", LoopingValues::Json(&update_data.attributes)),
    ("fitting", LoopingValues::Json(&update_data.fitting)),
    ("capacitor", LoopingValues::Json(&update_data.capacitor)),
    ("targeting", LoopingValues::Json(&update_data.targeting)),
    ("shields", LoopingValues::Json(&update_data.shields)),
    ("armor", LoopingValues::Json(&update_data.armor)),
    ("hull", LoopingValues::Json(&update_data.hull)),
    ("inventory", LoopingValues::Json(&update_data.inventory)),
    ("navigation", LoopingValues::Json(&update_data.navigation)),
    ("variants", LoopingValues::Json(&update_data.variants)),
  ];

  for (field_name, field_value) in &fields {
      match field_value {
        LoopingValues::String(opt) if opt.is_some() => {
            query.push_str(&format!("{} = ${}, ", field_name, params_count));
            params_count += 1;
            has_updates = true;
        }
        LoopingValues::Json(opt) if opt.is_some() => {
            query.push_str(&format!("{} = ${}, ", field_name, params_count));
            params_count += 1;
            has_updates = true;
        }
        _ => {}
      }
  }

  // Returns if there is not data in the request body
  if !has_updates {
    return Ok(HttpResponse::Ok().json(serde_json::json!({
      "status": "success",
      "message": "No fields updated"
    })))
  }

  // Removes the trailing comma if there is a request, otherwise will return
  if params_count > 1 {
    query.pop();
    query.pop(); // This removes the ", "
  } else {
    return Ok(HttpResponse::Ok().json(serde_json::json!({
      "status": "success",
      "message": "No fields updated"
    })))
  }

  query.push_str(&format!(" WHERE name = ${}", params_count));

  let mut query_builder = sqlx::query(&query);
  
  // Macro to bind parameters to query
  bind_if_some!(query_builder, update_data.name);
  bind_if_some!(query_builder, update_data.info);
  bind_if_some!(query_builder, update_data.attributes);
  bind_if_some!(query_builder, update_data.fitting);
  bind_if_some!(query_builder, update_data.capacitor);
  bind_if_some!(query_builder, update_data.targeting);
  bind_if_some!(query_builder, update_data.shields);
  bind_if_some!(query_builder, update_data.armor);
  bind_if_some!(query_builder, update_data.hull);
  bind_if_some!(query_builder, update_data.inventory);
  bind_if_some!(query_builder, update_data.navigation);
  bind_if_some!(query_builder, update_data.variants);
  
  // Bind the ship ID
  query_builder = query_builder.bind(ship_name);

  query_builder
    .execute(pool.get_ref())
    .await
    .map_err(|err| {
      eprintln!("Error updating ship: {}", err);
      actix_web::error::ErrorInternalServerError("Internal database error")
    })?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
      "status": "success",
      "message": "Ship updated successfully"
    })))
}