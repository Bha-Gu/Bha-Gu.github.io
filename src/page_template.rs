use leptos::prelude::*;
use slug::slugify;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{
    Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit, window,
};

#[component]
pub fn SideBarPage(ids: Vec<String>, children: Children) -> impl IntoView {
    view! {
        <div class="page-layout">
            <main class="page-content">{children()}</main>

            <SideBar ids=ids />
        </div>
    }
}

#[component]
pub fn StaticPage(children: Children) -> impl IntoView {
    view! {
        <div class="page-layout">
            <main class="page-content">{children()}</main>

        </div>
    }
}

#[component]
fn SideBar(ids: Vec<String>) -> impl IntoView {
    let active = RwSignal::new(String::new());

    let ids_clone = ids.clone();

    Effect::new(move |_| {
        let callback = Closure::<dyn FnMut(js_sys::Array, IntersectionObserver)>::new({
            let active = active;

            move |entries: js_sys::Array, _| {
                for entry in entries.iter() {
                    let entry: IntersectionObserverEntry = entry.unchecked_into();

                    if entry.is_intersecting() {
                        let element: Element = entry.target().unchecked_into();
                        active.set(element.id());
                    }
                }
            }
        });

        let mut options = IntersectionObserverInit::new();
        options.set_root_margin("-80px 0px -70% 0px");
        options.set_threshold(&0.0.into());

        let observer =
            IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
                .unwrap();

        let document = window().unwrap().document().unwrap();

        for id in &ids_clone {
            let slug = slugify(id);

            if let Some(element) = document.get_element_by_id(&slug) {
                observer.observe(&element);
            }
        }

        callback.forget();
        std::mem::forget(observer);
    });

    let items = ids
        .into_iter()
        .map(|id| {
            let slug = slugify(id.clone());
            view! {
                <li class:active={let slug = slug.clone(); move || active.get() == slug} class="sidebar__link">
                    <a href=format!("#{slug}")>{id.clone()}</a>
                </li>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <aside class="sidebar">
            <div class="sidebar__header">
                <span class="sidebar__title">"On This Page"</span>
            </div>
            <hr />
            <div class="sidebar__section">{items}</div>
        </aside>
    }
}
