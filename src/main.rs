#![allow(non_snake_case)]

mod about;
mod game_page;
mod home;
mod navbar;
mod page_template;
mod posts;
use about::About;
use game_page::{GameList, GamePage};
use home::Home;
use navbar::NavBar;
use posts::{PostList, PostPage, PostRoot};

use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::path;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let routes = vec!["Home", "About", "Posts", "Games"];

    view! {
        <Router>
            <NavBar navbar_list=routes />
            <Routes fallback=|| "Not found.">
                // / just has an un-nested "Home"
                <Route path=path!("/") view=Home />
                <Route path=path!("/About") view=About />
                <ParentRoute path=path!("/Games") view=GameList>
                    <Route path=path!("") view=|| view! { <div>"Select a game"</div> } />
                    <Route
                        path=path!("Pong")
                        view=|| view! { <GamePage game_id="pong_bevy".to_string() /> }
                    />
                </ParentRoute>
                <ParentRoute path=path!("/Posts") view=PostRoot>
                    <Route path=path!("") view=PostList />

                    <Route path=path!(":slug") view=PostPage />
                </ParentRoute>

            </Routes>
        </Router>
    }
}
