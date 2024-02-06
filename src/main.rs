use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::path::Path;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

async fn download_file() -> impl IntoResponse {
    let file_path = Path::new("file.txt");
    match File::open(file_path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/plain")
                .header("Content-Disposition", "attachment; filename=\"file.txt\"")
                .body(body)
                .unwrap();
            Ok(response)
        }
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not open file.")),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/download", get(download_file));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
