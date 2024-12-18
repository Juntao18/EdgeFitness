use std::str::FromStr;

use axum::extract::{Path, State};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::services::{database::DatabaseState, route_builder::AppState};

use super::review::Review;

#[derive(Debug, Serialize, Deserialize)]

pub struct Gym {
    pub name: String,
    pub location: String,
    pub owner: ObjectId,
    pub reviews: Vec<Review>,
    pub phone_number: String,
    pub opening_hours: String,
    pub closing_hours: String,
}

impl Gym {
    pub fn _new(
        name: String,
        location: String,
        owner: ObjectId,
        reviews: Vec<Review>,
        phone_number: String,
        opening_hours: String,
        closing_hours: String,
    ) -> Self {
        Self {
            name,
            location,
            owner,
            reviews,
            phone_number,
            opening_hours,
            closing_hours,
        }
    }

    pub async fn filter_by_id(
        Path(object_id): Path<String>,
        State(state): State<AppState>,
    ) -> String {
        println!("Filtering gyms by id");
        let parsed_id = ObjectId::from_str(&object_id);

        match parsed_id {
            Ok(id) => {
                let query_result = state.database.gyms.find(doc! {"_id": id}).await.unwrap();
                return DatabaseState::serialize_document_with_cursor(query_result)
                    .await
                    .unwrap();
            }
            Err(e) => return format!("Invalid ObjectID :: {}", e),
        }
    }

    pub async fn get_all_gyms(State(state): State<AppState>) -> String {
        println!("Fetching all gyms");
        let query_result = state.database.gyms.find(doc! {}).await.unwrap();
        return DatabaseState::serialize_document_with_cursor(query_result)
            .await
            .unwrap();
    }
}
