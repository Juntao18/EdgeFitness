use std::str::FromStr;

use axum::{extract::{Path, State}, Json};
use mongodb::bson::{doc, oid::ObjectId };
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::
    services::{auth::SignupRequst, database::DatabaseState, route_builder::AppState}
;

use super::review::Review;
#[derive(Debug, Serialize, Deserialize)]
struct ResetResponse {
    success: bool,
    message: String,
} 

#[derive(Debug, Serialize, Deserialize)]

pub enum UserType {
    Regular,
    GymOwner,
    FoodProvider,
    PersonalTrainer,
    Admin,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum UserOptions {
    None,
    StringValue(String),
    StringVector(Vec<String>, Vec<Review>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    address_line_1: String,
    address_line_2: String,
    city_county: String,
    eircode: String,
}

impl Address {
    pub fn new(
        address_line_1: String,
        address_line_2: String,
        city_county: String,
        eircode: String,
    ) -> Self {
        Self {
            address_line_1,
            address_line_2,
            city_county,
            eircode,
        }
    }
}



#[derive(Debug, Serialize, Deserialize)]

pub struct User {
    _id: ObjectId,
    username: String,
    pub password: String, 
    phone: String,
    utype: UserType,
    address: Address,
    options: UserOptions,
}

impl User {

    pub fn new_with_signup(signup: SignupRequst) -> Self{
        let asd  =Self { 
            _id: ObjectId::new(), 
            username: signup.username, 
            password: signup.password, 
            phone: signup.phone, 
            utype: match signup.utype.as_str() {
                "GymOwner" => UserType::GymOwner,
                "Admin" => UserType::Admin,
                "FoodProvider" => UserType::FoodProvider,
                "PersonalTrainer" => UserType::PersonalTrainer,
            _ => UserType::Regular
            
        }, address: signup.address, options: UserOptions::None };
        println!("{:?}", asd);
        asd

    }
    pub fn new_with_options(
        _id: ObjectId,
        username: String,
        password: String, 
        phone: String,
        utype: UserType,
        address: Address,
        options: UserOptions,
    ) -> Self {
        Self {
            _id,
            username,
            password, 
            phone,
            utype,
            address,
            options,
        }
    }

    pub fn new(
        _id: ObjectId,
        username: String,
        password: String, 
        phone: String,
        utype: UserType,
        address: Address,
    ) -> Self {
        Self {
            _id,
            username,
            password, 
            phone,
            utype,
            address,
            options: UserOptions::None,
        }
    }

    pub async fn reset_password(
        State(state): State<AppState>,
        Json(payload): Json<serde_json::Value>,
    ) -> String {
        let username_option = payload.get("username").and_then(serde_json::Value::as_str);
        let new_pw_option = payload
            .get("newPassword")
            .and_then(serde_json::Value::as_str); 

        match (username_option, new_pw_option) {
            (None, None) => to_string(&ResetResponse {success: false, 
                message: "Incorrectly formatted username and password (Neither is can be parsed as a string)".to_string()}).unwrap(),
            (None, Some(_)) => to_string(&ResetResponse {
                success: false, 
                message: "Incorrectly formatted username (It cannot be parsed as a string)".to_string()}).unwrap(),
            (Some(_), None) => to_string(&ResetResponse {
                success: false, 
                message: "Incorrectly formatted password (It cannot be parsed as a string)".to_string()}).unwrap(),
            (Some(username), Some(password)) => {

                println!("{} {}", username, password);


                let _result = state.database.users.update_one(doc!{"username": username}, doc!{"$set": doc! {"password": password}}).await;

                match _result {
                    Ok(msg) =>{
                        if msg.matched_count == 0 {
                           return to_string(&ResetResponse { success: false, message: format!("Couldn't find user to update. Does this username exist?")}).unwrap()
                        } 
                        to_string(&ResetResponse { success: true, message: format!("Password has been updated successfully {:?}", msg)}).unwrap()},
                    Err(e) => to_string(&ResetResponse {success: false, message: format!("{:?}", e)}).unwrap(),
                } 
            }
        } 
    }

    pub async fn get_all_trainers(State(state): State<AppState>) -> String { 
        let query_result = state
            .database
            .users
            .find(doc! { "utype": "PersonalTrainer"})
            .await
            .unwrap();
        DatabaseState::serialize_document_with_cursor(query_result)
            .await
            .unwrap()
    }

    pub async fn get_all_users(State(state): State<AppState>) -> String {
        println!("Fetching all users");
        let query_result = state.database.users.find(doc! {}).await.unwrap();
        return DatabaseState::serialize_document_with_cursor(query_result)
            .await
            .unwrap();
    }

    pub async fn filter_by_id(
        Path(object_id): Path<String>,
        State(state): State<AppState>,
    ) -> String {
        println!("Filtering users by id");
        let parsed_id = ObjectId::from_str(&object_id);

        match parsed_id {
            Ok(id) => {
                let query_result = state.database.users.find(doc! {"_id": id}).await.unwrap();
                return DatabaseState::serialize_document_with_cursor(query_result)
                    .await
                    .unwrap();
            }
            Err(e) => return format!("Invalid ObjectID :: {}", e),
        }
    }
}
