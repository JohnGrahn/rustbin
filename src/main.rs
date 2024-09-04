#![allow(non_snake_case)]

mod components;
mod routes;
mod server;
mod models;
mod jobs;
mod encryption;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use routes::Route;



fn main() {
    #[cfg(feature = "server")]
    {
        use tokio::runtime::Runtime;
        dotenv::dotenv().ok();
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");

        std::thread::spawn(move || {
            runtime.block_on(async {
                jobs::cleanup::run_cleanup_job().await;
            });
        });
    }
    
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}