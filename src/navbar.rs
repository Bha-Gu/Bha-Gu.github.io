use crate::Breadcrumbs;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_use::{OnClickOutsideOptions, on_click_outside_with_options};
use web_sys::window;

#[component]
fn ThemeToggle() -> impl IntoView {
    let dark = RwSignal::new(true);
    Effect::new(move |_| {
        let theme = dark.get();

        if let Some(document) = window().and_then(|w| w.document())
            && let Some(html) = document.document_element()
        {
            match theme {
                false => {
                    let _ = html.set_attribute("data-theme", "light");
                }
                true => {
                    let _ = html.set_attribute("data-theme", "dark");
                }
            }
        }
    });
    view! {
        <form class="navbar__theme-switch">
            <label>
                <input
                    class="navbar__theme-switch"

                    type="checkbox"
                    bind:checked=dark
                />
            </label>
        </form>
    }
}

#[component]
pub fn NavBar(navbar_list: Vec<&'static str>) -> impl IntoView {
    let target = NodeRef::<Div>::new();
    let (is_open, set_open) = signal(false);
    let _ = on_click_outside_with_options(
        target,
        move |_| set_open.set(false),
        OnClickOutsideOptions::default().ignore([".navbar__icon"]),
    );
    
    let navbar_items = navbar_list
        .into_iter()
        .map(|item| {
            view! {
                <li class="navbar__item">
                    <A
                        href=match item {
                            "Home" => String::from("/"),
                            item => format!("/{item}"),
                        }
                        attr:class="navbar__link"
                    >
                        {item}
                    </A>
                </li>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <nav class="navbar">
            <div class="navbar__container">
                <ul class="navbar__menu_base">
                    <p class="navbar__logo">"Bha-Gu"</p>
                    <Breadcrumbs />
                </ul>

                <ul class="navbar__menu_base">
                    <div node_ref=target class:open=move || is_open.get() class="navbar__menu">
                        {navbar_items}
                    </div>

                    <div
                        on:click=move |_| set_open.update(|val| *val = !*val)
                        class:open=move || is_open.get()
                        class="navbar__icon"
                    >
                        <span></span>
                        <span></span>
                        <span></span>
                    </div>
                    <li>
                        <ThemeToggle />
                    </li>
                </ul>

            </div>
        </nav>
    }
}


