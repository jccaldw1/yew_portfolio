use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct navbar_page {
    pub page_name: String,
    pub page_link: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct navbar_props {
    pages: Vec<navbar_page>,
}

#[function_component]
fn NavBar(props: &navbar_props) -> Html {
    let pages = props.pages.clone();
    // pages.iter().map();
    html! {
        <>
            {"This is a navbar with link: "}{props.pages[1].page_link.clone()}
        </>
    }
}

// Home page
#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

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
            <div>
                <button {onclick}>{ "+1" }</button>
                <p>{ *counter }</p>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
