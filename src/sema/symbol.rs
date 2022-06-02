#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Symbol(i32);

impl Symbol {
  pub fn new(symbol: i32) -> Self {
    Self(symbol)
  }

  pub fn get_symbol(&self) -> String {
    format!("${}", self.0)
  }

  pub fn increment(&mut self) {
    self.0 += 1;
  }

  pub fn next_symbol(&mut self) -> String {
    self.increment();
    format!("${}", self.0)
  }
}