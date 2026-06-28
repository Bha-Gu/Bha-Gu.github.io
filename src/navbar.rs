use leptos::prelude::*;
use leptos_router::components::A;
use web_sys::window;

#[component]
fn ThemeToggle() -> impl IntoView {
    let (dark_mode, set_dark_mode) = signal(true);

    let toggle_theme = move |_| {
        let new_theme = !dark_mode.get();
        set_dark_mode.set(new_theme);

        if let Some(document) = window().and_then(|w| w.document())
            && let Some(html) = document.document_element()
        {
            let theme = if new_theme { "dark" } else { "light" };

            let _ = html.set_attribute("data-theme", theme);
        }
    };

    view! {
        <button
            class=move || {
                if dark_mode.get() { "navbar__theme-switch dark" } else { "navbar__theme-switch" }
            }
            on:click=toggle_theme
            aria-label="Toggle theme"
        >
            <span class="navbar__theme-slider"></span>

            <span class="navbar__theme-option">"☀️"</span>

            <span class="navbar__theme-option">"🌙"</span>
        </button>
    }
}

#[component]
pub fn NavBar(navbar_list: Vec<&'static str>) -> impl IntoView {
    // let navbar_list = vec!["Home", "Contact", "Posts", "Games"];

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
                <a class="navbar__logo">"MySite"</a>
                <ul class="navbar__menu">
                    {navbar_items} <li>

                        <ThemeToggle />
                    </li>
                </ul>

            </div>
        </nav>
    }
}
