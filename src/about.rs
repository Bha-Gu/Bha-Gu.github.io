use crate::page_template::StaticPage;
use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <StaticPage>
            <h1>"About Me"</h1>

        </StaticPage>
    }
}
