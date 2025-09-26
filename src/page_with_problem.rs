use leptos::ev;
use leptos::logging::log;
use leptos::prelude::{RwSignal, ToChildren, Transition, TransitionProps, Update, use_context};
use leptos::{
    IntoView, component,
    html::{ElementChild, button, p},
    prelude::{Get, OnAttribute},
    server::Resource,
};

use crate::app::CurrentUser;

#[component]
pub fn ProblematicPage() -> impl IntoView {
    let user = use_context::<RwSignal<CurrentUser>>();
    let logged_in = Resource::new_blocking(
        move || user.get(),
        |input| async {
            log!("{:?}", input);
            let logged_in = match input {
                Some(user) => match user.me {
                    Some(_) => true,
                    None => false,
                },
                None => false,
            };
            logged_in
        },
    );
    (
        Transition(
            TransitionProps::builder()
                .children(ToChildren::to_children(move || {
                    p().child(("logged in state: ", move || logged_in.get()))
                }))
                .build(),
        ),
        button().child("update context").on(ev::click, move |_| {
            log!("update context click");
            let current_user = use_context::<RwSignal<CurrentUser>>().unwrap();
            current_user.update(|input| {
                input.me = None;
            });
            let current_user = use_context::<RwSignal<CurrentUser>>();
            log!(
                "inside callback, context value after update: {:?}",
                current_user.get()
            )
        }),
    )
}
