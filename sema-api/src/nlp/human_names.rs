use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

use once_cell::sync::OnceCell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanNames {
  names: HashSet<String>,
}

impl HumanNames {
  pub fn new(data_path: &str) -> Result<Self> {
    let path = format!("{}/human_names.json", data_path);
    println!("path: {}", path);
    let names = File::open(&path)?;
    let reader = BufReader::new(names);
    let json: HashMap<String, usize> = serde_json::from_reader(reader)?;

    let names = json.into_keys().collect::<HashSet<String>>();

    Ok(HumanNames { names })
  }

  pub fn init(data_path: &str) {
    let human_names = HumanNames::new(data_path).expect("Unable to create HumanNames instance");
    HUMAN_NAMES.set(human_names).expect("Unable to set HUMAN_NAMES");
  }

  pub fn contains(possible_name: &str) -> bool {
    let human_names = HUMAN_NAMES
      .get()
      .expect("HUMAN_NAMES is not initialized");
      human_names
      .names
      .contains(possible_name)
  }
}

pub static HUMAN_NAMES: OnceCell<HumanNames> = OnceCell::new();
