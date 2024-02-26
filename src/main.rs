use std::{fs, env::args};

use axum::{Router, routing::get, extract::State, response::Html};

#[derive(Clone)]
struct AppState {
    path: String,
}

#[tokio::main]
async fn main() {
    let path = args().skip(1).fold(String::new(), |mut acc, s| {
        acc.push_str(&s);
        acc
    });

    let state = AppState {
        path,
    };

    let app = Router::new()
        .route("/", get(page))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:1337").await.unwrap();
    println!("running on 127.0.0.1:1337");
    axum::serve(listener, app).await.unwrap();
}


async fn page(State(state): State<AppState>) -> Html<String> {
    let content = fs::read_to_string(state.path.clone())
        .unwrap_or(String::from(format!("file {} not found", state.path)));
    return Html(content);
}
