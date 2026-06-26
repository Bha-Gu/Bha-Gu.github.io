#![allow(non_snake_case)]

use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::{A, Outlet, ParentRoute, Route, Router, Routes};
use leptos_router::path;
use web_sys::window;

#[component]
fn GamePage(
    game_id: String,
    #[prop(default = "800".to_string())] width: String,
    #[prop(default = "600".to_string())] height: String,
) -> impl IntoView {
    let address = format!("https://bha-gu.github.io/{game_id}/");

    view! { <iframe class="game-frame" src={address} width={width} height={height} /> }
}

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
fn NavBar() -> impl IntoView {
    let navbar_list = vec!["Home", "Contact", "Games"];

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

#[component]
fn SideBarPage(ids: Vec<String>, children: Children) -> impl IntoView {
    view! {
        <div class="page-layout">
            <main class="page-content">{children()}</main>

            <aside class="sidebar">
                <SideBar ids=ids />
            </aside>
        </div>
    }
}

#[component]
fn StaticPage(children: Children) -> impl IntoView {
    view! {
        <div class="page-layout">
            <main class="page-content">{children()}</main>

        </div>
    }
}

#[component]
fn SideBar(ids: Vec<String>) -> impl IntoView {
    let items = ids
        .into_iter()
        .map(|id| {
            view! {
                <li>
                    <a href=format!("#{id}") class="sidebar__link">
                        {id.clone()}
                    </a>
                </li>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <aside class="sidebar">
            <div class="sidebar__header">
                <h2 class="sidebar__title">"On This Page"</h2>
            </div>

            <div class="sidebar__section">{items}</div>
        </aside>
    }
}

fn main() {
    mount_to_body(App);
}

#[component]
fn Name() -> impl IntoView {
    let ids = vec!["Base".into(), "About".into()];
    view! {
        <SideBarPage ids=ids>
            <h1 id="Base">"Bhaumikaditya Guleria"</h1>

            <h1 id="About">"Bhaumikaditya Guleria"</h1>

        </SideBarPage>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <StaticPage>
            <h1>"About Me"</h1>



        </StaticPage>
    }
}

#[component]
fn GameList() -> impl IntoView {
    view! {
        <div class="game-list">
            <h2>"Games"</h2>

            <div class="games-list">
                <A href="Pong">"Pong"</A>
                <A href="" exact=true>"Unload Game"</A>
            </div>
            <Outlet />
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <NavBar />
            <main>
                <Routes fallback=|| "Not found.">
                    // / just has an un-nested "Home"
                    <Route path=path!("/") view=|| view! { <Name /> } />
                    <Route path=path!("/Contact") view=|| view! { <About /> } />
                    <ParentRoute path=path!("/Games") view=GameList>
                        <Route path=path!("") view=|| view! { <div>"Select a game"</div> } />
                        <Route path=path!("Pong") view=|| view! { <GamePage game_id="pong_bevy".to_string() /> } />
                    </ParentRoute>

                // <Route
                // path=path!("/Games")
                // view=GameList
                // />
                //
                // <Route
                // path=path!("/Games/pong")
                // view=|| view! {
                // <GamePage game_id="pong" />
                // }
                // />
                </Routes>
            </main>
        </Router>
    }
}
