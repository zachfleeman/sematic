use anyhow::Result;
use futures::stream::StreamExt;
use serde_derive::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::{time::Duration};
use uuid::Uuid;

use mongodb::{
  bson::{self, doc, Document},
  options::ClientOptions,
  Client,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoVerbFrame {
  id: String,

  members: Option<Vec<String>>,

  subclasses_members: Option<Vec<String>>,
}

async fn get_verb_frames() -> Result<HashMap<String, Vec<String>>> {
  let client_options = ClientOptions::parse("mongodb://root:pass12345@localhost:27017").await?;
  let mdb_client = Client::with_options(client_options)?; 
  let db = mdb_client.database("verbnet");
  let collection = db.collection::<Document>("verbnet3.3");

  let pipeline = vec![
    doc! {
      "$project": {
        "id": "$id",
        "subclasses_members": "$subclasses.members.name",
        "members": "$members.name"
      }
    },
    doc! {
      "$unwind": {
        "path": "$subclasses_members",
        "preserveNullAndEmptyArrays": true
      }
    },
    doc! {
      "$sort": {
        "_id": 1i32
      }
    },
  ];

  let mut cursor = collection
    .aggregate(pipeline, None)
    .await?;

  let mut verb_frames = HashMap::new();

  while let Some(result) = cursor.next().await {
    let resp: MongoVerbFrame = bson::from_document(result?)?;

    let members = resp
      .members
      .unwrap_or(vec![]);

    let subclasses_members = resp
      .subclasses_members
      .unwrap_or(vec![]);

    let all_members = members
      .into_iter()
      .chain(subclasses_members.into_iter())
      .collect::<Vec<_>>();

    verb_frames.insert(resp.id, all_members);
  }

  Ok(verb_frames)
}

// pub async fn insert_verb_frames(verb_frames: HashMap<String, Vec<String>>) -> Result<()> {
//   let pool = PgPoolOptions::new()
//     .max_connections(5)
//     .connect_timeout(Duration::new(3, 0))
//     .connect(&"postgres://brochstilley:Hellothere12345!@localhost:5432/sema")
//     .await?;
//   let master_uuid = Uuid::parse_str("11111111-2222-3333-4444-555555555555").unwrap();

//   for verb_frame in verb_frames.into_iter() {
//     let frame = verb_frame
//       .0
//       .split("-")
//       .nth(0)
//       .unwrap();
//     let members = verb_frame.1;

//     sqlx::query!(
//       r#"
//         INSERT INTO verb_frame (frame, members, created_by, updated_by)
//         VALUES ($1, $2, $3, $4)
//         RETURNING *
//       "#,
//       frame,
//       &members,
//       master_uuid,
//       master_uuid,
//     )
//     .fetch_one(&pool)
//     .await
//     .map_err(anyhow::Error::from)?;
//   }

//   Ok(())
// }

#[tokio::main]
pub async fn main() -> Result<()> {
  println!("{}", "running verbframe migration");

  // let verb_frames = get_verb_frames().await?;

  // insert_verb_frames(verb_frames).await?;

  Ok(())
}
