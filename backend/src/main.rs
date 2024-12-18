mod models;
mod services;

use axum::Router;
use services::route_builder::RouteBuilder;

#[tokio::main]
async fn main() {
    let app: Router = RouteBuilder::build_route().await;
    println!("Made routes");

    let tcp_addr = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();

    println!("Hosting at: {:?}", tcp_addr.local_addr());
    let _result = axum::serve(tcp_addr, app.into_make_service()).await;
}
