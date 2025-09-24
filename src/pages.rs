use leptos::html::{ElementChild, p};
use leptos::prelude::{AnyView, IntoAny};
use leptos_router::{LazyRoute, lazy_route};

pub struct PageOne;

#[lazy_route]
impl LazyRoute for PageOne {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        p().child("this is page one").into_any()
    }
}

// ------------------------------------------------------

pub struct PageTwo;

#[lazy_route]
impl LazyRoute for PageTwo {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        p().child("this is page two").into_any()
    }
}

// ------------------------------------------------------

pub struct PageThree;

#[lazy_route]
impl LazyRoute for PageThree {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        p().child("this is page three").into_any()
    }
}
