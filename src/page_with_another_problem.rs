use leptos::server;
use leptos::{
    IntoView, component, ev,
    html::{ElementChild, button, div, p},
    prelude::{
        ClassAttribute, Get, OnAttribute, ServerFnError, Suspense, SuspenseProps, ToChildren,
        Write, signal,
    },
    server::{LocalResource, OnceResource},
};

#[component]
pub fn SecondProblem() -> impl IntoView {
    let (count, count_ser) = signal(1);
    let resource1 = OnceResource::new(async move {
        wait_on_server(2).await.unwrap();
        2
    });
    let resource2 = LocalResource::new(move || async move {
        wait_on_server(4).await.unwrap();
        count.get() * 3
    });
    div().class("bg-white h-full p-5").child((
        p().child("samples page"),
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
async fn wait_on_server(duration: u64) -> Result<(), ServerFnError> {
    use std::time::Duration;
    use tokio::time::sleep;

    sleep(Duration::from_secs(duration)).await;
    Ok(())
}
