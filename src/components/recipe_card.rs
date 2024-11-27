use crate::recipes::Recipe;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct RecipeCardProps {
  image: &'static str,
  recipe: Recipe,
}

#[component]
pub fn RecipeCard(props: RecipeCardProps) -> Element {
  rsx! {
    div { class: "flex flex-col gap-4 items-center",
      h1 { class: "font-bold", "{props.recipe.name}" }
      img { class: "h-80 w-80", src: props.image }
      h1 { class: "font-bold", "Ingredients" }
      ul { class: "list-disc",
        for ingredient in props.recipe.ingredients {
          li { "{ingredient}" }
        }
      }
      h1 { class: "font-bold", "Instructions" }
      ol { class: "list-decimal",
        for step in props.recipe.instructions {
          li { class: "my-2", "{step}" }
        }
      }
    }
  }
}
