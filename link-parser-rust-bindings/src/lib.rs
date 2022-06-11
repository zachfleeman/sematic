#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate serde_derive;

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

// use nlprule::{rules_filename, tokenizer_filename, Rules, Tokenizer};

pub mod lp;
pub mod pos;


use lp::sentence::Sentence as LPSentence;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr, CString};

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkParserOptions {
  pub verbosity: i32,
  pub linkage_limit: i32,
  pub min_null_count: i32,
  pub max_null_count: i32,
  // null_block: i32,
  // islands_ok: bool,
  // short_length: i32,
  // all_short: bool,
  // display_short: bool,
  // display_word_subscripts: bool,
  // display_link_subscripts: bool,
  // display_walls: bool,
  // allow_null: bool,
  // echo_on: bool,
  // batch_mode: bool,
  // panic_mode: bool,
  // screen_width: i32,
  // display_on: bool,
  // display_postscript: bool,
  // display_bad: bool,
  // display_links: bool,
}

impl Default for LinkParserOptions {
  fn default() -> Self {
    LinkParserOptions {
      verbosity: 0,
      linkage_limit: 10000,
      min_null_count: 0,
      max_null_count: 0,
      // null_block: 1,
      // islands_ok: false,
      // short_length: 6,
      // all_short: false,
      // display_short: true,
      // display_word_subscripts: true,
      // display_link_subscripts: true,
      // display_walls: false,
      // allow_null: true,
      // echo_on: false,
      // batch_mode: false,
      // panic_mode: false,
      // screen_width: 79,
      // display_on: true,
      // display_postscript: false,
      // display_bad: false,
      // display_links: false,
    }
  }
}

#[derive(Error, Debug)]
pub enum LinkParserError {
  #[error("Unable to create a new LinkParser instance")]
  Create,

  #[error("Unable to create a str from a CStr word")]
  WordToStr,

  #[error("Unable to create a str from a CStr disjuncts")]
  DisjunctsToStr,

  #[error("Unable to tokenize sentence")]
  Tokenize,
}

pub struct LinkParser {
  dict: *mut Dictionary_s,

  opts: *mut Parse_Options_s,
}

unsafe impl Send for LinkParser {}

// I don't know if this is really needed...
impl fmt::Debug for LinkParser {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "LinkParser")
  }
}

impl LinkParser {
  pub fn new(lp_opts: LinkParserOptions) -> LinkParser {
    let dict = unsafe {
      let en = CString::new("en").expect("CString en failed");
      dictionary_create_lang(en.as_ptr())
    };

    let opts = unsafe {
      let po = parse_options_create();

      // TODO: Add the rest of the parser options
      parse_options_set_verbosity(po, lp_opts.verbosity);
      parse_options_set_linkage_limit(po, lp_opts.linkage_limit);
      parse_options_set_min_null_count(po, lp_opts.min_null_count);
      parse_options_set_max_null_count(po, lp_opts.max_null_count);

      po
    };

    LinkParser {
      dict,
      opts,
    }
  }

  pub fn parse_sentence(&self, sentence: &str) -> Result<Option<LPSentence>, LinkParserError> {
    let mut word_disjuncts_pairs = Vec::new();

    let s = CString::new(sentence.to_owned()).expect("CString sentence failed");

    unsafe {
      let sent = sentence_create(s.as_ptr(), self.dict);
      sentence_split(sent, self.opts);
      let n = sentence_parse(sent, self.opts);

      println!("{}", n);

      let linkage = linkage_create(0, sent, self.opts);

      let num_words = linkage_get_num_words(linkage);
      println!("num_words: {}", num_words);

      if num_words == 0 {
        return Ok(None);
      }

      let diagram = linkage_print_diagram(linkage, true, 800);
      printf(diagram);
      linkage_free_diagram(diagram);

      let a = linkage_print_disjuncts(linkage);
      printf(a);
      linkage_free_disjuncts(a);

      for i in 0..num_words {
        let word = linkage_get_word(linkage, i);
        let word: &CStr = CStr::from_ptr(word);
        let word = word
          .to_str()
          .map_err(|_e| LinkParserError::WordToStr)?
          .to_string();

        let disjuncts = linkage_get_disjunct_str(linkage, i);
        let disjuncts: &CStr = CStr::from_ptr(disjuncts);
        let disjuncts = disjuncts
          .to_str()
          .map_err(|_e| LinkParserError::DisjunctsToStr)?
          .split(" ")
          // .map(LPDisjunct::new)
          .map(|d| d.to_string())
          .collect::<Vec<String>>();
        // .collect::<Vec<LPDisjunct>>();

        // let lp_word = LPWord::new(&word, disjuncts);

        // lp_sentence.add_word(lp_word);
        word_disjuncts_pairs.push((word, disjuncts));
      }

      sentence_delete(sent);

      linkage_delete(linkage);
    };

    // let tokens = self
    //   .tokenizer
    //   .pipe(&sentence)
    //   .next()
    //   .ok_or(LinkParserError::Tokenize)?;

    // dbg!(&tokens);

    // let lp_sentence = Sentence::new(sentence.clone(), word_disjuncts_pairs, tokens);
    let lp_sentence = LPSentence::new(sentence, word_disjuncts_pairs);

    Ok(Some(lp_sentence))
  }
}

impl Drop for LinkParser {
  fn drop(&mut self) {
    unsafe {
      parse_options_delete(self.opts);
      dictionary_delete(self.dict);
    }
  }
}
