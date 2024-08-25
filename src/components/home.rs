use dioxus::prelude::*;
use crate::server::create_paste;
use crate::routes::Route;

#[component]
pub fn Home() -> Element {
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