use yew_router::prelude::*;

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

//TODO: Make function components for Home page, About, Resume, and SignUp

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => ,
        Route::About => ,
        Route::Resume => ,
        Route::SignUp => ,
    }
}
