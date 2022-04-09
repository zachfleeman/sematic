use anyhow::Result;

use crate::services::allennlp_service::{get_semantic_role_labels, SRLArgs};

pub async fn play_with_srl(payload: &str) -> Result<()> {
  let srl_resp = get_semantic_role_labels(&payload)
    // .map_err(SemaAPiError::from)
    .await?;
  let srl_frames = srl_resp.frames()?;
  // dbg!(&srl_frames);

  let mut agents = vec![];

  // let mut patients = vec![];
  let frame_texts = srl_frames
    .iter()
    .map(|f| f.frame_text())
    .collect::<Vec<_>>();
  dbg!(&frame_texts);

  for frames in srl_frames.into_iter() {
    // println!("verb: {}", frames.verb);
    for (idx, frame) in frames
      .frames
      .clone()
      .into_iter()
      .enumerate()
    {
      println!("idx {}", idx);
      // println!("ft: {}", frame_texts[idx - 1]);

      match frame.arg {
        // SRLArgs::V => todo!(),
        // Agents (e.g. "John")
        SRLArgs::ARG0 => agents.push(
          frame
            .words
            .join(" "),
        ),
        // Patients
        SRLArgs::ARG1 => {
          let text = frame.text();
          let text_frags = frames
            .frames
            .clone()
            .iter()
            .enumerate()
            .map(|(fidx, f)| {
              if idx != fidx {
                f.text()
              } else {
                "".to_string()
              }
            })
            // .map(|(fidx, f)| if idx != fidx { f.text() } else { "".to_string() })
            .filter(|t| !t.is_empty())
            .collect::<Vec<_>>();

          dbg!(&text);
          dbg!(text_frags);
        }
        _ => {}
        // SRLArgs::ARG2 => todo!(),
        // SRLArgs::ARG3 => todo!(),
        // SRLArgs::ARG4 => todo!(),
        // SRLArgs::ARGM(_) => todo!(),
        // SRLArgs::R(_) => todo!(),
        // SRLArgs::Unknown => todo!(),
      }
      // dbg!(&frame);
    }
  }

  dbg!(&agents);

  Ok(())
}
