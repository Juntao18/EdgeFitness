use std::str::FromStr;

use axum::extract::{Path, State};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::services::{database::DatabaseState, route_builder::AppState};

use super::review::Review;

#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    pub _id: ObjectId,
    pub name: String,
    pub location: String,
    pub owner_id: ObjectId,
    pub reviews: Vec<Review>,
    pub phone_number: String,
    pub opening_hours: String,
    pub closing_hours: String,
}

impl Provider {
    pub fn new(
        _id: ObjectId,
        name: String,
        location: String,
        owner_id: ObjectId,
        reviews: Vec<Review>,
        phone_number: String,
        opening_hours: String,
        closing_hours: String,
    ) -> Self {
        Self {
            _id,
            name,
            location,
            owner_id,
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
        println!("Filtering providers by id");
        let parsed_id = ObjectId::from_str(&object_id);

        match parsed_id {
            Ok(id) => {
                let query_result = state
                    .database
                    .providers
                    .find(doc! {"_id": id})
                    .await
                    .unwrap();
                return DatabaseState::serialize_document_with_cursor(query_result)
                    .await
                    .unwrap();
            }
            Err(e) => return format!("Invalid ObjectID :: {}", e),
        }
    }

    pub async fn get_all_providers(State(state): State<AppState>) -> String {
        println!("Fetching all providers");
        let query_result = state.database.providers.find(doc! {}).await.unwrap();
        return DatabaseState::serialize_document_with_cursor(query_result)
            .await
            .unwrap();
    }

    pub async fn _filter_with_tags(
        Path(tags): Path<String>,
        State(state): State<AppState>,
    ) -> String {
        println!("Filtering providers by tags");

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

        let query = doc! {"$and": regex_queries};
        let cur = state.database.providers.find(query).await.unwrap();

        return DatabaseState::serialize_document_with_cursor(cur)
            .await
            .unwrap();
    }
}
