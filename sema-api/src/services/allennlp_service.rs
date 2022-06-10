use anyhow::Result;
use strum_macros::{EnumString};
use std::{str::FromStr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRLResponse {
  pub verbs: Vec<SRLVerb>,

  pub words: Vec<String>,
}

impl SRLResponse {
  pub fn frames(&self) -> Result<Vec<SRLFrames>> {
    let frames = self
      .verbs
      .clone()
      .iter()
      .map(|verb| -> SRLFrames { SRLFrames::from_verb(verb, &self.words) })
      .collect::<Vec<SRLFrames>>();

    Ok(frames)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRLVerb {
  pub description: String,
  pub verb: String,
  pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SRLArgs {
  V,
  ARG0,
  ARG1,
  ARG2,
  ARG3,
  ARG4,
  ARGM(SRLArgMModifiers),
  R(Box<SRLArgs>),
  Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, EnumString)]
pub enum SRLArgMModifiers {
  COM, // comitative
  LOC, // locative
  DIR, // directional
  GOL, // goal
  MNR, // manner
  TMP, // temporal
  EXT, // extent
  REC, // reciprocals
  PRD, // secondary predication
  PRP, // purpose
  CAU, // cause
  DIS, // discourse
  ADV, // adverb
  ADJ, // adjectival
  MOD, // modal
  NEG, // negation
  DSP, // direct speech
  LVB, // light verb
  CXN, // construction
  Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRLFrames {
  pub verb: String,

  pub frames: Vec<SRLFrame>,
}

impl SRLFrames {
  pub fn from_verb(srl_verb: &SRLVerb, words: &Vec<String>) -> Self {
    let mut frames = Vec::new();

    let mut current_frame: Option<SRLFrame> = None;

    for (idx, tag) in srl_verb.tags.iter().enumerate() {
      //Break down tag
      let tag_parts = tag.split("-").collect::<Vec<&str>>();

      let tag_type = tag_parts[0];

      match tag_type {
        "B" => {
          // Save previous frame. Should only happen if two "B" tags are next to each other.
          if let Some(cf) = current_frame.clone() {
            frames.push(cf);
          }

          let arg = match tag_parts[1] {
            "V" => SRLArgs::V,
            "ARG0" => SRLArgs::ARG0,
            "ARG1" => SRLArgs::ARG1,
            "ARG2" => SRLArgs::ARG2,
            "ARG3" => SRLArgs::ARG3,
            "ARG4" => SRLArgs::ARG4,
            "ARGM" => {
              let argm_mod = SRLArgMModifiers::from_str(tag_parts[2]).unwrap();

              SRLArgs::ARGM(argm_mod)
            },
            // "R" => {},
            _ => SRLArgs::Unknown,
          };

          current_frame = Some(SRLFrame { arg, words: vec![words[idx].clone()] })
        },
        "I" => {
          if let Some(cf) = current_frame.as_mut() {
            cf.words.push(words[idx].clone());
          }
        },
        "O" => {},
        _ => {},
      }

      if idx == srl_verb.tags.len() - 1 {
        if let Some(cf) = current_frame.clone() {
          frames.push(cf);
        }
      }
    }

    SRLFrames {
      verb: srl_verb.verb.clone(),
      frames,
    }
  }

  pub fn frame_text(&self) -> String {
    self.frames.iter().map(|f| f.words.join(" ")).collect::<Vec<String>>().join(" ")
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRLFrame {
  pub arg: SRLArgs,
  pub words: Vec<String>,
}

impl SRLFrame {
  pub fn text(&self) -> String {
    self.words.join(" ")
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRLBody<'a> {
  pub sentence: &'a str,
}

pub async fn get_semantic_role_labels(sentence: &str) -> Result<SRLResponse> {
  let client = reqwest::Client::new();
  let body = SRLBody { sentence };

  let resp = client
    .post("https://srl.allennlp.sematic.rocks/predict")
    .json::<SRLBody>(&body)
    .send()
    // .map_err(|e| e.into())
    .await?;

  let srl_resp = resp
    .json::<SRLResponse>()
    // .map_err(|e| e.into())
    .await?;

  Ok(srl_resp)
}
