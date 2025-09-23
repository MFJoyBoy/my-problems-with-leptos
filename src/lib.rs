use crate::app::App;
use leptos::prelude::*;
use leptos_meta::MetaTags;

pub mod app;
pub mod page_with_problem;
pub mod problematic_router;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}

/// shell component used as initial html template, app component mount into body element here
/// this is component used to crate initial page on ssr
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>

            <body>
                <App/>
            </body>
        </html>
    }
}
