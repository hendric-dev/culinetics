use reqwest::get;
use serde::Deserialize;

pub const RECIPES: [Config; 1] = [Config {
  recipe: manganis::mg!(file("public/recipes/pancakes.json")),
  image: manganis::mg!(file("public/recipes/pancakes.jpg")),
}];

pub struct Config {
  pub recipe: &'static str,
  pub image: &'static str,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Recipe {
  pub name: String,
  pub ingredients: Vec<String>,
  pub instructions: Vec<String>,
}

pub async fn fetch_recipe(path: &str) -> Result<Recipe, reqwest::Error> {
  let host = web_sys::window().unwrap().location().origin().unwrap();
  let url = format!("{host}{path}");
  get(url).await?.json::<Recipe>().await
}
