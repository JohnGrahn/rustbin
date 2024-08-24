#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/paste/:id")]
    Paste { id: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasteData {
    pub id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

fn main() {
    #[cfg(feature = "server")]
    dotenv::dotenv().ok();
    
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
fn Home() -> Element {
    let mut content = use_signal(|| String::new());
    let mut error = use_signal(|| None::<String>);

    let create_paste = move |_| {
        let content_str = content.read().to_string();
        error.set(None);
        spawn(async move {
            match create_paste(content_str).await {
                Ok(id) => {
                    let navigator = use_navigator();
                    navigator.push(Route::Paste { id });
                }
                Err(e) => error.set(Some(e.to_string())),
            }
        });
    };

    rsx! {
        div { class: "min-h-screen bg-gray-100 flex flex-col items-center justify-center p-4",
            div { class: "w-full max-w-md bg-white rounded-lg shadow-md p-6",
                h1 { class: "text-3xl font-bold mb-6 text-center text-gray-800", "Rustbin" }
                textarea {
                    class: "w-full h-64 p-2 border rounded mb-4 resize-none focus:outline-none focus:ring-2 focus:ring-blue-500",
                    placeholder: "Enter your paste content here",
                    oninput: move |evt| content.set(evt.value().clone()),
                }
                button {
                    class: "w-full bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded transition duration-200",
                    onclick: create_paste,
                    "Create Paste"
                }
                {error.read().as_ref().map(|err| rsx!(
                    p { class: "text-red-500 mt-2 text-center", "{err}" }
                ))}
            }
        }
    }
}

#[component]
fn Paste(id: String) -> Element {
    let id_for_display = id.clone();
    let paste = use_resource(move || {
        let id = id.clone();
        async move { get_paste(id).await }
    });

    rsx! {
        div { class: "min-h-screen bg-gray-100 flex flex-col items-center justify-center p-4",
            div { class: "w-full max-w-2xl bg-white rounded-lg shadow-md p-6",
                h1 { class: "text-3xl font-bold mb-6 text-center text-gray-800", "Paste {id_for_display}" }
                {match paste.read().as_ref() {
                    Some(Ok(paste_data)) => rsx! {
                        pre { class: "bg-gray-100 p-4 rounded overflow-x-auto",
                            code { class: "text-sm", "{paste_data.content}" }
                        }
                        p { class: "mt-4 text-sm text-gray-600",
                            "Created at: {paste_data.created_at}"
                        }
                    },
                    Some(Err(e)) => rsx! { p { class: "text-red-500", "Error loading paste: {e}" } },
                    None => rsx! { p { class: "text-gray-600", "Loading..." } },
                }}
                Link {
                    class: "mt-6 inline-block bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded transition duration-200",
                    to: Route::Home {},
                    "Back to Home"
                }
            }
        }
    }
}

#[server(CreatePaste)]
async fn create_paste(content: String) -> Result<String, ServerFnError> {
    use rand::Rng;
    use sqlx::PgPool;

    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    let id: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    sqlx::query!(
        "INSERT INTO pastes (id, content, created_at) VALUES ($1, $2, $3)",
        id,
        content,
        Utc::now()
    )
    .execute(&pool)
    .await?;

    Ok(id)
}

#[server(GetPaste)]
async fn get_paste(id: String) -> Result<PasteData, ServerFnError> {
    use sqlx::PgPool;

    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    let paste = sqlx::query_as!(
        PasteData,
        "SELECT id, content, created_at FROM pastes WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok(paste)
}