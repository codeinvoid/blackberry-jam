use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use graphql::schema::{build_schema, AppSchema};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt};

pub mod graphql;
#[allow(warnings, unused)]
pub mod prisma;

// async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
//     schema.execute(req.into_inner()).await.into()
// }

async fn graphql_handler(State(schema): State<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

#[tokio::main]
async fn main() {
    let schema = build_schema().await;

    let app = Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .with_state(schema)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let file_appender = rolling::hourly("./logs", "log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            .with_env_filter("server")
            .with_max_level(tracing::Level::DEBUG)
            .finish()
            .with(fmt::Layer::default().with_writer(non_blocking)),
    )
    .expect("Unable to set global tracing subscriber");

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
