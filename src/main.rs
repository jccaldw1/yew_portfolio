use yew::prelude::*;

mod navbar;
use navbar::navbar_page as navbar_page;
use navbar::NavBar as NavBar;



#[function_component]
fn App() -> Html {
    let navbar_page0 = navbar_page {
        page_name: "Home".to_string(),
        page_link: "/home".to_string()
    };

    let navbar_page1 = navbar_page {
        page_name: "About".to_string(),
        page_link: "/about".to_string()
    };

    html! {
        <>
            <NavBar pages={vec![navbar_page0, navbar_page1]}/>
            <Home/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
