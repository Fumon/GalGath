use axum::{
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

mod icons;
mod pwa_serving;

const PORT: u32 = 3000;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/robots.txt",
            get(|| async { "User-agent: *\nDisallow: /\n\nUser-agent: AdsBot\nDisallow: /\n" }),
        )
        .route("/pwa", get(pwa_serving::serve_pwa))
        .route("/service-worker.js", get(pwa_serving::serve_service_worker))
        .route("/manifest.json", get(pwa_serving::serve_manifest))
        .route("/icon/:size", get(pwa_serving::serve_icon))
        .route("/favicon.ico", get(pwa_serving::serve_favicon))
        .route("/pwa/share", post(recieve_share));

    println!("Running server on {PORT}...");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Goodbye");
}

async fn recieve_share(mut multipart: Multipart) -> impl IntoResponse {
    println!("Got share:");
    let mut field_count = 0;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap();
        let mime: mime::Mime = field.content_type().unwrap().parse().unwrap();
        println!("  Field {field_count}; Name: {name}; Mime: {mime}");

        match (mime.type_(), mime.subtype()) {
            (mime::TEXT, _) => println!("    Text: {:?}", field.bytes().await.unwrap()),
            (_, _) => println!("    Non-text"),
        }

        // println!("    Full field: {:?}", field);
        field_count += 1;
    }

    StatusCode::SEE_OTHER
}
