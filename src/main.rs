mod api;
mod config;
mod models;
mod repository;
mod schema;
mod services;

mod api_doc;

use crate::config::settings::Settings;
use api::routes::create_router;
use config::init_pool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use axum::{
    extract::State,
    http::{StatusCode, HeaderMap, header},
    response::{Json, Response},
    routing::get,
    Router,
};


use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Packet};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

// Shared state to store the last fingerprint
#[derive(Clone, Default)]
struct AppState {
    latest_fingerprint: Arc<RwLock<Option<String>>>,
}

#[derive(Serialize)]
struct FingerprintResponse {
    // Can be used for validation
    has_data: bool,
    data_size: usize,

    // The actual fingerprint data
    fingerprint: Option<String>,
}

#[tokio::main]
async fn main() {
    let settings = Settings::new();
    let pool = init_pool(&settings.database_url);
    let mut app: Router<()> = Router::new();
    let app_router = create_router(pool);

    let docs =
        SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", api_doc::ApiDoc::openapi());

    // Create Shared State
    let state = AppState::default();

    // Start MQTT Client
    let mqtt_state = state.clone();
    tokio::spawn(async move {
        if let Err(e) = mqtt_client_task(mqtt_state).await {
            eprintln!("MQTT client error: {}", e);
        }
    });

    // Create Axum Router
    let app = Router::new()
        .route("/fingerprint", get(get_fingerprint))
        .route("/image", get(get_image))
        .route("/health", get(health_check))
        .with_state(state);

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

async fn mqtt_client_task(state: AppState) -> Result<(), Box<dyn std::error::Error>> {

    // Configure MQTT client
    let mut mqttoptions = MqttOptions::new("biometric_client", "mosquitto", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(30));

    // Create MQTT client
    let (client, mut connection) = AsyncClient::new(mqttoptions, 10);

    // Subscribe to the fingerprint topic
    let topic = "fingerprint/image";
    client.subscribe(topic, QoS::AtMostOnce).await?;
    println!("Subscribed to topic: {}", topic);

    // Handle incoming messages
    loop {
        match connection.poll().await {
            Ok(notification) => {
                if let rumqttc::Event::Incoming(packet) = notification {
                    if let rumqttc::Packet::Publish(publish) = packet {
                        if publish.topic == "fingerprint/image" {
                            // Convert payload to string and store it
                            match String::from_utf8(publish.payload.to_vec()) {
                                Ok(fingerprint_string) => {
                                    let mut fingerprint = state.latest_fingerprint.write().await;
                                    *fingerprint = Some(fingerprint_string.clone());
                                    println!("Received fingerprint data: {} characters", fingerprint_string.len());
                                }
                                Err(e) => {
                                    eprintln!("Failed to parse fingerprint data as UTF-8: {}", e);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("MQTT connection error: {}", e);
                sleep(Duration::from_secs(5)).await; // Wait before retrying
            }
        }
    }
}

// API endpoint to get the latest fingerprint data
async fn get_fingerprint(State(state): State<AppState>) -> Result<Json<FingerprintResponse>, StatusCode> {
    let fingerprint = state.latest_fingerprint.read().await;

    match fingerprint.as_ref() {
        Some(data) => Ok(Json(FingerprintResponse {
            has_data: true,
            data_size: data.len(),
            fingerprint: Some(data.clone()),
        })),
        None => Ok(Json(FingerprintResponse {
            has_data: false,
            data_size: 0,
            fingerprint: None,
        })),
    }
}

// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

// Aux function to convert hex data to PNG
fn hex_to_png(hex_data: &str) -> Result<Vec<u8>, String> {

    // Parse hex into bytes
    let mut data4 = Vec::new();
    let cleaned = hex_data.chars().filter(|c| c.is_ascii_hexdigit()).collect::<String>();

    for chunk in cleaned.as_bytes().chunks(2) {
        if chunk.len() == 2 {
            let hex_pair = std::str::from_utf8(chunk)
                .map_err(|e| e.to_string())?;

            let byte = u8::from_str_radix(hex_pair, 16)
                .map_err(|e| e.to_string())?;

            data4.push(byte);
        }
    }

    let expected = 3200;
    if data4.len() < expected {
        return Err(format!("Expected {} bytes, got {}", expected, data4.len()));
    }

    // Expand 4-bit to 8-bit
    let mut pixels = Vec::new();
    for &byte in &data4[..expected] {
        let hi = (byte >> 4) & 0x0F;
        let lo = byte & 0x0F;
        pixels.push((hi << 4) | hi);
        pixels.push((lo << 4) | lo);
    }

    // Create PNG image
    let mut png_data = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut png_data, 80, 80);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
        writer.write_image_data(&pixels).map_err(|e| e.to_string())?;
    }

    Ok(png_data)
}

// API endpoint to get the fingerprint image
async fn get_image(State(state): State<AppState>) -> Result<Response, StatusCode> {
    let fingerprint = state.latest_fingerprint.read().await;

    match fingerprint.as_ref() {
        Some(data) => {
            match hex_to_png(data) {
                Ok(png_data) => {
                    let mut headers = HeaderMap::new();
                    headers.insert(
                        header::CONTENT_TYPE,
                        "image/png".parse().unwrap(),
                    );

                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "image/png")
                        .body(png_data.into())
                        .unwrap())
                }
                Err(e) => {
                    eprintln!("Error converting hex to PNG: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        None => {
            eprintln!("No fingerprint data available");
            Err(StatusCode::NOT_FOUND)
        }
    }
}

