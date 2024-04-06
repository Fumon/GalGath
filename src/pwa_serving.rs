use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::icons;

pub(crate) async fn serve_pwa() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Minimal PWA Share Target</title>
    <link rel="manifest" href="manifest.json">
    <script>
        if ('serviceWorker' in navigator) {
            window.addEventListener('load', () => {
                navigator.serviceWorker.register('service-worker.js')
                    .then(reg => console.log('Service Worker registered', reg))
                    .catch(err => console.log('Service Worker registration failed', err));
            });
        }
    </script>
</head>
<body>
    <h1>Minimal PWA Share Target</h1>
    <p>This is a minimal Progressive Web App that can be installed and used as a share target.</p>
</body>
</html>"#,
    )
}

pub(crate) async fn serve_service_worker() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "application/javascript")],
        r#"
self.addEventListener('install', event => {
    console.log('Service Worker installing.');
});
    
// self.addEventListener('fetch', event => {
//     // You might want to handle fetch events for caching in the future
// });"#,
    )
}

pub(crate) async fn serve_manifest() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "application/manifest+json")],
        r##"{
    "name": "GalGath Share Target",
    "short_name": "GalGath",
    "description": "A minimal PWA that can receive shared content.",
    "start_url": "/pwa",
    "scope": "/pwa",
    "display": "standalone",
    "background_color": "#000000",
    "theme_color": "#ffffff",
    "icons": [
        {
            "src": "/icon/48",
            "sizes": "48x48",
            "type": "image/png"
        },
        {
            "src": "/icon/72",
            "sizes": "72x72",
            "type": "image/png"
        },
        {
            "src": "/icon/96",
            "sizes": "96x96",
            "type": "image/png"
        },
        {
            "src": "/icon/144",
            "sizes": "144x144",
            "type": "image/png"
        },
        {
            "src": "/icon/192",
            "sizes": "192x192",
            "type": "image/png"
        }
    ],
    "share_target": {
        "action": "/pwa/share",
        "method": "POST",
        "enctype": "multipart/form-data",
        "params": {
            "files": [{
                "name": "file",
                "accept": ["text/*"]
            }]
        }
    }
}"##,
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
