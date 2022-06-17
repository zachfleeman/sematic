#[macro_use]
extern crate serde_derive;
extern crate derive_more;
extern crate serde_json;

pub mod config;
pub mod nlp;
pub mod parse;
pub mod process_sentences;
pub mod routes;
pub mod sema;
pub mod sentence;
pub mod services;
pub mod state;
pub mod wordnet;
pub mod verify;
pub mod middleware;