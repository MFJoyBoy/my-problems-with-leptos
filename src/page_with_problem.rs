use leptos::html::hr;
use leptos::logging::log;
use leptos::prelude::{GetUntracked, Set, Update};
use leptos::server::codee::string::FromToStringCodec;
use leptos::{
    IntoView, component, ev,
    html::{ElementChild, button, div, p},
    prelude::{Get, OnAttribute},
};
use leptos_meta::provide_meta_context;
use leptos_use::use_cookie;

#[component]
pub fn ProblematicPage() -> impl IntoView {
    provide_meta_context();
    let (cookie, cookie_set) = use_cookie::<i32, FromToStringCodec>("number");
    match cookie.get_untracked() {
        Some(value) => {
            log!("initial cookie value: {}", value);
        }
        None => {
            log!("initial cookie value is none");
            cookie_set.set(Some(0));
            log!("initial cookie after set is: {:?}", cookie.get_untracked());
        }
    };

    div().child((One(), hr(), Two()))
}

#[component]
fn One() -> impl IntoView {
    let (cookie, cookie_set) = use_cookie::<i32, FromToStringCodec>("number");
    (
        p().child("component one"),
        p().child(("the cookie value: ", move || cookie.get())),
        button().child("button").on(ev::click, move |_| {
            cookie_set.update(|value| {
                let number = *value.as_ref().unwrap() + 1;
                *value = Some(number);
            });
        }),
    )
}

#[component]
fn Two() -> impl IntoView {
    let (cookie, cookie_set) = use_cookie::<i32, FromToStringCodec>("number");
    (
        p().child("component two"),
        p().child(("the cookie value: ", move || cookie.get())),
        button().child("button").on(ev::click, move |_| {
            cookie_set.update(|value| {
                let number = *value.as_ref().unwrap() + 1;
                *value = Some(number);
            });
        }),
    )
}
