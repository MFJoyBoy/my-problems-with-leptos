use leptos::prelude::Get;
use leptos::server;
use leptos::server::OnceResource;
use leptos::{
    IntoView, component,
    html::{ElementChild, div, p},
    prelude::{ClassAttribute, ServerFnError, Suspense, SuspenseProps, ToChildren},
};

#[component]
pub fn PageOne() -> impl IntoView {
    let resource1 = OnceResource::new(async move {
        wait_on_server(2).await.unwrap();
        2
    });
    let resource2 = OnceResource::new(async move {
        wait_on_server(4).await.unwrap();
        4
    });
    div().class("bg-white h-full p-5").child((
        p().child("samples page"),
        Suspense(
            SuspenseProps::builder()
                .fallback(|| p().child("loading suspense 1 ..."))
                .children(ToChildren::to_children(move || {
                    p().child(move || format!("suspense 1 loaded, data: {:?}", resource1.get()))
                }))
                .build(),
        ),
        Suspense(
            SuspenseProps::builder()
                .fallback(|| p().child("loading suspense 2 ..."))
                .children(ToChildren::to_children(move || {
                    p().child(move || format!("suspense 2 loaded, data: {:?}", resource2.get()))
                }))
                .build(),
        ),
    ))
}

#[server]
async fn wait_on_server(duration: u64) -> Result<(), ServerFnError> {
    use std::time::Duration;
    use tokio::time::sleep;

    sleep(Duration::from_secs(duration)).await;
    Ok(())
}
