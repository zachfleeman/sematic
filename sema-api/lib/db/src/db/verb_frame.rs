use uuid::Uuid;
use serde_derive::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::{PgExecutor};
use anyhow::Result;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DBVerbFrame {
  pub id: Uuid,

  pub frame: String,

  pub members: Vec<String>,

  pub created_at: DateTime<Utc>,

  pub created_by: Uuid,

  pub updated_at: DateTime<Utc>,

  pub updated_by: Uuid
}

// TEMP: hack to get the project compiling. Doesn't really use a DB yet.
pub async fn get_verb_frames_by_member(
  _pool: impl PgExecutor<'_>,
  _member: &str
) -> Result<Vec<DBVerbFrame>> {
  // let verb_frames = sqlx::query_as!(
  //   DBVerbFrame,
  //   r#"
  //     SELECT
  //       id,
  //       frame,
  //       members,
  //       created_at,
  //       created_by,
  //       updated_at,
  //       updated_by
  //     FROM
  //       verb_frame
  //     WHERE
  //       members @> ARRAY[$1]
  //   "#,
  //   member
  // )
  // .fetch_all(pool)
  // .await?;

  // Ok(verb_frames)
  Ok(vec![])
}