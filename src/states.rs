/*
^ All States Are Imported From Here
*/
mod home;
mod results;
mod create;

pub use home::HomeState;
pub use results::ResultViewer;
pub use create::CreateRecipe;
pub enum States {
    Home,
    ResultViewer,
    RecipeCreator,
}