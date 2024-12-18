use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::models::{gym::Gym, provider::Provider, user::User, workout::Workout};

use super::{auth::Auth, database::DatabaseState};

pub struct RouteBuilder {}

#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseState,
}

impl RouteBuilder {
    pub async fn build_route() -> Router {
        let state = AppState {
            database: DatabaseState::new().await,
        };

        Router::new()
            .nest_service("/", host_frontend())
            .route(
                "/insert-sample-data",
                get(DatabaseState::upload_sample_data),
            )
            .route("/purge-database", get(DatabaseState::purge_database))
            .nest(
                "/api/auth",
                Router::new()
                    .route("/login", post(Auth::login_user))
                    .route("/signup", post(Auth::create_user)),
            )
            .nest(
                "/api/gym",
                Router::new()
                    .route("/search/:object_id", get(Gym::filter_by_id))
                    .route("/all", get(Gym::get_all_gyms)),
            )
            .nest(
                "/api/workout",
                Router::new()
                    .route("/all", get(Workout::get_all_workouts))
                    .route("/search/:objectID", get(Workout::filter_by_id))
                    .route("/search/tags/*tags", get(Workout::filter_with_tags))
                    .route("/upload", post(Workout::upload_workout)),
            )
            .nest(
                "/api/user",
                Router::new()
                    .route("/all", get(User::get_all_users))
                    .route("/search/:objectID", get(User::filter_by_id))
                    .route("/trainer/all", get(User::get_all_trainers))
                    .route("/reset-password", post(User::reset_password)),
            )
            .nest(
                "/api/provider",
                Router::new()
                    .route("/all", get(Provider::get_all_providers))
                    .route("/search/:objectID", get(Provider::filter_by_id)),
            )
            .with_state(state)
    }
}

fn host_frontend() -> ServeDir {
    let serve = ServeDir::new("../frontend/");
    println!("{:?}", serve);
    serve
}
