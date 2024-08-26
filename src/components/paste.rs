use dioxus::prelude::*;
use crate::server::get_paste;
use crate::routes::Route;
use chrono::Utc;

#[component]
pub fn Paste(id: String) -> Element {
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
                    Some(Ok(paste_data)) => {
                        let now = Utc::now();
                        let time_left = paste_data.expires_at.signed_duration_since(now);
                        rsx! {
                            pre { class: "bg-gray-100 p-4 rounded overflow-x-auto",
                                code { class: "text-sm", "{paste_data.content}" }
                            }
                            p { class: "mt-4 text-sm text-gray-600",
                                "Created at: {paste_data.created_at}"
                            }
                            p { class: "mt-2 text-sm text-gray-600",
                                "Expires in: {time_left.num_minutes()} minutes"
                            }
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