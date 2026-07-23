extern crate alloc;
use alloc::format;

use crate::page_template::StaticPage;

use leptos::prelude::*;
use leptos_router::components::{A, Outlet};

#[component]
pub fn GamePage(
    game_id: &'static str,
    #[prop(default = "800")] width: &'static str,
    #[prop(default = "600")] height: &'static str,
) -> impl IntoView {
    let address = format!("https://bha-gu.github.io/{game_id}/");

    view! { <iframe class="game-frame" src=address width=width height=height /> }
}

#[component]
pub fn GameList() -> impl IntoView {
    view! {
        <StaticPage>
            <div class="game-list">
                <h2>"Games"</h2>

                <div class="games-list">
                    <A href="Pong">"Pong"</A>
                    <A href="GameOfLife">"Game of Life"</A>
                    <A href="" exact=true>
                        "Unload Game"
                    </A>
                </div>
                <Outlet />
            </div>
        </StaticPage>
    }
}
