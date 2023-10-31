use async_graphql::{
    http::{graphiql_source, playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use axum::{
    http::Method,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router, Server,
};

use schemas::graphql_schema::{MutationRoot, QueryRoot};
use tower_http::cors::{Any, CorsLayer};

mod schemas;
mod sql_queries;

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let schema = Schema::new(QueryRoot, MutationRoot {}, EmptySubscription);
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        // allow requests from any origin
        .allow_origin(Any);

    log::info!("Starting on Port: http://0.0.0.0:3000/");

    let app = Router::new()
        .route("/graphiql", get(graphiql))
        .route("/playground", get(playground))
        .route("/", post(graphql_handler))
        .layer(Extension(schema))
        .layer(cors);

    Server::bind(&format!("0.0.0.0:3000").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn graphiql() -> impl IntoResponse {
    // Html(playground_source(GraphQLPlaygroundConfig::new("/")))
    Html(graphiql_source("/graphiql", None))
}

async fn graphql_handler(
    Extension(schema): Extension<Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner();

    schema.execute(req).await.into()
}
