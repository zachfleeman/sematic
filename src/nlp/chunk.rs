
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
  pub pos: String,
  pub start: usize,
  pub end: usize,
  pub plural: Option<String>,
  pub phrase: String,
}

impl Chunk {
  pub fn new() -> Self {
    Chunk {
      pos: String::new(),
      start: 0,
      end: 0,
      plural: None,
      phrase: String::new(),
    }
  }

  pub fn from_chunk_type(chunk_type: String, start: usize) -> Self {
    let mut a = chunk_type.split("-");
    let _place = a.next().unwrap();
    let pos = a
      .next()
      .unwrap()
      .to_owned();
    let plural = a
      .next()
      .map(|p| p.to_owned());

    Chunk {
      pos,
      start,
      end: start,
      plural,
      phrase: String::new(),
    }
  }
}