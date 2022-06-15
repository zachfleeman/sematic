use link_parser_rust_bindings::{
  lp::{word::Word}
};

use crate::{
  nlp::sentence_parts::SentenceParts
};

const ONES: [&str; 20] = [
  "zero",
  "one",
  "two",
  "three",
  "four",
  "five",
  "six",
  "seven",
  "eight",
  "nine",
  "ten",
  "eleven",
  "twelve",
  "thirteen",
  "fourteen",
  "fifteen",
  "sixteen",
  "seventeen",
  "eighteen",
  "nineteen",
];
const TENS: [&str; 10] = [
  "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const ORDERS: [&str; 8] = [
  "zero",
  "hundred",
  "thousand",
  "million",
  "billion",
  "trillion",
  "quadrillion",
  "quintillion", // enough for u64::MAX
];

pub fn is_num_word(word: &str) -> bool {
  let tester = |w| ONES.contains(&w) || TENS.contains(&w) || ORDERS.contains(&w);

  if word.contains("-") {
    word
      .split("-")
      .all(tester)
  } else {
    tester(word)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextNumberTypes {
  Num(f32),
  Hundred,
  Thousand,
  Million,
  Billion,
  Trillion,
  Quadrillion,
  Quintillion,
  And,
  Unknown,
}

impl TextNumberTypes {
  pub fn from_text(text: &str) -> TextNumberTypes {
    let num_types = text
      .split("-")
      .into_iter()
      .map(|t| match t {
        "one" => TextNumberTypes::Num(1.),
        "two" => TextNumberTypes::Num(2.),
        "three" => TextNumberTypes::Num(3.),
        "four" => TextNumberTypes::Num(4.),
        "five" => TextNumberTypes::Num(5.),
        "six" => TextNumberTypes::Num(6.),
        "seven" => TextNumberTypes::Num(7.),
        "eight" => TextNumberTypes::Num(8.),
        "nine" => TextNumberTypes::Num(9.),
        "ten" => TextNumberTypes::Num(10.),
        "eleven" => TextNumberTypes::Num(11.),
        "twelve" => TextNumberTypes::Num(12.),
        "thirteen" => TextNumberTypes::Num(13.),
        "fourteen" => TextNumberTypes::Num(14.),
        "fifteen" => TextNumberTypes::Num(15.),
        "sixteen" => TextNumberTypes::Num(16.),
        "seventeen" => TextNumberTypes::Num(17.),
        "eighteen" => TextNumberTypes::Num(18.),
        "nineteen" => TextNumberTypes::Num(19.),
        "twenty" => TextNumberTypes::Num(20.),
        "thirty" => TextNumberTypes::Num(30.),
        "forty" => TextNumberTypes::Num(40.),
        "fifty" => TextNumberTypes::Num(50.),
        "sixty" => TextNumberTypes::Num(60.),
        "seventy" => TextNumberTypes::Num(70.),
        "eighty" => TextNumberTypes::Num(80.),
        "ninety" => TextNumberTypes::Num(90.),
        "zero" => TextNumberTypes::Num(0.),
        "hundred" => TextNumberTypes::Hundred,
        "thousand" => TextNumberTypes::Thousand,
        "million" => TextNumberTypes::Million,
        "billion" => TextNumberTypes::Billion,
        "trillion" => TextNumberTypes::Trillion,
        "quadrillion" => TextNumberTypes::Quadrillion,
        "quintillion" => TextNumberTypes::Quintillion,
        "and" => TextNumberTypes::And,
        _ => TextNumberTypes::Unknown,
      })
      .collect::<Vec<TextNumberTypes>>();

    if num_types.len() == 1 {
      return num_types[0].clone();
    }

    if num_types.len() == 2 {
      if num_types
        .iter()
        .all(|nt| matches!(nt, TextNumberTypes::Num(_)))
      {
        let mut total = 0.;

        for nt in num_types {
          if let TextNumberTypes::Num(num) = nt {
            total += num;
          }
        }

        return TextNumberTypes::Num(total);
      }
    }

    TextNumberTypes::Unknown
  }

  pub fn total(nums: Vec<TextNumberTypes>) -> f32 {
    nums
      .into_iter()
      .fold(0., |acc, nt| match nt {
        TextNumberTypes::Num(n) => acc + n,
        TextNumberTypes::Hundred => acc * 100.,
        TextNumberTypes::Thousand => acc * 1000.,
        TextNumberTypes::Million => acc * 1000000.,
        TextNumberTypes::Billion => acc * 1000000000.,
        TextNumberTypes::Trillion => acc * 1000000000000.,
        TextNumberTypes::Quadrillion => acc * 1000000000000000.,
        TextNumberTypes::Quintillion => acc * 1000000000000000000.,
        TextNumberTypes::And => acc,
        TextNumberTypes::Unknown => acc,
      })
  }
}

pub fn construct_number(word: &Word, part: &SentenceParts) -> Option<f32> {
  if !word.has_raw_disjunct("Dmcn+") && !word.has_raw_disjunct("ND+") {
    return None;
  }

  // attempt to parse the word text as a number. e.g. "123" -> 123
  if let Ok(num_val) = word
    .get_cleaned_word()
    .parse::<f32>()
  {
    return Some(num_val);
  }

  let mut num_words = vec![word];

  let prev_words = part
    .links
    .get_prev_words(word);

  for prev_word in prev_words
    .into_iter()
    .rev()
  {
    if prev_word.has_raw_disjunct("NN+") {
      num_words.push(prev_word);
      break;
    }

    if prev_word.has_raw_disjunct("NA+") {
      num_words.push(prev_word);
    }

    // if prev_word.has_raw_disjunct("Dmcn+") {
    //   num_words.push(prev_word);
    // }
  }

  let nt = num_words
    .into_iter()
    .rev()
    .map(|w| TextNumberTypes::from_text(&w.get_cleaned_word()))
    .collect::<Vec<TextNumberTypes>>();

  let total = TextNumberTypes::total(nt);


  Some(total)
}
