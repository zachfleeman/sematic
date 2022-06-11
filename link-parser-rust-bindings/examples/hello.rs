use anyhow::Result;
use link_parser_rust_bindings::{LinkParser, LinkParserOptions};

fn main() -> Result<()> {
  let link_parser = LinkParser::new(LinkParserOptions::default());

  let sentence = link_parser.parse_sentence("the needle was painful")?;
  // let sentence = link_parser.parse_sentence("create a new folder".to_string())?;

  dbg!(sentence);

  Ok(())
}
