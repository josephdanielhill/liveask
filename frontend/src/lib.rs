#![deny(
    warnings,
    unused_imports,
    unused_must_use,
    unused_variables,
    unused_mut,
    dead_code,
    clippy::expect_used
)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::dbg_macro,
    clippy::unwrap_used,
    clippy::panic,
    clippy::needless_update,
    clippy::match_like_matches_macro,
    clippy::from_over_into,
    clippy::useless_conversion,
    clippy::float_cmp_const,
    clippy::lossy_float_literal,
    clippy::string_to_string,
    clippy::unneeded_field_pattern,
    clippy::verbose_file_reads
)]
#![allow(
    clippy::use_self,
    clippy::module_name_repetitions,
    clippy::let_unit_value,
    clippy::let_underscore_untyped
)]
mod components;
mod environment;
mod fetch;
mod global_events;
mod local_cache;
mod pages;
mod routes;
mod tracking;

use events::{EventBridge, Events};
use global_events::GlobalEvent;
use pages::AdminLogin;
use routes::Route;
use shared::GetEventResponse;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::{prelude::Dispatch, store::Store};

use crate::{
    components::IconBar,
    pages::{Event, Home, NewEvent, Print, Privacy},
};

pub const VERSION_STR: &str = "2.3.12";
pub const GIT_BRANCH: &str = env!("VERGEN_GIT_BRANCH");

#[derive(Default, Clone, Eq, PartialEq, Store)]
pub struct State {
    pub event: Option<GetEventResponse>,
    pub new_question: Option<i64>,
    pub admin: bool,
}

impl State {
    #[must_use]
    pub const fn set_new_question(mut self, v: Option<i64>) -> Self {
        self.new_question = v;
        self
    }
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn set_event(mut self, v: Option<GetEventResponse>) -> Self {
        self.event = v;
        self
    }
    #[must_use]
    pub const fn set_admin(mut self, v: bool) -> Self {
        self.admin = v;
        self
    }
}

pub enum Msg {
    State(Rc<State>),
    GlobalEvent(GlobalEvent),
}

pub struct AppRoot {
    connected: bool,
    events: EventBridge<GlobalEvent>,
    state: Rc<State>,
    _dispatch: Dispatch<State>,
}
impl Component for AppRoot {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut context = Events::<GlobalEvent>::default();

        let events = context.subscribe(ctx.link().callback(Msg::GlobalEvent));

        Self {
            _dispatch: Dispatch::<State>::subscribe(ctx.link().callback(Msg::State)),
            state: Rc::default(),
            connected: true,
            events,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::State(state) => {
                self.state = state;
                false
            }
            Msg::GlobalEvent(e) => match e {
                GlobalEvent::SocketStatus { connected, .. } => {
                    self.connected = connected;
                    true
                }
                _ => false,
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <div class="app-host">
                    <ContextProvider<Events<GlobalEvent>> context={self.events.clone()}>
                    <div class={classes!("main",not(self.connected).then_some("offline"))}>
                        <IconBar/>

                        <div class="router">
                            <Switch<Route> render={Switch::render(switch)} />
                        </div>
                    </div>
                    </ContextProvider<Events<GlobalEvent>>>
                </div>
            </BrowserRouter>
        }
    }
}

#[must_use]
pub const fn not(b: bool) -> bool {
    !b
}

fn switch(switch: &Route) -> Html {
    match switch {
        Route::Event { id } => {
            html! { <Event id={id.clone()} /> }
        }
        Route::Print { id } => {
            html! { <Print id={id.clone()} /> }
        }
        Route::EventMod { id, secret } => {
            html! { <Event id={id.clone()} secret={secret.clone()} /> }
        }
        Route::NewEvent => {
            html! { <NewEvent /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::Privacy => {
            html! { <Privacy /> }
        }
        Route::Login => {
            html! { <AdminLogin /> }
        }
    }
}
