use axum::{routing::get, Router};

mod assets;
mod ebook_frontend;
mod pwa_serving;

const PORT: u32 = 3000;

#[tokio::main]
async fn main() {
    let ebookstate: ebook_frontend::EbookState = Default::default();

    let routes = Router::new()
        .route(
            "/robots.txt",
            get(|| async { "User-agent: *\nDisallow: /\n\nUser-agent: AdsBot\nDisallow: /\n" }),
        )
        .route("/pwa", get(pwa_serving::serve_pwa))
        .route(
            "/pwa/share_status.html",
            get(pwa_serving::serve_share_status),
        )
        .route("/service-worker.js", get(pwa_serving::serve_service_worker))
        .route("/manifest.json", get(pwa_serving::serve_manifest))
        .route("/icon/:size", get(pwa_serving::serve_icon))
        .route("/favicon.ico", get(pwa_serving::serve_favicon))
        .route("/pwa/sharews", get(ebook_frontend::share_websocket))
        // .route("/pwa/share", post(ebook_frontend::recieve_share))
        .route("/", get(ebook_frontend::show_current_ebook))
        .route("/epub/cur", get(ebook_frontend::download_current_ebook))
        // .route(
        //     "/stat/root_contents",
        //     get(|| async {
        //         std::fs::read_dir("/").unwrap().into_iter().filter_map(|rd| rd.ok()).filter_map(|rdd| rdd.file_name().into_string().ok()).collect::<Vec<String>>().join("\n")
        //     }),
        // )
        .with_state(ebookstate);

    println!("Running server on {PORT}...");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}"))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
    println!("Goodbye");
}
