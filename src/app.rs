use leptos::{
    IntoView, component,
    html::{ElementChild, p},
    prelude::{RwSignal, provide_context},
};
use leptos_meta::provide_meta_context;

use crate::problematic_router::RootsOfProblems;

#[component]
pub fn App() -> impl IntoView {
    let current_user = RwSignal::new(CurrentUser { me: None });
    provide_context(current_user);
    provide_meta_context();
    (p().child("app page"), RootsOfProblems())
}

#[derive(Clone)]
pub struct CurrentUser {
    pub me: Option<String>,
}
