use axum::http::StatusCode;
use url::Url;

const MAX_URL_LEN: usize = 512;

pub fn validate(raw: &str) -> Result<Url, (StatusCode, String)> {
    let raw = raw.trim();
    if raw.is_empty() || raw.len() > MAX_URL_LEN {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            format!("Invalid url length, must be less than {} characters", MAX_URL_LEN)
        ));
    }
    let url = Url::parse(raw).map_err(|e| (
        StatusCode::UNPROCESSABLE_ENTITY,
        format!("Invalid url structure: {}", e)
    ))?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            "Only http or https urls are allowed".to_string()
        ));
    }
    if url.host_str().is_none() {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            "Invalid url host".to_string()
        ));
    }
    Ok(url)
}