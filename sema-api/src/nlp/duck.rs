use crate::services::duckling::{DucklingParseResponse, DucklingValueOption, NoTypeValue};
use std::ops::Range;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Duck {
  pub parts: Vec<DuckPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuckPart {
  pub content: String,

  pub chars: Range<usize>,

  pub dim: String, // replace with enum

  pub latent: bool,

  pub value: DuckValues,

  pub values: Vec<DuckValues>,
}

impl From<DucklingParseResponse> for DuckPart {
  fn from(duckling_parse_resp: DucklingParseResponse) -> Self {
    let mut part_values: Vec<DuckValues> = vec![];

    let value = match duckling_parse_resp.value {
      DucklingValueOption::Value {
        grain,
        value,
        values,
      } => {
        if let Some(v) = values {
          v.into_iter()
            .for_each(|v| part_values.push(v.into()));
        };

        DuckValues::Value { grain, value }
      }
      DucklingValueOption::Interval { to, from, values } => {
        if let Some(v) = values {
          v.into_iter()
            .for_each(|v| part_values.push(v.into()));
        };
        DuckValues::Interval {
          to: to.into(),
          from: from.into(),
        }
      }
    };

    DuckPart {
      content: duckling_parse_resp.body,
      chars: duckling_parse_resp.start..duckling_parse_resp.end,
      dim: duckling_parse_resp.dim,
      latent: duckling_parse_resp.latent,
      value,
      values: part_values,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuckValue {
  pub value: String,
  pub grain: String,
}

impl From<NoTypeValue> for DuckValue {
  fn from(no_type_value: NoTypeValue) -> Self {
    DuckValue {
      value: no_type_value.value,
      grain: no_type_value.grain,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DuckValues {
  Value { grain: String, value: String },
  Interval { to: DuckValue, from: DuckValue },
}

impl From<DucklingValueOption> for DuckValues {
  fn from(duckling_val_opt: DucklingValueOption) -> Self {
    match duckling_val_opt {
      DucklingValueOption::Value {
        grain,
        value,
        values: _,
      } => DuckValues::Value { grain, value },
      DucklingValueOption::Interval {
        to,
        from,
        values: _,
      } => DuckValues::Interval {
        to: to.into(),
        from: from.into(),
      },
    }
  }
}

impl From<Vec<DucklingParseResponse>> for Duck {
  fn from(duckling_parse_responses: Vec<DucklingParseResponse>) -> Self {
    let mut parts = Vec::new();
    for duckling_parse_response in duckling_parse_responses {
      let duck_part = duckling_parse_response.into();

      parts.push(duck_part);
    }
    Duck { parts }
  }
}
