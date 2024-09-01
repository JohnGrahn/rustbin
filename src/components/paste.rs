use dioxus::prelude::*;
use crate::server::get_paste;
use crate::routes::Route;
use chrono::Utc;
use pulldown_cmark::{Parser, html::push_html};
use chrono::Duration;

fn format_duration(duration: Duration) -> String {
    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{} day{}", days, if days > 1 { "s" } else { "" }));
    }
    if hours > 0 {
        parts.push(format!("{} hour{}", hours, if hours > 1 { "s" } else { "" }));
    }
    if minutes > 0 || (days == 0 && hours == 0) {
        parts.push(format!("{} minute{}", minutes, if minutes > 1 { "s" } else { "" }));
    }

    parts.join(", ")
}

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
                            {match paste_data.display_format.as_str() {
                                "PlainText" => rsx! {
                                    pre { class: "bg-gray-100 p-4 rounded overflow-x-auto",
                                        code { class: "text-sm", "{paste_data.content}" }
                                    }
                                },
                                "SourceCode" => rsx! {
                                    pre { class: "bg-gray-100 p-4 rounded overflow-x-auto",
                                        code { class: "text-sm",
                                            {paste_data.content.lines().enumerate().map(|(i, line)| {
                                                rsx! {
                                                    span { class: "mr-4 text-gray-500", "{i + 1}" }
                                                    "{line}\n"
                                                }
                                            })}
                                        }
                                    }
                                },
                                "Markdown" => {
                                    let mut html_output = String::new();
                                    let parser = Parser::new(&paste_data.content);
                                    push_html(&mut html_output, parser);
                                    rsx! {
                                        div { class: "bg-gray-100 p-4 rounded overflow-x-auto prose",
                                            dangerous_inner_html: "{html_output}"
                                        }
                                    }
                                },
                                _ => rsx! {
                                    pre { class: "bg-gray-100 p-4 rounded overflow-x-auto",
                                        code { class: "text-sm", "{paste_data.content}" }
                                    }
                                }
                            }}
                            p { class: "mt-4 text-sm text-gray-600",
                                "Created at: {paste_data.created_at}"
                            }
                            p { class: "mt-2 text-sm text-gray-600",
                                "Expires in: {format_duration(time_left)}"
                            }
                            {if paste_data.burn_after_read {
                                Some(rsx!(
                                    p { class: "mt-2 text-sm text-red-600 font-bold",
                                        "Warning: This paste will be deleted after viewing (30 seconds grace period after creation)."
                                    }
                                ))
                            } else {
                                None
                            }}
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