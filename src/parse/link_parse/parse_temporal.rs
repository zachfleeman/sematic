use anyhow::Result;

use link_parser_rust_bindings::{
  lp::{word::Word}
};

use crate::{
  nlp::sentence_parts::SentenceParts,
  sema::{
    sema_sentence::SemaSentence,
    symbol::Symbol,
    temporal::{Temporals, AbsoluteProperties, Absolute},
  },
};

use super::link_parse::ParseState;

pub static LONG_MONTHS: [&str; 12] = [
  "January",
  "February",
  "March",
  "April",
  "May",
  "June",
  "July",
  "August",
  "September",
  "October",
  "November",
  "December",
];

pub static SHORT_MONTHS: [&str; 12] = [
  "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/*
Examples:
October
October 1st
October 1st, 2019
2pm
2pm on October 1st
2pm on October 1st, 2019
October 1st, 2019 at 2pm
on the 1st of October
*/

#[derive(Debug, Clone, PartialEq)]
pub enum CalendarTypes {
  Gregorian, // BC
  Julian,    // AD
}

#[derive(Debug, Clone)]
pub struct IRYear {
  pub year: i32,
  pub calendar: CalendarTypes,
}

#[derive(Debug, Clone)]
pub enum TemporalIR {
  Month(usize),
  Day(usize), // zero-indexed, so 0 is the first day of the month
  Year(IRYear),
  Word(String),
  Punctuation,
  // Specific words
  Day_,
  Of_,
  // Everything else
  NA
}

impl TemporalIR {
  pub fn from_word(word: &Word) -> TemporalIR {
    if word.has_raw_disjunct("Xd+") {
      return TemporalIR::Punctuation;
    }

    if let Some(month_index) = get_month_index(word) {
      return TemporalIR::Month(month_index);
    }

    if let Some(day) = get_day(word) {
      return TemporalIR::Day(day);
    }

    if let Some(year) = get_year(word) {
      return TemporalIR::Year(year);
    }

    // TemporalIR::Word(word.get_cleaned_word())
    match word.get_cleaned_word().as_str() {
      "day" => TemporalIR::Day_,
      "of" => TemporalIR::Of_,
      _ => TemporalIR::NA,
    }
  }
}

#[derive(Debug, Clone)]
pub struct TemporalIRState {
  pub part: SentenceParts,
  pub current_index: usize,
  pub ir: Vec<TemporalIR>,
  pub groups: Vec<Vec<TemporalIR>>,
}

impl TemporalIRState {
  pub fn new(part: &SentenceParts) -> Result<Self> {
    let mut temporal_state = TemporalIRState {
      part: part.clone(),
      current_index: 0,
      ir: vec![],
      groups: vec![],
    };

    for word in part
      .links
      .words
      .iter()
    {
      let temporal_ir = TemporalIR::from_word(word);
      temporal_state
        .ir
        .push(temporal_ir);
    }

    // let mut groups = vec![];
    let mut current_group = None;

    for ir in temporal_state.ir.iter() {
      match ir {
        TemporalIR::NA => {
          if let Some(group) = current_group {
            temporal_state.groups.push(group);
          }
          current_group = None;
        }
        _ => {
          if current_group.is_none() {
            current_group = Some(vec![]);
          }

          current_group
            .as_mut()
            .unwrap()
            .push(ir.clone());
        }
      }
    }

    // dbg!(&temporal_state);



    Ok(temporal_state)
  }
}

pub fn parse_temporal(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
  _parse_state: &mut ParseState,
) -> Result<SemaSentence> {
  println!("Parse Temporal");
  let mut output_sentence = sema_sentence.clone();

  let temporal_ir_state = TemporalIRState::new(part)?;
  

  for group in temporal_ir_state.groups.iter() {
    match group[..] {
      // October
      [TemporalIR::Month(ir_month)] => {
        let temporal = Temporals::Absolute( Absolute {
          symbol: symbol.get_symbol(),
          properties: vec![
            AbsoluteProperties::Month { month: ir_month as i32 },
          ],
        });

        output_sentence.temporal.push(temporal);
      }
      [TemporalIR::Month(ir_month), TemporalIR::Day(ir_day)] => {
        let temporal = Temporals::Absolute( Absolute {
          symbol: symbol.get_symbol(),
          properties: vec![
            AbsoluteProperties::Month { month: ir_month as i32 }, 
            AbsoluteProperties::Day { day: ir_day as i32 },
          ],
        });

        output_sentence.temporal.push(temporal);
      },
      _ => ()
    };
  };

  Ok(output_sentence)
}

// helpers

pub fn is_month(word: &Word) -> bool {
  let word_text = word.get_cleaned_word();
  LONG_MONTHS
    .into_iter()
    .any(|month| month == word_text)
    || SHORT_MONTHS
      .into_iter()
      .any(|month| month == word_text)
}

pub fn get_month_index(word: &Word) -> Option<usize> {
  let word_text = word
    .get_cleaned_word()
    .to_lowercase();
  println!("{}", word_text);
  LONG_MONTHS
    .into_iter()
    .position(|month| month.to_lowercase() == word_text)
    .or_else(|| {
      SHORT_MONTHS
        .into_iter()
        .position(|month| month.to_lowercase() == word_text)
        .or_else(|| None)
    })
}

pub fn get_day(word: &Word) -> Option<usize> {
  let mut num = None;
  let word_text = word.get_cleaned_word();

  if let Ok(num_val) = word_text.parse::<usize>() {
    num = Some(num_val);
  }

  if word_text.ends_with("st") || word_text.ends_with("nd") || word_text.ends_with("rd") {
    if let Ok(num_val) = word_text[..word_text.len() - 2].parse::<usize>() {
      num = Some(num_val);
    }
  }

  if let Some(num_val) = num {
    if num_val < 1 || num_val > 31 {
      num = None;
    }
  }

  // zero index the day
  if let Some(num_val) = num {
    return Some(num_val - 1);
  }

  num
}

pub fn get_year(word: &Word) -> Option<IRYear> {
  let word_text = word.get_cleaned_word();
  if let Ok(num_val) = word_text.parse::<usize>() {
    if num_val > 0 && num_val < 2100 {
      return Some(IRYear {
        year: num_val as i32,
        calendar: CalendarTypes::Julian,
      });
    }
  }
  None
}
