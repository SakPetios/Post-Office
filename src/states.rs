/*
^ All States Are Imported From Here
*/
mod home;
mod results;
mod create;
mod console;

pub use home::HomeState;
pub use results::ResultViewer;
pub use create::CreateRecipe;
pub use console::Console;
pub enum States {
    Home,
    RecipeCreator,
    LuaConsole,
}