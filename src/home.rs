use crate::page_template::SideBarPage;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let ids = vec!["Base".into(), "About".into()];
    view! {
        <SideBarPage ids=ids>
            <h1 id="Base">"Bhaumikaditya Guleria"</h1>

            <h1 id="About">"Bhaumikaditya Guleria"</h1>

        </SideBarPage>
    }
}
