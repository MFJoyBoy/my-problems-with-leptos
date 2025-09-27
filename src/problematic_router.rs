use leptos::{
    IntoView, component,
    html::{ElementChild, div, header, hr, main, p},
    prelude::{ClassAttribute, ToChildren, TypedChildren},
};
use leptos_router::{
    SsrMode,
    components::{
        A, AProps, Route, RouteChildren, RouteProps, Router, RouterProps, Routes, RoutesProps,
    },
    path,
};

use crate::{page_with_problem::ProblematicPage, pages::routes_of_pages};

///view component that render and handler routes, all routes of web application defined here
#[component]
pub fn RootsOfProblems() -> impl IntoView {
    let routes_property = RoutesProps::builder()
        .children(RouteChildren::to_children(|| {
            (
                Route(RouteProps {
                    path: path!("/problem_one"),
                    view: ProblematicPage,
                    ssr: SsrMode::OutOfOrder,
                }),
                routes_of_pages(),
            )
        }))
        .fallback(|| p().child("nothing here, look other places for problems"))
        .build();
    Router(
        RouterProps::builder()
            .children(TypedChildren::to_children(|| {
                div()
                    .class("h-screen w-screen flex gap-5 p-5 bg-faze-light-gray")
                    .child(
                        header()
                            .class("rounded-xl overflow-hidden min-w-fit")
                            .child(A(AProps::builder()
                                .href("problem_one")
                                .children(ToChildren::to_children(|| p().child("problem one")))
                                .build())),
                    )
                    .child(hr())
                    .child(
                        main()
                            .class("grow rounded-xl overflow-hidden")
                            .child(Routes(routes_property)),
                    )
            }))
            .build(),
    )
}
