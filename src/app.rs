use crate::problematic_router::RootsOfProblems;
use leptos::prelude::Effect;
use leptos::reactive::spawn_local;
use leptos::server;
use leptos::{
    IntoView, component,
    html::{ElementChild, p},
    logging::log,
    prelude::{RwSignal, ServerFnError, Update, provide_context},
    server::OnceResource,
};
use leptos_meta::provide_meta_context;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_current_user_context();
    (p().child("app page"), RootsOfProblems())
}

#[derive(Clone, Debug, PartialEq)]
pub struct CurrentUser {
    pub me: Option<String>,
}

fn provide_current_user_context() {
    let current_user = RwSignal::new(CurrentUser { me: None });
    let resource_date = OnceResource::new_blocking(async move {
        let response = get_current_user_private_data().await;
        response
    });
    Effect::new(move || {
        spawn_local(async move {
            let response = resource_date.await;
            match response {
                Ok(value) => {
                    current_user.update(|input| {
                        input.me = Some(value);
                    });
                }
                Err(error) => {
                    log!("{error:?}");
                }
            }
        });
    });
    provide_context(current_user);
}

#[server]
async fn get_current_user_private_data() -> Result<String, ServerFnError> {
    Ok(String::from("The User"))
}
