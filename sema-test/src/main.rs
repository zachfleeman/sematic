#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use anyhow::Result;
use clap::Parser;
use console::{style, Term};
use std::{env, fs::File, io::BufReader, panic};

use link_parser_rust_bindings::{LinkParser, LinkParserOptions};

use sema_api::{
  nlp::{
    init_nlp_cells,
    sentence_parts::{SentenceEncodings, SentenceParts, SentenceText},
  },
  process_sentences::process::process_parts,
  sema::sema_sentence::SemaSentence,
  wordnet::init_wordnet_cells,
};

#[derive(Parser, Debug)]
struct Args {}

#[derive(Debug, Serialize, Deserialize)]
struct TestCase {
  sentence: String,
  data: SemaSentence,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum TestResultStatus {
  Success,
  Failure,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestResult {
  status: TestResultStatus,
  sentence: String,
  expected: SemaSentence,
  actual: SemaSentence,
}

fn get_data_path() -> String {
  let mut pwd = env::current_dir().unwrap();
  pwd.push("../sema-api/data");

  pwd
    .as_os_str()
    .to_str()
    .unwrap()
    .to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
  let _args = Args::parse();

  let data_path = get_data_path();

  println!("data_path: {}", data_path);

  init_nlp_cells(&data_path);
  init_wordnet_cells(&data_path);

  // Link Parser (cannot be in a OnceCell!)
  let link_parser_ops = LinkParserOptions {
    verbosity: 0,
    ..LinkParserOptions::default()
  };

  let link_parser = LinkParser::new(link_parser_ops);

  let term = Term::stdout();

  term.write_line(&format!("hello {}", style("sema-tests").cyan()))?;

  let pwd = env::current_dir().unwrap();
  let pwd = pwd
    .to_str()
    .unwrap();
  println!("pwd: {}", pwd);
  // import test data
  let file = File::open("src/tests/tests_1.json")?;
  let reader = BufReader::new(file);
  let json_tests_1: Vec<TestCase> = serde_json::from_reader(reader)?;

  let mut results = vec![];

  run_json_test_cases(&link_parser, json_tests_1, &mut results).await?;

  let passes = results
    .iter()
    .filter(|r| r.status == TestResultStatus::Success)
    .count();
  let fails = results
    .iter()
    .filter(|r| r.status == TestResultStatus::Failure)
    .collect::<Vec<_>>();
  let fails_count = fails.len();

  println!("total: {}", results.len());
  println!("passed: {}", style(passes).green());
  println!("failed: {}", style(fails_count).red());

  if fails_count == 0 {
    term.write_line(&format!("{}", style("All tests passed").green()))?;
  } else {
    term.write_line(&format!("{}", style("Some tests failed").red()))?;

    dbg!(&fails);
  }

  // dbg!(&results);

  Ok(())
}

// TODO: This doesn't work yet.
// need to figure out how to best handle duckling temporal timestamps in tests.
async fn _load_temporal_tests() -> Result<Vec<TestCase>> {
  let pwd = env::current_dir().unwrap();
  let pwd = pwd
    .to_str()
    .unwrap();
  println!("pwd: {}", pwd);
  // import test data
  let file = File::open("src/tests/temporal_tests_1.json")?;
  let reader = BufReader::new(file);
  let temporal_tests: Vec<TestCase> = serde_json::from_reader(reader)?;

  Ok(temporal_tests)
}

fn create_parts(link_parser: &LinkParser, sentence: &str) -> Result<SentenceParts> {
  let sentence_text = SentenceText::new(sentence.to_owned(), SentenceEncodings::None, false)?;
  let mut parts = SentenceParts::from_text(&sentence_text)?; //.map_err(SemaAPiError::from)?;
  let links = link_parser
    .parse_sentence(&parts.corrected_sentence)?
    .unwrap();
  parts.links = links;
  Ok(parts)
}

async fn run_json_test_cases(
  link_parser: &LinkParser,
  test_cases: Vec<TestCase>,
  results: &mut Vec<TestResult>,
) -> Result<()> {
  for test in test_cases.into_iter() {
    let parts = create_parts(link_parser, &test.sentence)?;

    let sema_sentences = process_parts(vec![parts]).await?;
    let sema_sentence = sema_sentences
      .get(0)
      .unwrap();

    panic::set_hook(Box::new(|_info| {
      // do nothing
    }));

    let result = panic::catch_unwind(|| -> Result<()> {
      assert_json_diff::assert_json_eq!(
        &serde_json::to_string(sema_sentence)?,
        &serde_json::to_string(&test.data)?
      );

      Ok(())
    });

    let test_result = match result {
      Ok(_) => TestResult {
        status: TestResultStatus::Success,
        sentence: test.sentence,
        expected: test.data,
        actual: sema_sentence.clone(),
      },
      Err(cause) => {
        // println!("{:?}", e);
        dbg!(&cause);
        TestResult {
          status: TestResultStatus::Failure,
          sentence: test.sentence,
          expected: test.data,
          actual: sema_sentence.clone(),
        }
      }
    };

    results.push(test_result);
  }

  Ok(())
}
