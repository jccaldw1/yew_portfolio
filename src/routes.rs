use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{self, SignUp};
use pages::Home as Home;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/resume")]
    Resume,
    #[at("/signup")]
    SignUp
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => pages::Home(),
        Route::About => pages::About(),
        Route::Resume => pages::Resume(),
        Route::SignUp => pages::SignUp(),
    }
}
