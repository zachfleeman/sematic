use strum_macros::{self, Display};

/*
examples:
- still
- in 1935
- "when I overwashed"
- "before turning the thrift over to federal regulators"


temporal
  - relative


Questions:
- Should "tense" be a temporal type?
- How should I handle "before" and "after"?
  -
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "temporal_type")]
pub enum Temporals {
  Tense(Tense), // Not sure if it's best to have a temporal action, as well as an event property of tense.
  Relative(Relative),
  Absolute(Absolute),
  General(General),
  Duration(Duration),
  Interval(Interval),
}

impl Temporals {
  pub fn get_symbol(&self) -> String {
    match self {
      Temporals::Tense(t) => t.symbol.to_owned(),
      Temporals::Relative(r) => r.symbol.to_owned(),
      Temporals::Absolute(a) => a.symbol.to_owned(),
      Temporals::General(g) => g.symbol.to_owned(),
      Temporals::Duration(d) => d.symbol.to_owned(),
      Temporals::Interval(i) => i.symbol.to_owned(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tense {
  pub symbol: String,
  pub tense: Tenses,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum Tenses {
  Past,
  Present,
  Future,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relative {
  pub symbol: String,
  pub properties: Vec<RelativeProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
// #[serde(tag = "relative_type")]
pub enum RelativeProperties {
  Before { before: String }, // Can be a temporal or an event symbol.
  After { after: String },
  During { during: String },
  When { when: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Absolute {
  pub symbol: String,
  pub properties: Vec<AbsoluteProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
// #[serde(tag = "absolute_property_type")]
pub enum AbsoluteProperties {
  Year { year: i32 },
  Month { month: i32 },
  Week { week: i32 },
  Day { day: i32 },
  DayOfWeek { day_of_week: DaysOfWeek },
  Hour { hour: i32 },
  Minute { minute: i32 },
  Second { second: i32 },
  Epoch { epoch: i32 }, // unix epoch
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum DaysOfWeek {
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
  Sunday,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct General {
  pub symbol: String,
  pub general: GeneralTemporal,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum GeneralTemporal {
  Past,
  Present,
  Future,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duration {
  pub symbol: String,
  pub properties: Vec<DurationTemporal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
// #[serde(tag = "temporal_duration_type")]
pub enum DurationTemporal {
  Year { year: i32 },
  Month { month: i32 },
  Week { week: i32 },
  Day { day: i32 },
  Hour { hour: i32 },
  Minute { minute: i32 },
  Second { second: i32 },
  Epoch { epoch: i32 },    // unix epoch
  Start { start: String }, // to another temporal?
  End { end: String },     // to another temporal?
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interval {
  pub symbol: String,
  pub properties: Vec<IntervalProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
// #[serde(tag = "temporal_interval_type")]
#[serde(untagged)]
pub enum IntervalProperties {
  Years { years: i32 },
  Months { months: i32 },
  Weeks { weeks: i32 },
  Days { days: i32 },
  Hours { hours: i32 },
  Minutes { minutes: i32 },
  Seconds { seconds: i32 },
}