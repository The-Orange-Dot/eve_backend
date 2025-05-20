use serde::{Deserialize, Serialize};
use validator::{Validate};
use std::str::FromStr;
use std::fmt;


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Size {
  Small,
  Medium,
  Large,
  Capital
}

// Adds validation for small, medium, or large
impl FromStr for Size {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "small" => Ok(Size::Small),
      "medium" => Ok(Size::Medium),
      "large" => Ok(Size::Large),
      "capital" => Ok(Size::Capital),
      _ => Err(format!("Invalid size: '{}'. Must be 'small,' 'medium,' or 'large', or 'capital'", s))
    }
  }
}

// Allows display
impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Capacitor {
  pub capacity: u32,
  pub recharge_time: f32,
}

#[derive(Deserialize, Serialize)]
pub struct Fitting {
  pub high_slots: u8,
  pub med_slots: u8,
  pub low_slots: u8,
  pub turret_slots: u8,
  pub launcher_slots: u8,
  pub drones: u8,
  pub drone_bandwidth: u32,
  pub powergrid: u32,
  pub cpu: u32,
  pub rig_calibration: u32,
  pub rig_slots: u8,
  pub rig_size: Size
}

#[derive(Deserialize, Serialize)]
pub struct Attributes {
  pub long_axis: u32,
  pub class: String
}

#[derive(Deserialize, Serialize)]
pub struct Targeting {
  pub max_target_range: f32,
  pub max_locked_targets: u8,
  pub signature_radius: u32,
  pub scan_resolution: u32,
  pub radar_sensor_strength: u8,
  pub magnetometric_sensor_strength: u8,
  pub gravimetric_sensor_strength: u8,
  pub ladar_sensor_strength: u8
}

#[derive(Deserialize, Serialize)]
pub struct Navigation {
  pub max_velocity: u32,
  pub inertia_modifier: f32,
  pub warp_speed: f32,
  pub mass: u64,
  pub align_time: f32,
}

#[derive(Deserialize, Serialize)]
pub struct Shields {
  capacity: u32,
  recharge_time: f32,
  em_resistance: u8,
  thermal_resistance: u8,
  kinetic_resistance: u8,
  explosive_resistance: u8,
}

#[derive(Deserialize, Serialize)]
pub struct Armor {
  capacity: u32,
  em_resistance: u8,
  thermal_resistance: u8,
  kinetic_resistance: u8,
  explosive_resistance: u8,
}

#[derive(Deserialize, Serialize)]
pub struct Hull {
  capacity: u32,
  em_resistance: u8,
  thermal_resistance: u8,
  kinetic_resistance: u8,
  explosive_resistance: u8,
}

#[derive(Deserialize, Serialize)]
pub struct Inventory {
  pub frigate_bay: u8,
  pub capacity: u32,
  pub volume: u64,
  pub volume_packaged: u64,
}

#[derive(Deserialize, Serialize)]
pub struct Info {
  pub description: String,
  pub quote: String,
  pub manufacturer: String,
  pub faction: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Variant {
  pub id: Option<i32>,
  pub base_ship_id: i32,
  pub variant_ship_id: i32,
  pub variant_role: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct Ship {
  pub id: Option<i32>,
  #[validate(length(min = 3))]
  pub name: String,
  pub attributes: Attributes,
  pub info: Info,
  pub fitting: Fitting,
  pub capacitor: Capacitor,
  pub targeting: Targeting,
  pub navigation: Navigation,
  pub shields: Shields,
  pub armor: Armor,
  pub hull: Hull,
  pub inventory: Inventory,
  pub variants: Vec<Variant>,
}