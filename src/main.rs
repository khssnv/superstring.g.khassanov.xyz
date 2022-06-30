extern crate superstring;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::Path,
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    routing::post,
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use utoipa::{Component, OpenApi};
use utoipa_swagger_ui::Config;

use superstring::{naive, tsp, unique_by_substrings};

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        handlers(
            handler,
        ),
        components(CreateShortestCommonSuperstring, ShortestCommonSuperstring),
        tags(
            (name = "superstring", description = "Shortest Common Superstring")
        )
    )]
    struct ApiDoc;
    let api_doc = ApiDoc::openapi();
    let config = Arc::new(Config::from("/api-doc/openapi.json"));

    let server = async {
        let app = Router::new()
            .route("/shortest-common-superstring", post(handler))
            .route(
                "/swagger-ui/*tail",
                get(serve_swagger_ui).layer(Extension(config)),
            )
            .route(
                "/api-doc/openapi.json",
                get({
                    let doc = api_doc.clone();
                    move || async { Json(doc) }
                }),
            )
            .layer(
                CorsLayer::new()
                    .allow_origin("http://localhost:4000".parse::<HeaderValue>().unwrap())
                    .allow_origin("http://127.0.0.1:4000".parse::<HeaderValue>().unwrap())
                    .allow_origin(
                        "http://superstring.g.khassanov.xyz"
                            .parse::<HeaderValue>()
                            .unwrap(),
                    )
                    .allow_origin(
                        "http://superstring.garage.khassanov.xyz"
                            .parse::<HeaderValue>()
                            .unwrap(),
                    )
                    .allow_methods([Method::POST]),
            );
        serve(app, 4000).await;
    };

    tokio::join!(server);
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn serve_swagger_ui(
    Path(tail): Path<String>,
    Extension(state): Extension<Arc<Config<'static>>>,
) -> impl IntoResponse {
    match utoipa_swagger_ui::serve(&tail[1..], state) {
        Ok(file) => file
            .map(|file| {
                (
                    StatusCode::OK,
                    [("Content-Type", file.content_type)],
                    file.bytes,
                )
                    .into_response()
            })
            .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response()),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/shortest-common-superstring",
    request_body = CreateShortestCommonSuperstring,
    responses(
        (status = 200, description = "Submitted", body = ShortestCommonSuperstring),
        (status = 400, description = "Bad request", body = ShortestCommonSuperstring)
    )
)]
async fn handler(Json(payload): Json<CreateShortestCommonSuperstring>) -> impl IntoResponse {
    println!("variant={:?}, arguments={:?}", payload.variant, payload.substrings);

    let input_clean = unique_by_substrings(payload.substrings);
    let resp;
    match payload.variant.as_str() {
        "naive" => {
            resp = (
                StatusCode::OK,
                Json(ShortestCommonSuperstring {
                    shortest_common_superstring: naive::shortest_superstring(input_clean),
                }),
            );
        }
        "tsp" => {
            resp = (
                StatusCode::OK,
                Json(ShortestCommonSuperstring {
                    shortest_common_superstring: tsp::shortest_superstring(input_clean),
                }),
            );
        }
        _ => {
            resp = (
                StatusCode::BAD_REQUEST,
                Json(ShortestCommonSuperstring {
                    shortest_common_superstring: String::from(""),
                }),
            );
        }
    };

    println!("resp={:?}", resp);

    resp
}

#[derive(Deserialize, Component)]
struct CreateShortestCommonSuperstring {
    variant: String,
    substrings: Vec<String>,
}

#[derive(Serialize, Component, Debug)]
struct ShortestCommonSuperstring {
    shortest_common_superstring: String,
}
