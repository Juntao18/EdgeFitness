use axum::{extract::State, Json};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::models::user::{Address, User};

use super::route_builder::AppState;

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct SignupRequst {
    pub username: String,
    pub password: String,
    pub phone: String,
    pub utype: String,
    pub address: Address,
}

pub struct Auth {}

impl Auth {
    pub async fn create_user(
        State(state): State<AppState>,
        Json(payload): Json<SignupRequst>,
    ) -> String {
        let new_user = User::new_with_signup(payload);
        //returns string
        println!("Creating user :: {:#?}", new_user);

        let result = state.database.users.insert_one(new_user).await; // get the users collection from the database

        format!("{:?}", result)
    }

    pub async fn login_user(
        State(state): State<AppState>,
        Json(payload): Json<LoginRequest>,
    ) -> String {
        println!("{:?}", payload);
        let user_req = state
            .database
            .users
            .find_one(doc! {"username": &payload.username})
            .await;

        println!("{:#?}", user_req);
        let response = {
            match user_req {
                Ok(potential_user) => match potential_user {
                    Some(user) => {
                        if user.password == payload.password {
                            "0".to_string()
                        } else {
                            "2".to_string()
                        }
                    }
                    None => "1".to_string(),
                },
                Err(_) => "5".to_string(),
            }
        };
        println!("{}", response);
        response
    }
}
