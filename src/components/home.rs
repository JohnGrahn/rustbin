use dioxus::prelude::*;
use crate::server::create_paste;
use crate::routes::Route;
use crate::models::ExpirationTime;

#[component]
pub fn Home() -> Element {
    let mut content = use_signal(|| String::new());
    let mut error = use_signal(|| None::<String>);
    let mut expiration = use_signal(|| ExpirationTime::OneHour);

    let create_paste = move |_| {
        let content_str = content.read().to_string();
        let expiration_value = *expiration.read();
        error.set(None);
        spawn(async move {
            match create_paste(content_str, expiration_value).await {
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
                select {
                    class: "w-full p-2 border rounded mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500",
                    onchange: move |evt| {
                        match evt.value().as_str() {
                            "FiveMinutes" => expiration.set(ExpirationTime::FiveMinutes),
                            "TenMinutes" => expiration.set(ExpirationTime::TenMinutes),
                            "ThirtyMinutes" => expiration.set(ExpirationTime::ThirtyMinutes),
                            "OneHour" => expiration.set(ExpirationTime::OneHour),
                            "TwelveHours" => expiration.set(ExpirationTime::TwelveHours),
                            "OneDay" => expiration.set(ExpirationTime::OneDay),
                            "OneWeek" => expiration.set(ExpirationTime::OneWeek),
                            "TwoWeeks" => expiration.set(ExpirationTime::TwoWeeks),
                            "OneMonth" => expiration.set(ExpirationTime::OneMonth),
                            _ => error.set(Some("Invalid expiration time selected".to_string())),
                        }
                    },
                    option { value: "FiveMinutes", "5 minutes" }
                    option { value: "TenMinutes", "10 minutes" }
                    option { value: "ThirtyMinutes", "30 minutes" }
                    option { value: "OneHour", selected: true, "1 hour" }
                    option { value: "TwelveHours", "12 hours" }
                    option { value: "OneDay", "1 day" }
                    option { value: "OneWeek", "1 week" }
                    option { value: "TwoWeeks", "2 weeks" }
                    option { value: "OneMonth", "1 month" }
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