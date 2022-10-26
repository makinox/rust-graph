mod config;
mod handler;
mod schemas;

use actix_web::{
    guard,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use config::mongo::DBMongo;
use dotenv::dotenv;
use handler::graphql_handler::{Mutation, ProjectSchema, Query};
use std::env;

//graphql entry
async fn index(schema: Data<ProjectSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let current_env = match env::var("ENV") {
        Ok(v) => v.to_string(),
        Err(_) => "DEV".to_string(),
    };
    let mut address = "127.0.0.1";
    if current_env == "PROD" {
        address = "0.0.0.0";
    }
    //connect to the data source
    let db = DBMongo::init().await;
    let schema_data = Schema::build(Query, Mutation, EmptySubscription)
        .data(db)
        .finish();
    println!("Running in - http://localhost:8080");
    println!("Running in - {}:8080", address);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema_data.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind((address, 8080))?
    .run()
    .await
}
