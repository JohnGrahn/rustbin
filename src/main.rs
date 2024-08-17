#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        div { class: "container mx-auto p-4 text-center",
            h1 { class: "text-4xl font-bold mb-4", "Blog post {id}" }
            Link { 
                class: "text-blue-500 hover:text-blue-700",
                to: Route::Home {}, 
                "Go to counter" 
            }
        }
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        div { class: "container mx-auto p-4 text-center",
            h1 { class: "text-4xl font-bold mb-4", "High-Five Counter" }
            div { class: "flex justify-center space-x-4 mb-4",
                button { 
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    onclick: move |_| count += 1, 
                    "Up high!"
                }
                button { 
                    class: "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded",
                    onclick: move |_| count -= 1, 
                    "Down low!"
                }
            }
            p { class: "text-2xl mb-4", "Count: {count}" }
            button {
                class: "bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded mb-4",
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        tracing::info!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            p { class: "text-xl", "Server data: {text}"}
            Link {
                class: "text-blue-500 hover:text-blue-700",
                to: Route::Blog {
                    id: count()
                },
                "Go to blog"
            }
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}