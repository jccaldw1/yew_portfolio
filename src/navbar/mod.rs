use yew::prelude::*;
pub mod navbar;

#[derive(Properties, PartialEq, Clone)]
pub struct navbar_page {
    pub page_name: String,
    pub page_link: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct navbar_props {
    pub pages: Vec<navbar_page>,
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
        }).collect();
}
