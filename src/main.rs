#![allow(non_snake_case)]

mod components;
mod pages;
mod recipes;

use dioxus::prelude::*;
use pages::Home;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
  #[route("/")]
  Home {},
}

fn main() {
  dioxus_logger::init(Level::INFO).expect("Failed to initialize logger");
  launch(App);
}

fn App() -> Element {
  rsx! {
    Router::<Route> {}
  }
}
