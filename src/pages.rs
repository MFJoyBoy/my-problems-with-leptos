use leptos::{
    IntoView, component,
    html::{ElementChild, a, div, p},
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
    (
        p().child("page two"),
        a().child("go to three").href("/parent/three"),
    )
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
                    path: path!(""),
                    condition: || Some(false),
                    fallback: ViewFn::from(|| p().child("fallback view")),
                    ssr: SsrMode::OutOfOrder,
                    redirect_path: || "two",
                    view: || (),
                }),
                ProtectedRoute(ProtectedRouteProps {
                    path: path!("one"),
                    condition: || Some(false),
                    fallback: ViewFn::from(|| p().child("fallback view")),
                    ssr: SsrMode::OutOfOrder,
                    redirect_path: || "two",
                    view: PageOne,
                }),
                ProtectedRoute(ProtectedRouteProps {
                    path: path!("two"),
                    condition: || Some(true),
                    fallback: ViewFn::from(|| p().child("fallback view")),
                    ssr: SsrMode::OutOfOrder,
                    redirect_path: || "three",
                    view: PageTwo,
                }),
                ProtectedRoute(ProtectedRouteProps {
                    path: path!("three"),
                    condition: || Some(false),
                    fallback: ViewFn::from(|| p().child("fallback view")),
                    ssr: SsrMode::OutOfOrder,
                    redirect_path: || "",
                    view: PageThree,
                }),
                ProtectedRoute(ProtectedRouteProps {
                    path: path!("four"),
                    condition: || Some(false),
                    fallback: ViewFn::from(|| p().child("fallback view")),
                    ssr: SsrMode::OutOfOrder,
                    redirect_path: || "five",
                    view: PageFour,
                }),
                ProtectedRoute(ProtectedRouteProps {
                    path: path!("five"),
                    condition: || Some(true),
                    fallback: ViewFn::from(|| p().child("fallback view")),
                    ssr: SsrMode::OutOfOrder,
                    redirect_path: || "",
                    view: PageFive,
                }),
            )
        }),
    })
}
