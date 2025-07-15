mod api;
mod config;
mod models;
mod repository;
mod schema;
mod services;

mod api_doc;

use crate::config::settings::Settings;
use api::routes::create_router;
use axum::Router;
use config::init_pool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let settings = Settings::new();
    let pool = init_pool(&settings.database_url);
    let mut app: Router<()> = Router::new();
    let app_router = create_router(pool);

    let docs =
        SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", api_doc::ApiDoc::openapi());

    // let app: OpenApiRouter<_> = OpenApiRouter::with_openapi(ApiDoc::openapi())
    //     .nest("/api", app_router.into())
    //     .merge(docs); // rota /swagger-ui
    app = app.nest("/api/v1", app_router).merge(docs);
    let addr: String = format!("{}:{}", settings.server_host, settings.server_port)
        .parse()
        .unwrap();

    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
