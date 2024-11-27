use crate::components::RecipeCard;
use crate::recipes::{fetch_recipe, RECIPES};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
  let resource = use_resource(move || fetch_recipe(RECIPES[0].recipe));

  match &*resource.read_unchecked() {
    Some(Ok(recipe)) => {
      rsx! {
        div { class: "bg-red-500 fixed h-screen overflow-y-auto p-8 w-screen",
          RecipeCard { image: RECIPES[0].image, recipe: recipe.clone() }
        }
      }
    }
    Some(Err(err)) => {
      rsx! { "{err}" } // TODO: Make nicer
    }
    None => {
      rsx! { "Empty" } // TODO: Make nicer
    }
  }
}
