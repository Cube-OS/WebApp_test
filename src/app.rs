// use std::cell::RefCell;
// use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::{self};
use crate::components::navbar::NavbarComponent;
// use crate::components::appstate::AppState;
#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <NavbarComponent/>
            <Switch<router::Route> render={Switch::render(router::switch)} />
        </BrowserRouter>
    }
}