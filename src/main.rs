#![allow(non_snake_case)]

use chrono::{DateTime, FixedOffset};
use gray_matter::{Matter, engine::YAML};
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::{A, Outlet, ParentRoute, Route, Router, Routes};
use leptos_router::path;
use pulldown_cmark::{Options, Parser, html};
use rust_embed::RustEmbed;
use serde::Deserialize;
use web_sys::window;

#[derive(RustEmbed)]
#[folder = "assets/posts/"]
#[include = "*.md"]
struct Posts;

#[derive(Debug, Deserialize, Clone)]
struct FrontMatter {
    layout: Option<String>,
    title: String,
    date: DateTime<FixedOffset>,
    categories: Option<String>,
    tags: Option<Vec<String>>,
    read_time: Option<usize>,
    excerpt: Option<String>,
}

fn extract_front_matter(md: &str) -> Option<FrontMatter> {
    let matter = Matter::<YAML>::new();
    let result = matter.parse::<FrontMatter>(md).ok()?;

    result.data
}

fn markdown_to_html(md: &str) -> Option<String> {
    let matter = Matter::<YAML>::new();
    let result = matter.parse::<FrontMatter>(md);
    let result = match result {
        Ok(r) => r,
        Err(e) => {
            leptos::leptos_dom::logging::console_log(&format!("md->html error {e:?}"));
            return None;
        }
    };

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&result.content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Some(html_output)
}

#[component]
fn GamePage(
    game_id: String,
    #[prop(default = "800".to_string())] width: String,
    #[prop(default = "600".to_string())] height: String,
) -> impl IntoView {
    let address = format!("https://bha-gu.github.io/{game_id}/");

    view! { <iframe class="game-frame" src=address width=width height=height /> }
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
    let navbar_list = vec!["Home", "Contact", "Posts", "Games"];

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
fn PostList() -> impl IntoView {
    use std::collections::BTreeMap;

    let posts: Vec<(String, FrontMatter)> = Posts::iter()
        .filter_map(|path| {
            let md = Posts::get(&path)?;

            let markdown = String::from_utf8(md.data.into_owned()).ok()?;
            let front = extract_front_matter(&markdown)?;

            let slug = path.trim_end_matches(".md").to_string();

            Some((slug, front))
        })
        .collect();

    let mut grouped: BTreeMap<String, Vec<(String, FrontMatter)>> = BTreeMap::new();

    for (slug, front) in posts {
        let month = front.date.format("%B %Y").to_string();
        grouped.entry(month).or_default().push((slug, front));
    }

    // newest month first
    let grouped: Vec<_> = grouped.into_iter().rev().collect();

    view! {
        <div class="post-list">
            <h2>"Posts"</h2>

            <div class="posts-list">
                <For
                    each=move || grouped.clone()
                    key=|(month, _)| month.clone()
                    children=move |(month, posts)| {
                        view! {
                            <>
                                <h3 class="group-heading">{month}</h3>
                                <For
                                    each=move || posts.clone()
                                    key=|(slug, _)| slug.clone()
                                    children=move |(slug, front)| {
                                        view! {
                                            <A href=slug>
                                                <h4>{front.title}</h4>
                                                <p>{front.excerpt}</p>
                                                <div class="post-tags">
                                                    <For
                                                        each=move || front.tags.clone().unwrap_or(vec![])
                                                        key=|tag| tag.clone()
                                                        children=move |tag| {
                                                            view! {
                                                                <span class="post-tag">{tag.clone()}</span>
                                                            }
                                                        }
                                                    />
                                                </div>
                                                <small>
                                                    {front.date.format("%b %-d, %Y").to_string()} " • "
                                                    {front.read_time} " min"
                                                </small>
                                            </A>
                                        }
                                    }
                                />
                            </>
                        }
                    }
                />
            </div>
        </div>
    }
}

use leptos_router::hooks::use_params_map;

#[component]
fn PostPage() -> impl IntoView {
    let params = use_params_map();

    let slug = move || params.read().get("slug").unwrap_or_default();

    let markdown = move || {
        let filename = format!("{}.md", slug());

        Posts::get(&filename).map(|x| String::from_utf8(x.data.into_owned()).unwrap())
    };

    view! {
        {move || {
            markdown()
                .map(|md| {
                    let front = extract_front_matter(&md).unwrap();
                    let html = markdown_to_html(&md).unwrap();

                    view! {
                        <article>
                            <h1>{front.title}</h1>
                            <article inner_html=html />
                        </article>
                    }
                })
                .unwrap()
        }}
    }
}

#[component]
fn GameList() -> impl IntoView {
    view! {
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
                        <Route
                            path=path!("Pong")
                            view=|| view! { <GamePage game_id="pong_bevy".to_string() /> }
                        />
                    </ParentRoute>
                    <ParentRoute
                        path=path!("/Posts")
                        view=|| {
                            view! {
                                <div>
                                    <Outlet />
                                </div>
                            }
                        }
                    >
                        <Route
                            path=path!("")
                            view=|| {
                                view! {
                                    <div>
                                        <PostList />
                                    </div>
                                }
                            }
                        />
                        <Route path=path!(":slug") view=PostPage />
                    </ParentRoute>

                </Routes>
            </main>
        </Router>
    }
}
