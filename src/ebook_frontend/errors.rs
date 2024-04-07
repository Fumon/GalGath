

pub(crate) struct EbookFrontendError(pub anyhow::Error);

impl EbookFrontendError {
    pub(crate) fn no_book_in_storage() -> EbookFrontendError {
        EbookFrontendError(anyhow::anyhow!("No epubs have been converted yet"))
    }
}

impl axum::response::IntoResponse for EbookFrontendError {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        ).into_response()
    }
}

impl<E> From<E> for EbookFrontendError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
