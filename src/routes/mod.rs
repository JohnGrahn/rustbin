use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::home::Home;
use crate::components::paste::Paste;

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/:id")]
    Paste { id: String },
}