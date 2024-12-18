use axum::extract::State;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};
use serde::Serialize;
use serde_json::to_string;

use crate::models::{
    exercise::Exercise,
    gym::Gym,
    provider::Provider,
    review::Review,
    user::{Address, User, UserOptions, UserType},
    workout::Workout,
};

use super::route_builder::AppState;
#[derive(Debug, Clone)]

pub struct DatabaseState {
    pub users: Collection<User>,
    pub gyms: Collection<Gym>,
    pub workouts: Collection<Workout>,
    pub providers: Collection<Provider>,
}

impl DatabaseState {
    pub async fn new() -> Self {
        let uri = "mongodb+srv://Bence:Bence.123@edgefitness.2dqj0.mongodb.net/?retryWrites=true&w=majority&appName=EdgeFitness7";
        let client = Client::with_uri_str(uri)
            .await
            .expect("Cannot connect to database");
        println!("Connected to MongoDB database");
        let db = client.database("EdgeFitness");

        let state = DatabaseState {
            users: db.collection("users"),
            gyms: db.collection("gyms"),
            workouts: db.collection("workouts"),
            providers: db.collection("providers"),
        };

        println!("Selected collections");
        state
    }

    pub async fn purge_database(State(state): State<AppState>) -> String {
        let workouts = state.database.workouts.delete_many(doc! {}).await;
        let gyms = state.database.gyms.delete_many(doc! {}).await;
        let users = state.database.users.delete_many(doc! {}).await;
        let providers = state.database.providers.delete_many(doc! {}).await;

        format!("{:?} {:?} {:?} {:?}", workouts, gyms, users, providers)
    }

    pub async fn upload_sample_data(State(state): State<AppState>) -> String {
        let user1 = User::new(
            ObjectId::new(),
            "Johnny".to_string(),
            "Password".to_string(),
            "01236352532".to_string(),
            UserType::Regular,
            Address::new(
                "addy1".to_string(),
                "address_line_2".to_string(),
                "Dublin".to_string(),
                "K7863AA".to_string(),
            ),
        );

        let user2 = User::new(
            ObjectId::new(),
            "Alice".to_string(),
            "SecurePass123".to_string(),
            "01587459632".to_string(),
            UserType::Admin,
            Address::new(
                "addy2".to_string(),
                "apartment 5B".to_string(),
                "Galway".to_string(),
                "G4578LM".to_string(),
            ),
        );

        let user3 = User::new(
            ObjectId::new(),
            "Carlos".to_string(),
            "SuperSecret".to_string(),
            "01658472311".to_string(),
            UserType::GymOwner,
            Address::new(
                "addy3".to_string(),
                "suite 402".to_string(),
                "Cork".to_string(),
                "C9831OP".to_string(),
            ),
        );

        let user4 = User::new_with_options(
            ObjectId::new(),
            "Maximus Superfit".to_string(),
            "SuperSecret".to_string(),
            "01658472311".to_string(),
            UserType::PersonalTrainer,
            Address::new(
                "addy3".to_string(),
                "suite 402".to_string(),
                "Cork".to_string(),
                "C9831OP".to_string(),
            ),
            UserOptions::StringVector(
                vec![
                    "Max Fitness".to_string(),
                    "Toning".to_string(),
                    "Lets get that summer bod!".to_string(),
                ],
                vec![
                    Review::new(5, "Very Friendly".to_string()),
                    Review::new(2, "Made fun of me".to_string()),
                ],
            ),
        );

        let user5 = User::new_with_options(
            ObjectId::new(),
            "Jenny".to_string(),
            "SuperSecret".to_string(),
            "01658472311".to_string(),
            UserType::PersonalTrainer,
            Address::new(
                "addy3".to_string(),
                "suite 402".to_string(),
                "Cork".to_string(),
                "C9831OP".to_string(),
            ),
            UserOptions::StringVector(
                vec![
                    "Jenny (Bicep) Jones".to_string(),
                    "Biceps".to_string(),
                    "I LOVE BICEP CURLS".to_string(),
                ],
                vec![
                    Review::new(5, "Great biceps".to_string()),
                    Review::new(4, "Good bicep routine".to_string()),
                ],
            ),
        );

        let user6 = User::new_with_options(
            ObjectId::new(),
            "Josh".to_string(),
            "SuperSecret".to_string(),
            "01658472311".to_string(),
            UserType::PersonalTrainer,
            Address::new(
                "addy3".to_string(),
                "suite 402".to_string(),
                "Cork".to_string(),
                "C9831OP".to_string(),
            ),
            UserOptions::StringVector(
                vec![
                    "Josh Jones".to_string(),
                    "Bulk up".to_string(),
                    "Ready to step up your game?\nLets get it started".to_string(),
                ],
                vec![
                    Review::new(5, "Bulk Bulk".to_string()),
                    Review::new(4, "Super Bulk".to_string()),
                ],
            ),
        );

        let user_vec = vec![user1, user2, user3, user4, user5, user6];

        let gym1 = Gym {
            name: "PowerHouse Fitness".to_string(),
            location: "123 Main St, Dublin".to_string(),
            owner: ObjectId::new(),
            reviews: vec![
                Review::new(5, "Great equipment and friendly staff.".to_string()),
                Review::new(4, "Nice place, but can get crowded.".to_string()),
            ],
            phone_number: "0123456789".to_string(),
            opening_hours: "06:00".to_string(),
            closing_hours: "22:00".to_string(),
        };

        let gym2 = Gym {
            name: "Urban Strength".to_string(),
            location: "456 King St, Galway".to_string(),
            owner: ObjectId::new(),
            reviews: vec![
                Review::new(3, "Good gym, but limited parking.".to_string()),
                Review::new(4, "Affordable membership options.".to_string()),
                Review::new(5, "Love the personal trainers here!".to_string()),
            ],
            phone_number: "0987654321".to_string(),
            opening_hours: "05:30".to_string(),
            closing_hours: "21:00".to_string(),
        };

        let gym3 = Gym {
            name: "FitHub".to_string(),
            location: "789 Queen St, Cork".to_string(),
            owner: ObjectId::new(),
            reviews: vec![
                Review::new(4, "Clean and well-maintained facilities.".to_string()),
                Review::new(2, "Limited equipment in peak hours.".to_string()),
            ],
            phone_number: "0112233445".to_string(),
            opening_hours: "07:00".to_string(),
            closing_hours: "23:00".to_string(),
        };

        let gym_vec = vec![gym1, gym2, gym3];

        let workout1 = Workout::new(
            ObjectId::new(),
            "Super Day".to_string(),
            vec![
                Review::new(3, "it is okay".to_string()),
                Review::new(5, "Best workout ever".to_string()),
            ],
            "Sure my best one".to_string(),
            vec![Exercise::new(
                "Bicep Curls".to_string(),
                4,
                12,
                "Keep the motion slow".to_string(),
            )],
        );

        let workout2 = Workout::new(
            ObjectId::new(),
            "Flying training".to_string(),
            vec![
                Review::new(1, "WOO".to_string()),
                Review::new(5, "WEEEEEEEEEEEEE".to_string()),
            ],
            "Super man".to_string(),
            vec![Exercise::new(
                "Backflips".to_string(),
                4,
                20,
                "Try stop half way through".to_string(),
            )],
        );

        let workout3 = Workout::new(
            ObjectId::new(),
            "Pizza Making".to_string(),
            vec![
                Review::new(1, "Sandwiches are better".to_string()),
                Review::new(5, "I hurt my head".to_string()),
            ],
            "Extra sauce is the best".to_string(),
            vec![
                Exercise::new(
                    "Spin the dough".to_string(),
                    4,
                    20,
                    "Try stop half way through".to_string(),
                ),
                Exercise::new(
                    "Cook the pizza".to_string(),
                    10,
                    500,
                    "Try not the burn it!!".to_string(),
                ),
            ],
        );

        let workout_vector = vec![workout1, workout2, workout3];

        let provider1 = Provider::new(
            ObjectId::new(),
            "Health Shop".to_string(),
            "California".to_string(),
            ObjectId::new(),
            vec![
                Review::new(1, "Sandwiches are better".to_string()),
                Review::new(5, "I hurt my head".to_string()),
            ],
            "123456789".to_string(),
            "2:00".to_string(),
            "18:00".to_string(),
        );

        let provider2 = Provider::new(
            ObjectId::new(),
            "Junk Food".to_string(),
            "Texas".to_string(),
            ObjectId::new(),
            vec![
                Review::new(1, "Sandwiches are better".to_string()),
                Review::new(5, "I hurt my head".to_string()),
            ],
            "98764321".to_string(),
            "21:30".to_string(),
            "23:00".to_string(),
        );

        let provider3 = Provider::new(
            ObjectId::new(),
            "Super Shop".to_string(),
            "Dublin".to_string(),
            ObjectId::new(),
            vec![
                Review::new(1, "Sandwiches are better".to_string()),
                Review::new(5, "I hurt my head".to_string()),
            ],
            "54378123".to_string(),
            "5:00".to_string(),
            "21:00".to_string(),
        );

        let provders_vec = vec![provider1, provider2, provider3];

        let _res_providers_insert = state.database.providers.insert_many(provders_vec).await;
        let _res_workout_insert = state.database.workouts.insert_many(workout_vector).await;
        let _res_user_insert = state.database.users.insert_many(user_vec).await;
        let _res_gym_insert = state.database.gyms.insert_many(gym_vec).await;

        format!("Inserted sample data")
    }

    pub async fn serialize_document_with_cursor<T>(
        res: mongodb::Cursor<T>,
    ) -> Result<String, String>
    where
        T: serde::de::DeserializeOwned + Serialize,
    {
        let t_vector: Vec<T> = res.try_collect().await.unwrap();
        Ok(to_string(&t_vector).unwrap())
    }
}
