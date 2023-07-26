// use std::cell::RefCell;
// use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;
use super::*;

// use crate::components::appstate::AppState;
#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <NavbarComponent/>
            <Switch<Router> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}