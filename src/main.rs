mod api;
mod config;
mod models;
mod repository;
mod schema;
mod services;

mod api_doc;

use crate::api::handlers::fingerprint::mqtt_client_task;
use crate::config::settings::Settings;
use api::routes::create_user_router;
use config::init_pool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use axum::Router;

use crate::api::routes::create_mqtt_router;
use crate::models::fingerprint::AppState;

#[tokio::main]
async fn main() {
    let settings = Settings::new();
    let pool = init_pool(&settings.database_url);
    let app_router = create_user_router(pool);

    let docs =
        SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", api_doc::ApiDoc::openapi());

    // Start MQTT Client
    let state = AppState::default();

    {
        let mqtt_state = state.clone();
        tokio::spawn(async move {
            if let Err(e) = mqtt_client_task(mqtt_state).await {
                eprintln!("MQTT client error: {}", e);
            }
        });
    }

    let app_mqtt = create_mqtt_router(state.clone());

    // let app: OpenApiRouter<_> = OpenApiRouter::with_openapi(ApiDoc::openapi())
    //     .nest("/api", app_router.into())
    //     .merge(docs); // rota /swagger-ui
    let app = Router::new()
        .nest("/api/v1", app_mqtt)
        .nest("/api/v1", app_router)
        .merge(docs)
        .with_state(state.clone());

    let addr: String = format!("{}:{}", settings.server_host, settings.server_port)
        .parse()
        .unwrap();

    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
