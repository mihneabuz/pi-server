use axum::{response::Html, response::IntoResponse, routing::get, Router};
use maud::{html, Markup};

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(std::thread::available_parallelism().unwrap().get())
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3000))
            .await
            .unwrap();

        axum::serve(listener, app()).await.unwrap();
    })
}

fn app() -> Router {
    Router::new().route("/", get(root))
}

fn render(markup: Markup) -> Html<String> {
    Html::from(markup.into_string())
}

async fn root() -> impl IntoResponse {
    render(html! {
        h1 { "Hello, World!" }
    })
}
