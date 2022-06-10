use convert_case::{Case, Casing};

use std::string::ToString;
use strum_macros::Display;

use super::symbol::Symbol;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "agent_type")]
pub enum Agents {
  Ego(Ego),
  Person(Person),
  Group(Group),
  Company(Company),
  // Organization,
}

impl Agents {
  pub fn get_symbol(&self) -> String {
    match self {
      Self::Ego(ego) => ego.symbol.to_owned(),
      Self::Person(p) => p.symbol.to_owned(),
      Self::Group(g) => g.symbol.to_owned(),
      Self::Company(c) => c.symbol.to_owned(),
    }
  }

  pub fn is_ego(&self) -> bool {
    match self {
      Self::Ego(_) => true,
      _ => false,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ego {
  // #[dummy(faker = "FakeString(COMMANDING_AGT_symbol)")]
  pub symbol: String,
  pub properties: Vec<AgentProperties>,
}

impl Ego {
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
// #[serde(tag = "agent_property_type")]
pub enum AgentProperties {
  Modifier {
    modifier_type: String, // category (chromatic)
    modifier: Option<String>, // actual language used (colored)
    amplifiers: Vec<String>, // like satellites (hot pink)
  },
}

impl AgentProperties {
  pub fn new_modifier(modifier_type: String, modifier: Option<String>) -> Self {
    Self::Modifier {
      modifier_type: modifier_type.to_case(Case::Snake),
      modifier,
      amplifiers: Vec::new(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
  pub symbol: String,
  pub properties: Vec<PersonProperties>,
}

impl Person {
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
// #[serde(tag = "agent_property_type")]
pub enum PersonProperties {
  // Subject is for targeting. Basically the opposite of "Ego"
  Subject { subject: bool },
  Name { name: String },
  FirstName { first_name: String },
  LastName { last_name: String },
  Email { email: String },
  PhoneNumber { phone_number: String },
  Address { address: String },
  Modifier {
    modifier_type: String, // category (chromatic)
    modifier: Option<String>, // actual language used (colored)
    amplifiers: Vec<String>, // like satellites (hot pink)
  },
  Gender {
    gender: Genders
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum Genders {
  #[strum(serialize = "male")]
  Male,
  #[strum(serialize = "female")]
  Female,

  #[strum(serialize = "other")]
  Other,

  #[strum(serialize = "unknown")]
  Unknown
}

impl PersonProperties {
  pub fn value(pp: &PersonProperties) -> String {
    match pp {
      PersonProperties::Subject { subject } => subject.to_string(),
      PersonProperties::Name { name } => name.to_owned(),
      PersonProperties::FirstName { first_name } => first_name.to_owned(),
      PersonProperties::LastName { last_name } => last_name.to_owned(),
      PersonProperties::Email { email } => email.to_owned(),
      PersonProperties::PhoneNumber { phone_number } => phone_number.to_owned(),
      PersonProperties::Address { address } => address.to_owned(),
      PersonProperties::Modifier {
        modifier_type,
        modifier,
        amplifiers,
      } => {
        let mut s = modifier_type.to_owned();

        for amplifier in amplifiers {
          s.push_str(&format!("{}_", amplifier));
        }

        if let Some(modifier) = modifier {
          s.push_str(&format!("{}", modifier));
        }

        s
      }
      PersonProperties::Gender { gender } => gender.to_string()
    }
  }

  pub fn new_modifier(modifier_type: String, modifier: Option<String>) -> Self {
    Self::Modifier {
      modifier_type: modifier_type.to_case(Case::Snake),
      modifier,
      amplifiers: Vec::new(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
  pub symbol: String,
  pub properties: Vec<GroupProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
// #[serde(tag = "group_property_type")]
pub enum GroupProperties {
  Name { name: String },
  Members { members: Vec<String> },
  Location { location: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
  pub symbol: String,
  pub properties: Vec<CompanyProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
// #[serde(tag = "company_property_type")]
pub enum CompanyProperties {
  Name { name: String },
  PhoneNumber { phone_number: String },
  Email { email: String },
  Industry { industry: String },
  Profession { profession: String },
}
