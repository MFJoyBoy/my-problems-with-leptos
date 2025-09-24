use leptos::{
    IntoView, component,
    html::{ElementChild, div, header, hr, main, p},
    prelude::{ToChildren, TypedChildren},
};
use leptos_router::{
    Lazy, SsrMode,
    components::{
        A, AProps, Route, RouteChildren, RouteProps, Router, RouterProps, Routes, RoutesProps,
    },
    path,
};

use crate::pages::{PageOne, PageThree, PageTwo};

///view component that render and handler routes, all routes of web application defined here
#[component]
pub fn RootsOfProblems() -> impl IntoView {
    let routes_property = RoutesProps::builder()
        .children(RouteChildren::to_children(|| {
            (
                Route(RouteProps {
                    path: path!("/one"),
                    view: Lazy::<PageOne>::new(),
                    ssr: SsrMode::OutOfOrder,
                }),
                Route(RouteProps {
                    path: path!("/two"),
                    view: Lazy::<PageTwo>::new(),
                    ssr: SsrMode::OutOfOrder,
                }),
                Route(RouteProps {
                    path: path!("/three"),
                    view: Lazy::<PageThree>::new(),
                    ssr: SsrMode::OutOfOrder,
                }),
            )
        }))
        .fallback(|| p().child("nothing here, look other places for problems"))
        .build();
    Router(
        RouterProps::builder()
            .children(TypedChildren::to_children(|| {
                div()
                    .child(
                        header().child((
                            A(AProps::builder()
                                .href("one")
                                .children(ToChildren::to_children(|| p().child("page one")))
                                .build()),
                            A(AProps::builder()
                                .href("two")
                                .children(ToChildren::to_children(|| p().child("page two")))
                                .build()),
                            A(AProps::builder()
                                .href("three")
                                .children(ToChildren::to_children(|| p().child("page three")))
                                .build()),
                        )),
                    )
                    .child(hr())
                    .child(main().child(Routes(routes_property)))
            }))
            .build(),
    )
}
