use webapp_macro::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::{Switch, BrowserRouter, history::History, prelude::RouterScopeExt, Routable};
use wasm_bindgen::JsCast;
use web_sys::{HtmlSelectElement, HtmlInputElement};
use std::rc::Rc;
use std::cell::RefCell;
use super::*;
use std::borrow::BorrowMut;
// use std::fmt::Error;
use std::sync::Mutex;
use lazy_static::*;
use wasm_bindgen_futures::spawn_local;
use serde_json::*;

use strum_macros::*;
use strum::*;
use serde::*;

use std::net::UdpSocket;
use wasm_bindgen::closure::Closure;

use std::ops::DerefMut;

webapp_macro!("./commands/example.json");