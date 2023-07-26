use yew::prelude::*;
use yew_router::{history::History, prelude::RouterScopeExt};  // Import necessary traits and structs
use crate::web::{Route};  // Import your Route enum

pub struct NavbarComponent;

impl Component for NavbarComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().history().unwrap();  // Create a 'history' object
        let history_clone = history.clone();
        let onclick = Callback::from(move |_: MouseEvent| {
            // let mut app_state = APP_STATE.lock().unwrap();
            // app_state.reset();
            history_clone.push(Route::Home);
        });        
        
        html! {
            // Keyword: danger sets to red to match CUAVA theme
            <nav class="navbar navbar-expand-lg">
                <div class="container-fluid">
                    <a class="navbar-brand clickable" onclick={onclick}>{ "CUAVA Groundbase Web App" }</a>
                </div>
            </nav>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
}
