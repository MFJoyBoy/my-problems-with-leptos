use leptos::{
    IntoView, component,
    html::{ElementChild, a},
};

#[component]
pub fn ProblematicPage() -> impl IntoView {
    a().child("navigate").href("/parent")
}
