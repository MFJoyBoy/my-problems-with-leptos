use leptos::{
    IntoView, component,
    html::{ElementChild, p},
};
use leptos_meta::provide_meta_context;

use crate::problematic_router::RootsOfProblems;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    (p().child("app page"), RootsOfProblems())
}
