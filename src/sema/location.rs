use strum_macros::{self, Display};
use super::symbol::Symbol;

/*
Need to handles:
- "from the mirror up"
- "In step 3"
- "behind his head "
 */

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "location_type")]
pub enum Locations {
  Relative(Relative),
  Country(Country),
  City(City),
  LatLong(LatLong),
  Direction(Direction),
}

impl Locations {
  pub fn get_symbol(&self) -> String {
    match self {
      Self::Relative(r) => r.symbol.to_owned(),
      Self::Country(c) => c.symbol.to_owned(),
      Self::City(c) => c.symbol.to_owned(),
      Self::LatLong(ll) => ll.symbol.to_owned(),
      Self::Direction(d) => d.symbol.to_owned(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
  pub symbol: String,
  pub properties: Vec<CountryProperties>,
}

// Relative locations

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relative {
  pub symbol: String,
  pub properties: Vec<RelativeProperties>,
}

impl Relative {
  pub fn new(symbol: &mut Symbol) -> Self {
    Self {
      symbol: symbol.next_symbol(),
      properties: vec![],
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
// #[serde(tag = "relative_location_type")]
pub enum RelativeProperties {
  RelativeLocation {
    relative_location: RelativeLocationTypes,
  },
  Entity {
    entity: String,
  },
  Agent {
    agent: String,
  },
  Location {
    location: String,
  },
  Temporal {
    temporal: String,
  },
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum RelativeLocationTypes {
  #[strum(serialize = "at")]
  At,
  #[strum(serialize = "in")]
  In,
  #[strum(serialize = "on")]
  On,
  #[strum(serialize = "above")]
  Above,
  #[strum(serialize = "below")]
  Below,
  #[strum(serialize = "between")]
  Between,
  #[strum(serialize = "behind")]
  Behind,
  #[strum(serialize = "under")]
  Under,
  #[strum(serialize = "in_front")]
  InFront,
  #[strum(serialize = "next_to")]
  NextTo,
  #[strum(serialize = "near")]
  Near,
  #[strum(serialize = "at_the_edge_of")]
  AtTheEdgeOf,
  #[strum(serialize = "in_direction_of")]
  InDirectionOf,
}

// Physical locations

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
// #[serde(tag = "country_property_type")]
pub enum CountryProperties {
  CountryName { country_name: String },
  CountryCode { country_code: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
  pub symbol: String,
  pub properties: Vec<CityProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
// #[serde(tag = "city_property_type")]
pub enum CityProperties {
  CityName { city_name: String },
  // CityPrefix { city_prefix: String },
  // CitySuffix { city_suffix: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatLong {
  pub symbol: String,
  pub properties: Vec<LatLongProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
// #[serde(tag = "lat_long_property_type")]
pub enum LatLongProperties {
  Latitude { latitude: String },
  Longitude { longitude: String },
  LatLong { latitude: String, longitude: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Direction {
  pub symbol: String,
  pub direction: Directions,
  pub amplifiers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Directions {
  Left,
  Right,
  Up,
  Down,
  Back,
  Front,
  North,
  South,
  East,
  West,
  Northeast,
  Northwest,
  Southeast,
  Southwest,
  UpRight,
  UpLeft,
  DownRight,
  DownLeft,
  BackRight,
  BackLeft,
  FrontRight,
  FrontLeft,
  UpBack,
  UpFront,
  DownBack,
  DownFront,
  LeftBack,
  LeftFront,
  RightBack,
  RightFront,
}
