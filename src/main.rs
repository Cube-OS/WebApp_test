pub mod web;
// mod app;
pub use crate::web::*;

fn main() {
    yew::start_app::<web::Main>();
}
