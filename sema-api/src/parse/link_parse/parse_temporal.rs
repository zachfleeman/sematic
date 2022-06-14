use anyhow::Result;

use link_parser_rust_bindings::lp::word::Word;

use crate::{
  nlp::sentence_parts::SentenceParts,
  sema::{
    sema_sentence::SemaSentence,
    symbol::Symbol,
    temporal::{Absolute, AbsoluteProperties, DaysOfWeek, Relative, RelativeProperties, Temporals},
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
  Month(i32, Word),
  Day(i32, Word), // zero-indexed, so 0 is the first day of the month
  Year(IRYear, Word),
  Word(String, Word),
  Punctuation,
  // Specific words
  Day_,
  Of_,
  Next_,
  Last_,
  // Everything else
  NA,
  DayOfWeek(DaysOfWeek, Word),
}

impl TemporalIR {
  pub fn from_word(word: &Word) -> TemporalIR {
    let w = word.clone();

    if word.has_raw_disjunct("Xd+") || word.has_raw_disjunct("Xx-") {
      return TemporalIR::Punctuation;
    }

    if let Some(month_index) = get_month_index(word) {
      return TemporalIR::Month(month_index, w);
    }

    if let Some(day) = get_day(word) {
      return TemporalIR::Day(day, w);
    }

    if let Some(year) = get_year(word) {
      return TemporalIR::Year(year, w);
    }

    if let Some(day_of_week) = get_day_of_week(word) {
      return TemporalIR::DayOfWeek(day_of_week, w);
    }

    // NOTE: this disjunct only applies if the following day of week is uppercase (e.g. "Monday")
    //       otherwise the disjunct could be "dCO+"
    if word.has_raw_disjunct("DTi+") {
      return match word
        .get_cleaned_word()
        .as_ref()
      {
        "next" => return TemporalIR::Next_,
        "last" => return TemporalIR::Last_,
        _ => TemporalIR::NA,
      };
    }

    // TemporalIR::Word(word.get_cleaned_word())
    match word
      .get_cleaned_word()
      .as_str()
    {
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

    let mut current_group = None;

    for ir in temporal_state
      .ir
      .iter()
    {
      match ir {
        TemporalIR::NA => {
          if let Some(group) = current_group {
            temporal_state
              .groups
              .push(group);
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

    // dbg!(&temporal_state.ir);
    // dbg!(&temporal_state.groups);

    Ok(temporal_state)
  }
}

pub fn parse_temporal(
  sema_sentence: &SemaSentence,
  part: &SentenceParts,
  symbol: &mut Symbol,
  parse_state: &mut ParseState,
) -> Result<SemaSentence> {
  let mut output_sentence = sema_sentence.clone();

  let temporal_ir_state = TemporalIRState::new(part)?;

  // NOTE: This match statement could use some refacoring
  for group in temporal_ir_state
    .groups
    .iter()
  {
    match &group[..] {
      // October
      [TemporalIR::Day(day, day_word), TemporalIR::Of_, TemporalIR::Month(month, month_word)] => {
        let temporal = Temporals::Absolute(Absolute {
          symbol: symbol.next_symbol(),
          properties: vec![
            AbsoluteProperties::Month { month: *month },
            AbsoluteProperties::Day { day: *day },
          ],
        });

        parse_state.add_symbol(
          &temporal.get_symbol(),
          vec![day_word.position, month_word.position],
        );

        output_sentence
          .temporal
          .push(temporal);
      }
      [TemporalIR::Month(month, month_word), TemporalIR::Day(day, day_word), TemporalIR::Year(year, year_word)] =>
      {
        let temporal = Temporals::Absolute(Absolute {
          symbol: symbol.next_symbol(),
          properties: vec![
            AbsoluteProperties::Month { month: *month },
            AbsoluteProperties::Day { day: *day },
            AbsoluteProperties::Year { year: year.year },
          ],
        });

        parse_state.add_symbol(
          &temporal.get_symbol(),
          vec![month_word.position, day_word.position, year_word.position],
        );

        output_sentence
          .temporal
          .push(temporal);
      }
      // December 28th, 2019
      [TemporalIR::Month(month, month_word), TemporalIR::Day(day, day_word), TemporalIR::Punctuation, TemporalIR::Year(year, year_word)] =>
      {
        let temporal = Temporals::Absolute(Absolute {
          symbol: symbol.next_symbol(),
          properties: vec![
            AbsoluteProperties::Month { month: *month },
            AbsoluteProperties::Day { day: *day },
            AbsoluteProperties::Year { year: year.year },
          ],
        });

        parse_state.add_symbol(
          &temporal.get_symbol(),
          vec![month_word.position, day_word.position, year_word.position],
        );

        output_sentence
          .temporal
          .push(temporal);
      }
      // next Tuesday
      [TemporalIR::Next_, TemporalIR::DayOfWeek(day_of_week, dow_word)] => {
        let dow_symbol = symbol.next_symbol();
        let next_symbol = symbol.next_symbol();

        let dow = Temporals::Absolute(Absolute {
          symbol: dow_symbol.to_owned(),
          properties: vec![AbsoluteProperties::DayOfWeek {
            day_of_week: day_of_week.to_owned(),
          }],
        });

        parse_state.add_symbol(&dow_symbol, vec![dow_word.position]);

        let rel_next = Temporals::Relative(Relative {
          symbol: next_symbol.to_owned(),
          properties: vec![RelativeProperties::Next {
            next: dow_symbol.clone(),
          }],
        });

        parse_state.add_symbol(&next_symbol, vec![dow_word.position - 1]);

        output_sentence
          .temporal
          .push(dow);

        output_sentence
          .temporal
          .push(rel_next);
      }
      // last Tuesday
      [TemporalIR::Last_, TemporalIR::DayOfWeek(day_of_week, dow_word)] => {
        let dow_symbol = symbol.next_symbol();
        let last_symbol = symbol.next_symbol();
        let dow = Temporals::Absolute(Absolute {
          symbol: dow_symbol.to_owned(),
          properties: vec![AbsoluteProperties::DayOfWeek {
            day_of_week: day_of_week.to_owned(),
          }],
        });

        parse_state.add_symbol(&dow_symbol, vec![dow_word.position]);

        let rel_last = Temporals::Relative(Relative {
          symbol: last_symbol.to_owned(),
          properties: vec![RelativeProperties::Previous {
            previous: dow_symbol.clone(),
          }],
        });

        parse_state.add_symbol(&last_symbol, vec![dow_word.position - 1]);

        output_sentence
          .temporal
          .push(dow);

        output_sentence
          .temporal
          .push(rel_last);
      }
      [TemporalIR::Month(month, month_word)] => {
        let temporal = Temporals::Absolute(Absolute {
          symbol: symbol.next_symbol(),
          properties: vec![AbsoluteProperties::Month { month: *month }],
        });

        parse_state.add_symbol(
          &temporal.get_symbol(),
          vec![month_word.position],
        );

        output_sentence
          .temporal
          .push(temporal);
      }
      [TemporalIR::Month(month, month_word), TemporalIR::Day(day, day_word)] => {
        let temporal = Temporals::Absolute(Absolute {
          symbol: symbol.next_symbol(),
          properties: vec![
            AbsoluteProperties::Month { month: *month },
            AbsoluteProperties::Day { day: *day },
          ],
        });

        parse_state.add_symbol(
          &temporal.get_symbol(),
          vec![month_word.position, day_word.position],
        );

        output_sentence
          .temporal
          .push(temporal);
      }
      [TemporalIR::Year(year, year_word)] => {
        let temporal = Temporals::Absolute(Absolute {
          symbol: symbol.next_symbol(),
          properties: vec![AbsoluteProperties::Year { year: year.year }],
        });

        parse_state.add_symbol(&temporal.get_symbol(), vec![year_word.position]);

        output_sentence
          .temporal
          .push(temporal);
      }
      _ => (),
    };
  }

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

pub fn get_month_index(word: &Word) -> Option<i32> {
  let word_text = word
    .get_cleaned_word()
    .to_lowercase();

  LONG_MONTHS
    .into_iter()
    .position(|month| month.to_lowercase() == word_text)
    .or_else(|| {
      SHORT_MONTHS
        .into_iter()
        .position(|month| month.to_lowercase() == word_text)
        .or_else(|| None)
    })
    .map(|index| index as i32)
}

pub fn get_day(word: &Word) -> Option<i32> {
  // the TM disjunct connects months to days.
  // Can this be used?
  if word.has_raw_disjunct("Dmcn+") {
    return None;
  }

  let mut num = None;
  let word_text = word.get_cleaned_word();

  if let Ok(num_val) = word_text.parse::<i32>() {
    num = Some(num_val);
  }

  if word_text.ends_with("st")
    || word_text.ends_with("nd")
    || word_text.ends_with("rd")
    || word_text.ends_with("th")
  {
    if let Ok(num_val) = word_text[..word_text.len() - 2].parse::<i32>() {
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

pub fn get_day_of_week(word: &Word) -> Option<DaysOfWeek> {
  let word_text = word.get_cleaned_word();

  DaysOfWeek::from_str(&word_text)
}

pub fn get_year(word: &Word) -> Option<IRYear> {
  // Not sure if this is the best thing to do.
  if word.has_raw_disjunct("Dmcn+") {
    return None;
  }

  // NOTE: years are kinda recognized by the link-parser with [!<YEAR-DATE>]
  // don't know if using it would be helpful here, but maybe...

  let word_text = word.get_cleaned_word();
  if let Ok(num_val) = word_text.parse::<i32>() {
    if num_val > 0 && num_val < 2100 {
      return Some(IRYear {
        year: num_val,
        calendar: CalendarTypes::Julian,
      });
    }
  }
  None
}