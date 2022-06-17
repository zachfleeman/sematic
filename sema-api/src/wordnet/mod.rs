pub mod wordnet_verbs;
pub mod wordnet_nouns;
pub mod wordnet_noun_objects;

use self::wordnet_verbs::WordnetVerbs;
use self::wordnet_nouns::WordnetNouns;
use self::wordnet_noun_objects::init_wordnet_noun_objects;

pub fn init_wordnet_cells(data_path: &str) {
  WordnetVerbs::init(data_path);
  WordnetNouns::init(data_path);
  init_wordnet_noun_objects(data_path).expect("Failed to init wordnet noun objects");
}