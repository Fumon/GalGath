use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::assets::icons;

pub(crate) async fn serve_pwa() -> Html<&'static str> {
    Html(
        include_str!("assets/pages/pwa.html"),
    )
}

pub(crate) async fn serve_share_status() -> Html<&'static str> {
    Html(
        include_str!("assets/pages/share_status.html"),
    )
}

pub(crate) async fn serve_service_worker() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "application/javascript")],
        include_str!("assets/scripts/service-worker.js"),
    )
}

pub(crate) async fn serve_manifest() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "application/manifest+json")],
        include_str!("assets/manifest.json"),
    )
}

pub(crate) async fn serve_icon(Path(size): Path<i32>) -> impl IntoResponse {
    match icons::get_icon(size) {
        Some(icon) => (
            StatusCode::OK,
            [("Content-Type", mime::IMAGE_PNG.to_string())],
            icon,
        ),
        None => (
            StatusCode::BAD_REQUEST,
            [("Content-Type", mime::TEXT_PLAIN.to_string())],
            "Invalid icon size".as_bytes(),
        ),
    }
}

pub(crate) async fn serve_favicon() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "image/x-icon")],
        icons::FAVICON.as_slice(),
    )
}
