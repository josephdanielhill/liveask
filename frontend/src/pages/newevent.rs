use shared::{CreateEventErrors, EventInfo, ValidationError};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{fetch, routes::Route};

use super::event::BASE_API;

pub struct NewEvent {
    name: String,
    desc: String,
    email: String,
    name_ref: NodeRef,
    errors: CreateEventErrors,
}

#[derive(Debug)]
pub enum Input {
    Name,
    Email,
    Desc,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct NewEventProps;

pub enum Msg {
    Create,
    CreatedResult(Option<EventInfo>),
    InputChange(Input, InputEvent),
}
impl Component for NewEvent {
    type Message = Msg;
    type Properties = NewEventProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            name: String::new(),
            desc: String::new(),
            email: String::new(),
            name_ref: NodeRef::default(),
            errors: CreateEventErrors::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Create => {
                let name = self.name.clone();
                let desc = self.desc.clone();
                let email = self.email.clone();
                ctx.link().send_future(async move {
                    let res = fetch::create_event(BASE_API, name, desc, email).await;

                    match res {
                        Ok(e) => Msg::CreatedResult(Some(e)),
                        Err(e) => {
                            log::error!("create error: {}", e);
                            Msg::CreatedResult(None)
                        }
                    }
                });
                false
            }

            Msg::CreatedResult(event) => match event {
                Some(event) => {
                    ctx.link().history().unwrap().push(Route::EventMod {
                        id: event.tokens.public_token,
                        secret: event.tokens.moderator_token.unwrap(),
                    });
                    false
                }
                None => {
                    log::error!("no event created");
                    true
                }
            },

            Msg::InputChange(input, c) => {
                match input {
                    Input::Name => {
                        let e = self.name_ref.cast::<Element>().unwrap();
                        let e: HtmlInputElement = e.dyn_into().unwrap();

                        self.name = e.value();

                        self.errors.check(&self.name, &self.desc);
                    }
                    Input::Email => {
                        let target: HtmlInputElement = c.target_dyn_into().unwrap();
                        self.email = target.value()
                    }
                    Input::Desc => {
                        let target: HtmlTextAreaElement = c.target_dyn_into().unwrap();
                        self.desc = target.value();

                        self.errors.check(&self.name, &self.desc);
                    }
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="newevent-bg">
                <div class="title">
                    {"Create Event"}
                </div>
                <div class="form">
                    <div class="newevent">
                        <div class="input-box">
                            <input
                                ref={self.name_ref.clone()}
                                type="text"
                                name="eventname"
                                placeholder="event name"
                                value={self.name.clone()}
                                maxlength="30"
                                // autocomplete="off"
                                required=true
                                oninput={ctx.link().callback(|input| Msg::InputChange(Input::Name,input))}/>
                        </div>
                        <div hidden={self.errors.name.is_none()} class="invalid">
                            {self.name_error().unwrap_or_default()}
                        </div>
                        <div class="input-box">
                            <input
                                type="email"
                                name="mail"
                                placeholder="email (optional)"
                                value={self.email.clone()}
                                maxlength="100"
                                oninput={ctx.link().callback(|input| Msg::InputChange(Input::Email,input))}/>
                        </div>
                        <div class="input-box">
                            <textarea
                                id="input-desc"
                                name="desc"
                                placeholder="event description"
                                value={self.desc.clone()}
                                // mintrimlength="10"
                                maxlength="1000"
                                required=true
                                oninput={ctx.link().callback(|input| Msg::InputChange(Input::Desc,input))}>
                            </textarea>
                        </div>
                        <div hidden={self.errors.desc.is_none()} class="invalid">
                            {self.desc_error().unwrap_or_default()}
                        </div>
                    </div>
                    <button
                        class="button-finish"
                        disabled={!self.can_create()}
                        onclick={ctx.link().callback(|_| Msg::Create)}>
                        {"finish"}
                    </button>
                </div>
            </div>
        }
    }
}

impl NewEvent {
    fn can_create(&self) -> bool {
        !self.errors.has_any() && !self.name.is_empty() && !self.desc.is_empty()
    }

    fn desc_error(&self) -> Option<String> {
        match self.errors.desc {
            Some(ValidationError::Empty) => Some("Description cannot be empty".to_string()),
            Some(ValidationError::MinLength(len, max)) => Some(format!(
                "Description must be at least {} characters long. ({})",
                max, len
            )),
            Some(ValidationError::MaxLength(_, max)) => Some(format!(
                "Description cannot be longer than {} characters.",
                max
            )),
            Some(_) => Some("unknown error".to_string()),
            None => None,
        }
    }

    fn name_error(&self) -> Option<String> {
        match self.errors.name {
            Some(ValidationError::Empty) => Some("Name is required.".to_string()),
            Some(ValidationError::MinLength(len, max)) => Some(format!(
                "Name must be at least {} characters long. ({})",
                max, len
            )),
            Some(ValidationError::MaxLength(_, max)) => {
                Some(format!("Name cannot be longer than {} characters.", max))
            }
            Some(ValidationError::MaxWords(_, max)) => {
                Some(format!("Name must not contain more than {} words.", max))
            }
            None => None,
        }
    }
}
