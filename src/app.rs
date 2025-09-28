use leptos::form::FromFormData;
use leptos::html::{fieldset, input};
use leptos::prelude::{AddAnyAttr, CustomAttribute, Get};
use leptos::server;
use leptos::{
    IntoView, component, ev,
    ev::on,
    form::{ActionForm, ActionFormProps},
    html::ElementChild,
    logging::log,
    prelude::{Effect, ServerFnError, ToChildren},
    server::ServerAction,
};
use leptos_meta::provide_meta_context;
use serde::{Deserialize, Serialize};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let sign_up_action = ServerAction::<CreateNew>::new();
    Effect::new(move || {
        let response = sign_up_action.value().get();
        match response {
            Some(response) => {
                log!("{response:?}  ");
            }
            None => {
                log!("response is none");
            }
        }
    });
    ActionForm(
        ActionFormProps::builder()
            .action(sign_up_action)
            .children(ToChildren::to_children(move || {
                (
                    input().name("new[one]").placeholder("one ..."),
                    fieldset().child((input().name("new[two]").placeholder("two ..."),)),
                    input().attr("type", "submit"),
                )
            }))
            .build(),
    )
    .add_any_attr(on(ev::capture(ev::submit), |event| {
        //TODO: comment three line below and and correct data would be sent to browser and deserialize there
        let form_data = NewCreate::from_event(&event);
        log!("{:?}", form_data);
        event.prevent_default();
    }))
}

#[server]
async fn create_new(new: NewCreate) -> Result<(), ServerFnError> {
    log!("{:?}", new);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NewCreate {
    one: String,
    two: String,
}
