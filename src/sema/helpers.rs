use convert_case::{Case, Casing};

pub fn to_symbol(s: &str) -> String {
  let ss = s.to_lowercase().to_case(Case::Snake);
  format!("${}", ss)
}