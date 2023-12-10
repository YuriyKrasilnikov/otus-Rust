mod c_lib;

use anyhow::Context;

use time::Duration;

use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use aide::{
    axum::{
        routing::{get, post},
        ApiRouter, IntoApiResponse,
    },
    openapi::{Info, OpenApi},
    redoc::Redoc,
};

use askama::Template;
use axum::{
    error_handling::HandleErrorLayer,
    extract::Json,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse},
    BoxError, Extension,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

use c_lib::{SmartHouseLib, SMARTHOUSESTORE};

use serde_json::json;
use uuid::Uuid;

const LIBRARY_PATH: &str = "libs/libsmart_house.so";
const SMARTHOUSE_KEY: &str = "SmartHouse";

// We'll need to derive `JsonSchema` for
// all types that appear in the api documentation.
#[derive(Deserialize, JsonSchema)]
struct Room {
    name: String,
}

#[derive(Deserialize, JsonSchema)]
struct Device {
    room: String,
    name: String,
}

async fn session_middleware<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse
where
    B: Send + 'static,
{
    let session = match req.extensions().get::<Session>() {
        Some(session) => session.clone(),
        None => {
            debug!("No session found for the current request, creating a new one");
            Session::new(None)
        }
    };

    let key = session
        .get(SMARTHOUSE_KEY)
        .expect("Could not deserialize.")
        .unwrap_or(Uuid::new_v4().to_string());

    session
        .insert(SMARTHOUSE_KEY, key.clone())
        .expect("Could not serialize.");

    req.extensions_mut().insert(key);

    next.run(req).await
}

async fn add_room(
    Extension(key): Extension<String>,
    Json(room): Json<Room>,
) -> impl IntoApiResponse {
    let smart_house_store = SMARTHOUSESTORE.read().unwrap();

    let smart_house_lib = smart_house_store.get(&key).expect("Can't get Smart House");

    smart_house_lib.add_room(room.name);

    Json(json!(smart_house_lib.get_list_rooms_name()))
}

async fn remove_room(
    Extension(key): Extension<String>,
    Json(room): Json<Room>,
) -> impl IntoApiResponse {
    let smart_house_store = SMARTHOUSESTORE.read().unwrap();

    let smart_house_lib = smart_house_store.get(&key).expect("Can't get Smart House");

    smart_house_lib.remove_room(room.name);

    Json(json!(smart_house_lib.get_list_rooms_name()))
}

async fn add_device(
    Extension(key): Extension<String>,
    Json(device): Json<Device>,
) -> impl IntoApiResponse {
    let smart_house_store = SMARTHOUSESTORE.read().unwrap();

    let smart_house_lib = smart_house_store.get(&key).expect("Can't get Smart House");

    smart_house_lib.add_test_device_outlet(device.room.clone(), device.name.clone(), device.name);

    Json(json!(smart_house_lib.get_list_devices_name(device.room)))
}

async fn remove_device(
    Extension(key): Extension<String>,
    Json(device): Json<Device>,
) -> impl IntoApiResponse {
    let smart_house_store = SMARTHOUSESTORE.read().unwrap();

    let smart_house_lib = smart_house_store.get(&key).expect("Can't get Smart House");

    smart_house_lib.remove_device(device.room.clone(), device.name);

    Json(json!(smart_house_lib.get_list_devices_name(device.room)))
}

async fn report(Extension(key): Extension<String>) -> impl IntoApiResponse {
    let smart_house_store = SMARTHOUSESTORE.read().unwrap();

    let smart_house_lib = smart_house_store.get(&key).expect("Can't get Smart House");

    format!("Smart_House {}", smart_house_lib.report())
}

async fn root(Extension(key): Extension<String>) -> impl IntoApiResponse {
    if !SMARTHOUSESTORE.read().unwrap().contains_key(&key) {
        debug!("No lib found, creating a new one");
        let mut smart_house_store_rw = SMARTHOUSESTORE.write().unwrap();
        smart_house_store_rw.insert(
            key.clone(),
            SmartHouseLib::new(LIBRARY_PATH.to_string(), key.clone()),
        );
    }

    let template = UITemplate {
        smart_house_key: key,
    };
    match template.render() {
        // If we're able to successfully parse and aggregate the template, serve it
        Ok(html) => Html(html).into_response(),
        // If we're not, return an error or some bit of fallback HTML
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error: {}", err),
        )
            .into_response(),
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct UITemplate {
    smart_house_key: String,
}

// Note that this clones the document on each request.
// To be more efficient, we could wrap it into an Arc,
// or even store it as a serialized string.
async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "webserver=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router...");

    let assets_path = std::env::current_dir().unwrap();
    let port = 3000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(600)));

    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(session_layer)
        .into_inner();

    let api_router = ApiRouter::new()
        // Change `route` to `api_route` for the route
        // we'd like to expose in the documentation.
        .api_route("/add_room", post(add_room))
        .api_route("/remove_room", post(remove_room))
        .api_route("/add_device", post(add_device))
        .api_route("/remove_device", post(remove_device))
        .api_route("/report", post(report));

    let middleware_router = ApiRouter::new()
        .nest("/api/v1", api_router)
        .route("/", get(root))
        .layer(middleware::from_fn(session_middleware));

    let router = ApiRouter::new()
        .merge(middleware_router)
        // generate redoc-ui using the openapi spec route
        .route("/redoc", Redoc::new("/api.json").axum_route())
        .route("/api.json", get(serve_api))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .layer(session_service)
        .layer(TraceLayer::new_for_http());

    info!("router initialized, now listening on port {}", port);

    let mut api = OpenApi {
        info: Info {
            description: Some("an Homework OTUS Rust API".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    axum::Server::bind(&addr)
        .serve(
            router
                // Generate the documentation.
                .finish_api(&mut api)
                // Expose the documentation to the handlers.
                .layer(Extension(api))
                .into_make_service(),
        )
        .await
        .context("error while starting server")?;

    Ok(())
}
