use axum::http::{HeaderValue, Method, header};
use tower_http::cors::CorsLayer;

pub fn layer() -> Result<CorsLayer, Box<dyn std::error::Error>> {
    let origins = std::env::var("CORS_ALLOWED_ORIGINS")?
        .split(',')
        .map(str::trim)
        .map(|origin| {
            HeaderValue::from_str(origin)
                .expect("Invalid CORS origin in environment")
        })
        .collect::<Vec<_>>();

    Ok(CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
        ]))
}