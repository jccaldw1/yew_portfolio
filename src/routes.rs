use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/resume")]
    Resume,
    #[at("/signup")]
    SignUp
}

#[function_component]
pub fn NavBar(props: &navbar_props) -> Html {
    let pages = props.pages.clone();
    return pages
        .iter()
        .map(|page| {
            html!{
                <a role="button" href={page.page_link.to_string()}>
                    {page.page_name.to_string()}
                </a>
            }
        })
        .collect();
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => pages::Home(),
        Route::About => pages::About(),
        Route::Resume => pages::Resume(),
        Route::SignUp => pages::SignUp(),
    }
}

