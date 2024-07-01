use axum::extract::DefaultBodyLimit;
use lib::axum::app::AppBuilder;
use lib::axum::extractor::MultipartFiles;
use lib::routes;

// 0 or more
async fn with_optional_file(files: Option<MultipartFiles>) -> String {
    format!(
        "{:?}",
        files.map(|files| files
            .0
            .into_iter()
            .map(|file| file.filename)
            .collect::<Vec<_>>())
    )
}

// 1 or more files
async fn handler(MultipartFiles(files): MultipartFiles) -> String {
    format!(
        "{:?} uploaded",
        files
            .into_iter()
            .map(|file| file.filename)
            .collect::<Vec<_>>()
    )
}

#[tokio::main]
async fn main() {
    let route = routes!(
        get "/" => handler,
        get "/opt" => with_optional_file
    )
    .layer(DefaultBodyLimit::disable());
    AppBuilder::new().route(route).serve().await.unwrap();
}
