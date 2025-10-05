use leptos::logging::log;
use leptos::prelude::{Effect, RwSignal, Update};
use leptos::server;
use leptos::{
    IntoView, component, ev,
    html::{ElementChild, button, p},
    prelude::{Get, OnAttribute, ServerFnError, Suspense, SuspenseProps, ToChildren},
    server::Resource,
};

#[component]
pub fn ProblematicPage() -> impl IntoView {
    let number_signal = RwSignal::new(0);
    let number_resource = Resource::new(
        move || number_signal.get(),
        move |input| async move {
            let response = wait_on_server(input).await.unwrap();
            response
        },
    );
    Effect::new(move || {
        log!("signal number -> {:?}", number_signal.get());
        log!("resource number -> {:?}", number_resource.get());
    });
    (
        Suspense(
            SuspenseProps::builder()
                .children(ToChildren::to_children(move || {
                    let number = number_resource.get();
                    let text = format!("resource number: {:?}", number);
                    p().child(text)
                }))
                .fallback(|| p().child("waiting ..."))
                .build(),
        ),
        button().child("button").on(ev::click, move |_| {
            number_signal.update(|value| {
                *value += 1;
            });
        }),
    )
}

#[server]
async fn wait_on_server(number: i32) -> Result<i32, ServerFnError> {
    use std::time::Duration;
    tokio::time::sleep(Duration::from_secs(2)).await;
    Ok(number * 2)
}
