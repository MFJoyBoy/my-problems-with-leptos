use leptos::html::br;
use leptos::server;
use leptos::{
    IntoView, component, ev,
    html::{ElementChild, button, div, p},
    prelude::{
        ClassAttribute, Get, OnAttribute, ServerFnError, Suspense, SuspenseProps, ToChildren, Write,
    },
    reactive::signal::signal,
    server::Resource,
};

#[component]
pub fn ProblematicPage() -> impl IntoView {
    let (count, count_ser) = signal(0);
    let resource1 = Resource::new(move || count.get(), |input| async move { input * 2 });
    let resource2 = Resource::new_blocking(
        move || count.get(),
        |input| async move {
            wait_on_server().await.unwrap();
            input * 3
        },
    );
    div().class("bg-white h-full p-5").child((
        p().child("First one, Good luck"),
        br(),
        p().child(move || format!("count: {}", count.get())),
        Suspense(
            SuspenseProps::builder()
                .fallback(|| p().child("loading suspense 1 ..."))
                .children(ToChildren::to_children(move || {
                    p().child(move || format!("suspense data 1: {:?}", resource1.get()))
                }))
                .build(),
        ),
        Suspense(
            SuspenseProps::builder()
                .fallback(|| p().child("loading suspense 2 ..."))
                .children(ToChildren::to_children(move || {
                    p().child(move || format!("suspense data 2: {:?}", resource2.get()))
                }))
                .build(),
        ),
        button().child("button").on(ev::click, move |_| {
            *count_ser.write() += 1;
        }),
    ))
}

#[server]
async fn wait_on_server() -> Result<(), ServerFnError> {
    use std::time::Duration;
    use tokio::time::sleep;

    sleep(Duration::from_secs(2)).await;
    Ok(())
}
