// Part of Speech enum
// https://www.abisource.com/projects/link-grammar/dict/introduction.html#3.3

use serde::{Deserialize, Serialize};

const SUBSCRIPTS: [&'static str; 51] = [
  "a", "a-c", "a-s", "b", "c", "d", "e", "eq", "eqn", "f", "g", "h", "i", "id", "ij", "j", "j-a",
  "j-c", "j-g", "j-m", "j-n", "j-o", "j-opnr", "j-q", "j-r", "j-ru", "j-sum", "j-v", "k", "l", "m",
  "n", "n-u", "o", "ord", "p", "q", "q-d", "r", "s", "t", "ti", "tz", "u", "v", "v-d", "w", "w-d",
  "x", "y", "z",
];

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum POS {
  // .a: Adjective
  Adjective,

  // .a-c: Adjective, comparative/relative
  AdjectiveComparative,

  // .a-s: Adjective, superlative
  AdjectiveSuperlative,

  // .b: Given names that can be masculine or feminine
  GivenName,

  // .c Currency names
  CurrencyName,

  // .d: (Not used/not defined)
  Undefined,

  // .e: Adverbs
  Adverb,

  // .eq .eqn: Binary operators e.g. 2 + 2
  BinaryOperator,

  // .f: Given names that are always feminine
  GivenNameFeminine,

  // .g Gerund
  Gerund,

  // .h Hesitation markers, fillers, planners
  HesitationMarker,

  // .i Misc usage, mostly pertaining to units, lengths, times.
  Misc,

  // .id: Identifiers: e.g. "vitamin A"
  Identifier,

  // .ij: Interjections, fillers
  Interjection,

  // .j: Conjunctions.
  Conjunction,

  // .j-a: Conjunctions -- adjectives: "the black and white cat"
  ConjunctionAdjective,

  // .j-c: Conjunctions -- comparatives: "he is bigger, and badder, than the pope."
  ConjunctionComparative,

  // .j-g: Conjunctions - proper names: e.g. "The Great Southern and Western Railroad"
  ConjunctionProperName,

  // .j-m: Conjunctions -- post-nominal modifiers
  ConjunctionPostNominalModifier,

  // .j-n: Conjunctions -- nouns: "Jack and Jill"
  ConjunctionNoun,

  // .j-o: Conjunctions -- ditransitive e.g. "I gave Bob a doll and Mary a gun"
  ConjunctionDitransitive,

  // .j-opnr: Clause openers -- e.g. "but you are wrong!"
  ClauseOpener,

  // .j-q: Conjunctions -- Conjoined question words.
  ConjunctionQuestionWords,

  // .j-r: Conjunctions -- adverbs/prepositional phrases e.g. "the man for whom and with whom ..."
  ConjunctionsAdPrep,

  // .j-ru: Conjunctions -- interval e.g. "two to threefold more abundant"
  ConjunctionInterval,

  // .j-sum: Conjunctions -- numerical sums: e.g. "It's a hundred and two in the shade."
  ConjunctionNumericalSum,

  // .j-v: Conjunctions -- verbs: "sang and danced"
  ConjunctionVerb,

  // .k: (Not used/not defined)
  NotUsed,

  // .l: Location (cities, states, towns, etc.)
  Location,

  // .m: Given names that are always masculine
  GivenNameMasculine,

  // .n: Noun
  Noun,

  // .n-u: Noun, uncountable (mass noun)
  NounUncountable,

  // .o: Organizations (corporations)
  Organization,

  // .ord: Ordinal numbers e.g. first second third
  OrdinalNumber,

  // .p: Plural count nouns
  PluralCountNoun,

  // .q: verb, Question-related or paraphrasing
  QuestionVerb,

  // .q-d: verb, past tense
  QuestionPastTense,

  // .r: Prepositions and related
  Preposition,

  // .s: Singular, mass or count nouns
  SingularMassNoun,

  // .t: Titles, roles. e.g. President, Captain
  Title,

  // .ti: Time, date e.g. AM, PM, December 2nd
  TimeDate,
  
  // .tz: Time-zones e.g. CDT, UTC
  TimeZone,
  
  // .u: Units of measurement
  UnitOfMeasurement,
  
  // .v: Verb
  Verb,
  
  // .v-d: Verb, past tense
  VerbPastTense,
  
  // .w: Verb
  Verb2,
  
  // .w-d: Verb, past tense
  VerbPastTense2,
  
  // .x: Prefix abbreviations, e.g. Mr., Dr., Mrs.
  PrefixAbbreviation,
  
  // .y: Postfix abbreviations, e.g. Ave., St., Co.
  PostfixAbbreviation,
  
  // .z: (Not used/not defined)
  NotUsed2,

  // left wall
  LeftWall,

  // right wall
  RightWall,
}

impl POS {
  pub fn from_lp_word(lp_word: &str) -> Option<POS> {
    let mut split = lp_word.split(".");

    match split.nth(0) {
        Some(word) => {
          match word {
            "LEFT-WALL" => return Some(POS::LeftWall),
            "RIGHT-WALL" => return Some(POS::RightWall),
            _ => {}
          }
        },
        None => todo!(),
    };

    match split.nth(0)
    {
      Some(subscript) => {
        if SUBSCRIPTS.contains(&subscript) {
          POS::from_subscript(&subscript)
        } else {
          None
        }
      }
      None => None,
    }
  }

  pub fn from_subscript(subscript: &str) -> Option<POS> {
    match subscript {
      "a" => Some(POS::Adjective),
      "a-c" => Some(POS::AdjectiveComparative),
      "a-s" => Some(POS::AdjectiveSuperlative),
      "b" => Some(POS::GivenName),
      "c" => Some(POS::CurrencyName),
      "d" => Some(POS::Undefined),
      "e" => Some(POS::Adverb),
      "eq" => Some(POS::BinaryOperator),
      "eqn" => Some(POS::BinaryOperator),
      "f" => Some(POS::GivenNameFeminine),
      "g" => Some(POS::Gerund),
      "h" => Some(POS::HesitationMarker),
      "i" => Some(POS::Misc),
      "id" => Some(POS::Identifier),
      "ij" => Some(POS::Interjection),
      "j" => Some(POS::Conjunction),
      "j-a" => Some(POS::ConjunctionAdjective),
      "j-c" => Some(POS::ConjunctionComparative),
      "j-g" => Some(POS::ConjunctionProperName),
      "j-m" => Some(POS::ConjunctionPostNominalModifier),
      "j-n" => Some(POS::ConjunctionNoun),
      "j-o" => Some(POS::ConjunctionDitransitive),
      "j-opnr" => Some(POS::ClauseOpener),
      "j-q" => Some(POS::ConjunctionQuestionWords),
      "j-r" => Some(POS::ConjunctionsAdPrep),
      "j-ru" => Some(POS::ConjunctionInterval),
      "j-sum" => Some(POS::ConjunctionNumericalSum),
      "j-v" => Some(POS::ConjunctionVerb),
      "k" => Some(POS::NotUsed),
      "l" => Some(POS::Location),
      "m" => Some(POS::GivenNameMasculine),
      "n" => Some(POS::Noun),
      "n-u" => Some(POS::NounUncountable),
      "o" => Some(POS::Organization),
      "ord" => Some(POS::OrdinalNumber),
      // NOTE: There is something weird with .p, in that it's used for "I" and "my"
      "p" => Some(POS::PluralCountNoun),
      "q" => Some(POS::QuestionVerb),
      "q-d" => Some(POS::QuestionPastTense),
      "r" => Some(POS::Preposition),
      "s" => Some(POS::SingularMassNoun),
      "t" => Some(POS::Title),
      "ti" => Some(POS::TimeDate),
      "tz" => Some(POS::TimeZone),
      "u" => Some(POS::UnitOfMeasurement),
      "v" => Some(POS::Verb),
      "v-d" => Some(POS::VerbPastTense),
      "w" => Some(POS::Verb2),
      "w-d" => Some(POS::VerbPastTense2),
      "x" => Some(POS::PrefixAbbreviation),
      "y" => Some(POS::PostfixAbbreviation),
      "z" => Some(POS::NotUsed2),
      _ => None,
    }
  }
}
