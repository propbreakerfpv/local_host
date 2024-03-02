use std::fs;

use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version)]
struct Arguments {
    #[arg(short, long)]
    rout: Vec<String>,
    #[arg(short, long)]
    file: Vec<String>,
    #[arg(short, long, default_value = "1337")]
    port: u16,
}

#[derive(Clone)]
struct AppState {
    files: Vec<String>,
    routs: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    let state = AppState {
        files: args.file.clone(),
        routs: args.rout.clone(),
    };

    let app = Router::new().route("/*key", get(page)).with_state(state);

    let address = format!("127.0.0.1:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("running on {}", address);
    axum::serve(listener, app).await.unwrap();
}

async fn page(Path(path): Path<String>, State(state): State<AppState>) -> Html<String> {
    print!("got {} ", path);
    for (idx, rout) in state.routs.iter().enumerate() {
        let file = state.files.get(idx).unwrap();
        println!("file: {}", file);
        if rout.split('/').last() == Some(&path) {
            let content = fs::read_to_string(file).unwrap_or(format!("file {} not found", rout));
            println!("found file {}", file);
            return Html(content);
        }
    }
    println!("file not found");
    Html(format!("{} not found", path))
}
