mod home;
mod results;

pub use home::HomeState;
pub use results::ResultViewer;

pub enum States {
    Home,
    ResultViewer
}