#[derive(Properties, PartialEq, Clone)]
pub struct navbar_page {
    page_name: String,
    page_link: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct navbar_props {
    pages: Vec<navbar_page>,
}

#[function_component]
fn NavBar(props: &navbar_props) -> Html {
    let pages = props.pages.clone();
    let htmlMap = pages
        .iter()
        .map(|page| {
            html!{
                <a role="button" href={page.page_link.to_string()}>
                    {page.page_name.to_string()}
                </a>
            }
        }).collect();

    return htmlMap;
}
