use webapp_macro::*;
use yew::prelude::*;
use yew_router::{history::History, prelude::RouterScopeExt};
use wasm_bindgen::JsCast;
use web_sys::{HtmlSelectElement, HtmlInputElement};
use std::rc::Rc;
use std::cell::RefCell;
use super::*;


webapp_macro!("./commands/example.json");