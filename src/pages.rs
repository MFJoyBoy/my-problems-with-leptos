use leptos::{
    IntoView, component,
    html::{ElementChild, div, p},
    prelude::{ToChildren, ViewFn},
};
use leptos_router::{
    MatchNestedRoutes, SsrMode,
    components::{
        Outlet, ProtectedParentRoute, ProtectedParentRouteProps, ProtectedRoute,
        ProtectedRouteProps, RouteChildren,
    },
    path,
};

use crate::page_with_problem;

#[component]
pub fn ParentPage() -> impl IntoView {
    (p().child("parent page"), div().child(Outlet()))
}

#[component]
pub fn PageOne() -> impl IntoView {
    p().child("page one")
}

#[component]
pub fn PageTwo() -> impl IntoView {
    p().child("page two")
}

#[component]
pub fn PageThree() -> impl IntoView {
    p().child("page three")
}

#[component]
pub fn PageFour() -> impl IntoView {
    p().child("page four")
}

#[component]
pub fn PageFive() -> impl IntoView {
    p().child("page five")
}

pub fn routes_of_pages() -> impl MatchNestedRoutes + Clone {
    ProtectedParentRoute(ProtectedParentRouteProps {
        path: path!("/parent"),
        condition: || Some(true),
        fallback: ViewFn::from(|| ()),
        ssr: SsrMode::OutOfOrder,
        redirect_path: || "/",
        view: ParentPage,
        children: RouteChildren::to_children(|| {
            (
                ProtectedRoute(ProtectedRouteProps {
                    path: path!("/"),
                    condition: || Some(true),
                    fallback: ViewFn::from(|| ()),
                    ssr: SsrMode::OutOfOrder,
                    redirect_path: || "one",
                    view: page_with_problem::ProblematicPage,
                }),
                ProtectedRoute(ProtectedRouteProps {
                    path: path!("/one"),
                    condition: || Some(true),
                    fallback: ViewFn::from(|| ()),
                    ssr: SsrMode::OutOfOrder,
                    redirect_path: || "two",
                    view: page_with_problem::ProblematicPage,
                }),
                ProtectedRoute(ProtectedRouteProps {
                    path: path!("two"),
                    condition: || Some(false),
                    fallback: ViewFn::from(|| ()),
                    ssr: SsrMode::OutOfOrder,
                    redirect_path: || "three",
                    view: page_with_problem::ProblematicPage,
                }),
                ProtectedRoute(ProtectedRouteProps {
                    path: path!("three"),
                    condition: || Some(false),
                    fallback: ViewFn::from(|| ()),
                    ssr: SsrMode::OutOfOrder,
                    redirect_path: || "four",
                    view: page_with_problem::ProblematicPage,
                }),
                ProtectedRoute(ProtectedRouteProps {
                    path: path!("four"),
                    condition: || Some(false),
                    fallback: ViewFn::from(|| ()),
                    ssr: SsrMode::OutOfOrder,
                    redirect_path: || "fine",
                    view: page_with_problem::ProblematicPage,
                }),
                ProtectedRoute(ProtectedRouteProps {
                    path: path!("five"),
                    condition: || Some(true),
                    fallback: ViewFn::from(|| ()),
                    ssr: SsrMode::OutOfOrder,
                    redirect_path: || "",
                    view: page_with_problem::ProblematicPage,
                }),
            )
        }),
    })
}
