use leptos::ev::scroll;
use leptos::html;
use leptos::leptos_dom::helpers::window_event_listener;
use leptos::prelude::*;
use slug::slugify;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{
    Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit, window,
};

#[component]
pub fn SideBarPage(ids: Vec<String>, children: Children) -> impl IntoView {
    let content_ref = NodeRef::<html::Main>::new();
    let (progress, set_progress) = signal(0.0);

    let (scroll_tick, set_scroll_tick) = signal(());

    let listener = window_event_listener(scroll, move |_| {
        set_scroll_tick.set(());
    });

    Effect::new(move |_| {
        // Re-run whenever a scroll event occurs.
        scroll_tick.get();

        let Some(content) = content_ref.get() else {
            return;
        };

        let rect = content.get_bounding_client_rect();

        let Some(window) = web_sys::window() else {
            return;
        };

        let Ok(height) = window.inner_height() else {
            return;
        };

        let Some(viewport) = height.as_f64() else {
            return;
        };

        let denom = rect.height() - viewport;

        let progress = if denom <= 0.0 {
            if rect.top() + rect.height() <= viewport {
                1.0
            } else {
                0.0
            }
        } else {
            (-rect.top() / denom).clamp(0.0, 1.0)
        };

        set_progress.set(progress);
    });
    view! {
        <div class="page-layout">
            <main node_ref=content_ref class="page-content">
                {children()}
            </main>

            <SideBar ids=ids progress=progress />

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
fn SideBar(
    ids: Vec<String>,
    #[prop(default = signal(0.0).0)] progress: ReadSignal<f64>,
) -> impl IntoView {
    let active = RwSignal::new(String::new());

    let ids_clone = ids.clone();

    Effect::new(move |_| {
        let callback = Closure::<dyn FnMut(js_sys::Array, IntersectionObserver)>::new({
            let active = active;

            move |entries: js_sys::Array, _| {
                for entry in entries.iter() {
                    let entry: IntersectionObserverEntry = entry.into();

                    if entry.is_intersecting() {
                        let element: Element = entry.target().into();
                        active.set(element.id());
                    }
                }
            }
        });

        let options = IntersectionObserverInit::new();
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
                <li
                    class:active={
                        let slug = slug.clone();
                        move || active.get() == slug
                    }
                    class="sidebar__link"
                >
                    <a href=format!("#{slug}")>{id.clone()}</a>
                </li>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <aside class="sidebar" style=move || format!("--progress: {}%;", progress.get() * 100.0)>

            <div class="sidebar__header">
                <span class="sidebar__title">"On This Page"</span>

            </div>

            <hr />
            <div class="sidebar__section">{items}</div>
        </aside>
    }
}
