#![allow(non_snake_case)]

use dioxus::prelude::*;
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

#[component]
fn Home() -> Element {
  rsx! {
    div { class: "bg-red-500 flex justify-center h-screen w-screen",
      h1 { class: "italic mt-20 text-3xl", "Culinetics" }
    }
  }
}
