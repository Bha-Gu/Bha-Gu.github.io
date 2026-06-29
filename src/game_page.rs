use crate::page_template::StaticPage;
use leptos::prelude::*;
use leptos_router::components::{A, Outlet};

#[component]
pub fn GamePage(
    game_id: String,
    #[prop(default = "800".to_string())] width: String,
    #[prop(default = "600".to_string())] height: String,
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
                    <A href="" exact=true>
                        "Unload Game"
                    </A>
                </div>
                <Outlet />
            </div>
        </StaticPage>
    }
}
