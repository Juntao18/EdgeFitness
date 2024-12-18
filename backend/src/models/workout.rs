use std::str::FromStr;

use axum::extract::{Path, State};
use axum::Json;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::services::database::DatabaseState;
use crate::services::route_builder::AppState;

use super::exercise::Exercise;
use super::review::Review;

#[derive(Debug, Deserialize)]
pub struct NewWorkout {
    name: String,
    comment: String,
    exercises: Vec<Exercise>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct Workout {
    pub owner: ObjectId,
    pub name: String,
    pub reviews: Vec<Review>,
    pub comment: String,
    pub exercises: Vec<Exercise>,
}

impl Workout {
    pub fn new(
        owner: ObjectId,
        name: String,
        reviews: Vec<Review>,
        comment: String,
        exercises: Vec<Exercise>,
    ) -> Self {
        Self {
            owner,
            name,
            reviews,
            comment,
            exercises,
        }
    }

    pub fn new_with_post(new_workout: NewWorkout) -> Self {
        Self {
            owner: ObjectId::new(),
            name: new_workout.name,
            reviews: vec![],
            comment: new_workout.comment,
            exercises: new_workout.exercises,
        }
    }

    pub async fn upload_workout(
        State(state): State<AppState>,
        Json(payload): Json<NewWorkout>,
    ) -> String {
        if payload.name == "".to_string() {
            "Missing workout name".to_string();
        }
        let new_workout = Self::new_with_post(payload);
        println!("Adding workout :: {:#?}", new_workout);

        let result = state.database.workouts.insert_one(new_workout).await;
        println!("{:?}", result);
        format!("{:?}", result)
    }

    pub async fn get_all_workouts(State(state): State<AppState>) -> String {
        println!("Retrieving all workouts");
        let query_result = state.database.workouts.find(doc! {}).await.unwrap();
        return DatabaseState::serialize_document_with_cursor(query_result)
            .await
            .unwrap();
    }

    pub async fn filter_by_id(
        Path(object_id): Path<String>,
        State(state): State<AppState>,
    ) -> String {
        let parsed_id = ObjectId::from_str(&object_id);
        println!("Searching workouts by id");
        match parsed_id {
            Ok(id) => {
                let query_result = state
                    .database
                    .workouts
                    .find(doc! {"owner": id})
                    .await
                    .unwrap();
                return DatabaseState::serialize_document_with_cursor(query_result)
                    .await
                    .unwrap();
            }
            Err(e) => return format!("Invalid ObjectID :: {}", e),
        }
    }

    pub async fn filter_with_tags(
        Path(tags): Path<String>,
        State(state): State<AppState>,
    ) -> String {
        println!("Retrieving workouts by tags");

        let tags_vec: Vec<&str> = tags.split('/').collect();
        let regex_queries: Vec<_> = tags_vec
            .iter()
            .map(|tag| {
                doc! {
                    "tags": {
                        "$regex": format!("^{}$", tag),
                        "$options": "i"
                    }
                }
            })
            .collect();

        let query = doc! {
            "$and": regex_queries
        };
        let cur = state.database.workouts.find(query).await;

        return DatabaseState::serialize_document_with_cursor(cur.unwrap())
            .await
            .unwrap();
    }
}
