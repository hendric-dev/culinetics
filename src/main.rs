use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct Recipe {
    id: String,
    name: String,
    image: String,
    ingredients: Vec<String>,
    instructions: Vec<String>,
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const CHILI_CON_CARNE_IMG: Asset = asset!("/assets/recipes/chili-con-carne.jpg");
const CHILI_CON_CARNE_RECIPE_JSON: &str = include_str!("../assets/recipes/chili-con-carne.json");
const GYUDON_IMG: Asset = asset!("/assets/recipes/gyudon.jpg");
const GYUDON_RECIPE_JSON: &str = include_str!("../assets/recipes/gyudon.json");
const PANCAKES_IMG: Asset = asset!("/assets/recipes/pancakes.jpg");
const PANCAKES_RECIPE_JSON: &str = include_str!("../assets/recipes/pancakes.json");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let chili_recipe: Recipe = serde_json::from_str(CHILI_CON_CARNE_RECIPE_JSON).unwrap();
    let gyudon_recipe: Recipe = serde_json::from_str(GYUDON_RECIPE_JSON).unwrap();
    let pancakes_recipe: Recipe = serde_json::from_str(PANCAKES_RECIPE_JSON).unwrap();

    rsx! {
        RecipeDisplay { recipe: chili_recipe, img: CHILI_CON_CARNE_IMG }
        RecipeDisplay { recipe: gyudon_recipe, img: GYUDON_IMG }
        RecipeDisplay { recipe: pancakes_recipe, img: PANCAKES_IMG }
    }
}

#[component]
fn RecipeDisplay(recipe: Recipe, img: Asset) -> Element {
    rsx! {
        div { class: "flex gap-6 items-center",
            img { class: "w-100", src: img }
            ul {
                for ingredient in recipe.ingredients.iter() {
                    li { "{ingredient}" }
                }
            }
            ul {
                for ingredient in recipe.instructions.iter() {
                    li { "{ingredient}" }
                }
            }
        }
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
        }

        Outlet::<Route> {}
    }
}
