use crate::Breadcrumbs;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_use::{OnClickOutsideOptions, on_click_outside_with_options};
use web_sys::window;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::MediaQueryList;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Theme {
    Light,
    System,
    Dark,
}



fn system_theme() -> Theme {
    let dark = window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|media| media.matches())
        .unwrap_or(false);

    if dark {
        Theme::Dark
    } else {
        Theme::Light
    }
}
#[component]
fn ThemeToggle() -> impl IntoView {
    let dark = RwSignal::new(Theme::System);
    Effect::new(move |_| {
        let theme = match dark.get() {
            
        Theme::System => system_theme(),
        theme => theme,        } ;

        if let Some(document) = window().and_then(|w| w.document())
            && let Some(html) = document.document_element()
        {
            match theme {
                Theme::Dark => {
                    let _ = html.set_attribute("data-theme", "dark");
                }
                Theme::Light => {
                    let _ = html.set_attribute("data-theme", "light");
                }
                Theme::System => {
                    let _ = html.set_attribute("data-theme", "system");
                }
            }
        }
    });
    view! {
        <div class="navbar__theme-switcher">
            <label aria-label="Toggle Theme Light">
                <input
                    class="navbar__theme-light"
                    name="theme"
                    type="radio"

                    checked=move || dark.get() == Theme::Light
                    on:change=move |_| dark.set(Theme::Light)
                />
                <svg
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    aria-hidden="true"
                >
                    <circle cx="12" cy="12" r="4"></circle>
                    <path d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"></path>
                </svg>
            </label>

            <label aria-label="Toggle Theme System">
                <input
                    class="navbar__theme-system"
                    name="theme"
                    type="radio"

                    checked=move || dark.get() == Theme::System
                    on:change=move |_| dark.set(Theme::System)
                />
                <svg
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    aria-hidden="true"
                >
                    <rect x="3" y="4" width="18" height="12" rx="2"></rect>
                    <path d="M8 20h8M12 16v4"></path>
                </svg>
            </label>

            <label aria-label="Toggle Theme Dark">
                <input
                    class="navbar__theme-dark"
                    name="theme"
                    type="radio"

                    checked=move || dark.get() == Theme::Dark
                    on:change=move |_| dark.set(Theme::Dark)
                />
                <svg viewBox="0 0 24 24" fill="currentColor" stroke="none" aria-hidden="true">
                    <path d="M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8z"></path>
                </svg>
            </label>
        </div>
    }
}

#[component]
pub fn NavBar(navbar_list: &'static [&'static str]) -> impl IntoView {
    let target = NodeRef::<Div>::new();
    let (is_open, set_open) = signal(false);
    let _ = on_click_outside_with_options(
        target,
        move |_| set_open.set(false),
        OnClickOutsideOptions::default().ignore([".navbar__icon"]),
    );

    let navbar_items = navbar_list
        .iter()
        .map(|&item| {
            view! {
                <li class="navbar__item">
                    <A
                        href=match item {
                            "/Home" => "/",
                            item => item,
                        }
                        attr:class="navbar__link"
                    >
                        {&item[1..]}
                    </A>
                </li>
            }
        })
        .collect_view();

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
