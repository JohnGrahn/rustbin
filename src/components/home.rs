use dioxus::prelude::*;
use crate::server::create_paste;
use crate::routes::Route;
use crate::models::ExpirationTime;
use crate::encryption::{generate_key, encrypt};
use base64::{engine::general_purpose, Engine};

#[component]
pub fn Home() -> Element {
    let mut content = use_signal(|| String::new());
    let mut error = use_signal(|| None::<String>);
    let mut expiration = use_signal(|| ExpirationTime::OneHour);
    let mut burn_after_read = use_signal(|| false);
    let mut display_format = use_signal(|| String::from("PlainText"));

    let create_paste = move |_| {
        let content_str = content.read().to_string();
        let expiration_value = *expiration.read();
        let burn_after_read_value = *burn_after_read.read();
        let display_format_value = display_format.read().to_string();
        error.set(None);

        spawn(async move {
            let key = generate_key();
            match encrypt(&content_str, &key) {
                Ok(encrypted_content) => {
                    match create_paste(encrypted_content, expiration_value, burn_after_read_value, display_format_value).await {
                        Ok(id) => {
                            let navigator = use_navigator();
                            let key_base64 = general_purpose::URL_SAFE_NO_PAD.encode(key);
                            navigator.push(Route::Paste { id: format!("{}-{}", id, key_base64) });
                        }
                        Err(e) => error.set(Some(e.to_string())),
                    }
                }
                Err(e) => error.set(Some(format!("Encryption error: {}", e))),
            }
        });
    };

    rsx! {
        div { class: "flex flex-col md:flex-row h-screen bg-gray-100",
            // Text area
            div { class: "flex-grow p-4",
                textarea {
                    class: "w-full h-full p-4 border rounded resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white",
                    placeholder: "Enter your paste content here",
                    oninput: move |evt| content.set(evt.value().clone()),
                }
            }
            // Menu area
            div { class: "w-full md:w-64 bg-white p-4 border-l border-gray-200",
                h2 { class: "text-xl font-bold mb-4", "Paste Options" }
                // Expiration dropdown
                div { class: "mb-4",
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Expiration Time" }
                    select {
                        class: "w-full p-2 border rounded focus:outline-none focus:ring-2 focus:ring-blue-500",
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
                        option { value: "FiveMinutes", selected: matches!(*expiration.read(), ExpirationTime::FiveMinutes), "5 minutes" }
                        option { value: "TenMinutes", selected: matches!(*expiration.read(), ExpirationTime::TenMinutes), "10 minutes" }
                        option { value: "ThirtyMinutes", selected: matches!(*expiration.read(), ExpirationTime::ThirtyMinutes), "30 minutes" }
                        option { value: "OneHour", selected: matches!(*expiration.read(), ExpirationTime::OneHour), "1 hour" }
                        option { value: "TwelveHours", selected: matches!(*expiration.read(), ExpirationTime::TwelveHours), "12 hours" }
                        option { value: "OneDay", selected: matches!(*expiration.read(), ExpirationTime::OneDay), "1 day" }
                        option { value: "OneWeek", selected: matches!(*expiration.read(), ExpirationTime::OneWeek), "1 week" }
                        option { value: "TwoWeeks", selected: matches!(*expiration.read(), ExpirationTime::TwoWeeks), "2 weeks" }
                        option { value: "OneMonth", selected: matches!(*expiration.read(), ExpirationTime::OneMonth), "1 month" }
                    }
                }
                // Burn after reading checkbox
                div { class: "mb-4 flex items-center",
                    input {
                        r#type: "checkbox",
                        id: "burn-after-read",
                        class: "mr-2",
                        checked: *burn_after_read.read(),
                        oninput: move |evt| {
                            burn_after_read.set(evt.value().parse().unwrap_or(false));
                            // We don't change the expiration time here
                        },
                    }
                    label { r#for: "burn-after-read", class: "text-sm font-medium text-gray-700", "Burn after reading" }
                }
                // Display format dropdown
                div { class: "mb-4",
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Display Format" }
                    select {
                        class: "w-full p-2 border rounded focus:outline-none focus:ring-2 focus:ring-blue-500",
                        onchange: move |evt| display_format.set(evt.value().clone()),
                        option { value: "PlainText", "Plain Text" }
                        option { value: "SourceCode", "Source Code" }
                        option { value: "Markdown", "Markdown" }
                    }
                }
                // Create Paste button
                button {
                    class: "w-full bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded transition duration-200",
                    onclick: create_paste,
                    "Create Paste"
                }
                // Error message
                {error.read().as_ref().map(|err| rsx!(
                    p { class: "text-red-500 mt-2 text-center", "{err}" }
                ))}
            }
        }
    }
}