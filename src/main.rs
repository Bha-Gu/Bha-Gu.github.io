#![allow(non_snake_case)]
#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};

mod game_page;
mod home;
mod navbar;
mod page_template;
mod posts;
mod secret;
use game_page::{GameList, GamePage};
use home::Home;
use navbar::NavBar;
use posts::{PostList, PostPage, PostRoot};
use secret::SecretPageLogin;

use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::path;

#[allow(clippy::must_use_candidate)]
#[component]
pub fn Breadcrumbs() -> impl IntoView {
    let location = use_location();

    let breadcrumbs = move || {
        let path = location.pathname.get();

        path.split('/')
            .filter(|s| !s.is_empty())
            .scan(String::new(), |acc, segment| {
                acc.push('/');
                acc.push_str(segment);

                Some((acc.clone(), segment.to_string()))
            })
            .map(|(href, label)| {
                view! {
                    <span>"/"</span>
                    <a href=href>{label}</a>
                }
            })
            .collect_view()
    };

    view! {
        <p class="breadcrumb">
            <a href="/">"Home"</a>
            {breadcrumbs}
        </p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let routes = &["/Home", "/Posts", "/Games", "/Secret"];

    view! {
        <Router>
            <NavBar navbar_list=routes />
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=Home />
                <ParentRoute path=path!("/Games") view=GameList>
                    <Route path=path!("") view=|| view! { <div>"Select a game"</div> } />
                    <Route path=path!("Pong") view=|| view! { <GamePage game_id="pong_bevy" /> } />
                    <Route
                        path=path!("GameOfLife")
                        view=|| {
                            view! {
                                <GamePage game_id="wasm-game-of-life" width="950" height="950" />
                            }
                        }
                    />
                </ParentRoute>
                <ParentRoute path=path!("/Posts") view=PostRoot>
                    <Route path=path!("") view=PostList />

                    <Route path=path!(":slug") view=PostPage />
                </ParentRoute>

                <ParentRoute path=path!("/Secret") view=SecretPageLogin>

                    <Route path=path!("") view=GameList />

                    <Route path=path!("Pong") view=|| view! { <GamePage game_id="pong_bevy" /> } />
                    <Route
                        path=path!("GameOfLife")
                        view=|| {
                            view! {
                                <GamePage game_id="wasm-game-of-life" width="950" height="950" />
                            }
                        }
                    />
                // <Route path=path!(":slug") view=PostPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
