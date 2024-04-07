#![allow(warnings)]
use anyhow::anyhow;
use build_html::Html;
use galgath::{
    convert_book_stuff_to_crowbook, get_reddit_link_fullname, make_link_url,
    retrieve_book_stuff_from_reddit_text_post,
};
use reqwest::header::CONTENT_DISPOSITION;
use std::{fmt::Display, sync::Arc};

use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket},
        Multipart, State, WebSocketUpgrade,
    },
    response::{IntoResponse, Response},
};
use build_html::{Container, ContainerType, Html as bHtml, HtmlContainer, HtmlPage};
use tokio::sync::RwLock;

pub(crate) async fn recieve_share(mut multipart: Multipart) -> impl IntoResponse {
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

    axum::http::StatusCode::SEE_OTHER
}

mod errors;
use errors::*;

mod types;
pub(crate) use types::*;

#[axum_macros::debug_handler]
pub(crate) async fn share_websocket(
    ws: WebSocketUpgrade,
    State(state): State<EbookState>,
) -> Response {
    ws.on_upgrade(|socket| async {
        share_websocket_handler(socket, state).await;
    })
}

/// Send Text over a websocket with format
macro_rules! st {
    ($ws:expr, $msg:expr, $($arg:tt)+) => {
        $ws.send(Message::Text(format!($msg, $($arg)+))).await
    };
    ($ws:expr, $msg:expr) => {
        $ws.send(Message::Text(format!($msg))).await
    };
}

async fn share_websocket_handler(mut s: WebSocket, state: EbookState) -> Result<(), anyhow::Error> {
    while let Some(msg) = s.recv().await {
        let text = match msg? {
            Message::Text(text) => text,
            oth_msg => {
                st!(s, "Got non-text websocket message: {oth_msg:?}")?;
                continue;
            }
        };

        st!(s, "Recieved text message: `{text}`... Parsing")?;

        let url = match url::Url::parse(&text) {
            Ok(url) => url,
            Err(perr) => {
                st!(
                    s,
                    "Error while parsing as url: {perr:?}... Terminating connection"
                );
                return Err(perr.into());
            }
        };

        st!(s, "Parsing complete!")?;

        if !url.domain().unwrap().ends_with("reddit.com") {
            st!(s, "Non reddit URL... Terminating connection")?;
            return Err(anyhow!("Non reddit string"));
        }

        let text_post_alone = make_link_url(get_reddit_link_fullname(url));

        st!(s, "Retrieving markdown from: {text_post_alone}...")?;

        let (title, author, markdown) =
            retrieve_book_stuff_from_reddit_text_post(text_post_alone).await?;

        st!(s, "Retrieved post! <i>{title}</i> by <i>{author}</i>")?;

        st!(s, "Converting to ePub...")?;

        let mut epubbuf: Vec<u8> = Vec::new();

        if let Err(err) = {
            let mut book =
                convert_book_stuff_to_crowbook((title.clone(), author.clone(), markdown))?;
            book.render_format_to("epub", &mut epubbuf)
        }{
            st!(s, "Error in convertin ePub! {:?}", err)?;
            Err(err)?
        }

        // println!("Convert result: {epubbuf:?}");

        st!(s, "Success! Saving...")?;

        let r = RenderedEpub {
            title,
            author,
            pages: Default::default(),
            ebook: epubbuf,
        };

        // println!("Saving ebook: {:?}", &r.ebook);

        {
            (*state.0.write().await) = Some(r);
        }

        st!(s, "ePub should now be available at the root url!")?;

        break;
    }

    return Ok(());
}

#[axum_macros::debug_handler]
pub(crate) async fn show_current_ebook(
    State(state): State<EbookState>,
) -> Result<axum::response::Html<String>, EbookFrontendError> {
    let (title, author, pages) = {
        match *state.0.read().await {
            Some(ref book) => (book.title.clone(), book.author.clone(), book.pages),
            None => return Err(errors::EbookFrontendError::no_book_in_storage()),
        }
    };

    let response = axum::response::Html(
        HtmlPage::new()
            .with_title("GalGath")
            .with_header(1, "GalGath ePub Converter")
            .with_link(
                "/epub/cur",
                Container::new(ContainerType::Article)
                    .with_attributes([("id", "currentbook")])
                    .with_header(2, title)
                    .with_paragraph(format!("By: {author}"))
                    .with_paragraph(format!("pp: {pages}"))
                    .to_html_string(),
            )
            .to_html_string(),
    );
    Ok(response)
}

pub(crate) async fn download_current_ebook(
    State(state): State<EbookState>,
) -> Result<Response, EbookFrontendError> {
    let (title, epub) = {
        match *state.0.read().await {
            Some(ref v) => (v.title.clone(), v.ebook.clone()),
            None => return Err(errors::EbookFrontendError::no_book_in_storage()),
        }
    };
    
    // println!("Sending ebook: {:?}", &epub);

    let resp = Response::builder()
        .header(
            CONTENT_DISPOSITION,
            format!("attachment; filename=\"{title}.epub\""),
        )
        .body(epub.into())?;

    Ok(resp)
}
