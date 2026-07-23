use crate::page_template::{SideBarPage, StaticPage};
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use chrono::{DateTime, FixedOffset};
// use gloo_net::http::Request;
use gray_matter::{Matter, engine::YAML};
use leptos::prelude::*;
use leptos_router::components::{A, Outlet};
use leptos_router::hooks::use_params_map;
use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};
// use rust_embed::RustEmbed;
use serde::Deserialize;
use slug::slugify;
use web_sys::console::log_1;

use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

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

const fn level_tag(level: HeadingLevel) -> &'static str {
    match level {
        HeadingLevel::H1 => "h1",
        HeadingLevel::H2 => "h2",
        HeadingLevel::H3 => "h3",
        HeadingLevel::H4 => "h4",
        HeadingLevel::H5 => "h5",
        HeadingLevel::H6 => "h6",
    }
}

fn markdown_to_html(md: &str) -> Option<(String, Vec<String>)> {
    let mut id_strings = Vec::new();
    let matter = Matter::<YAML>::new();
    let result = match matter.parse::<FrontMatter>(md) {
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

    let mut events = Vec::new();
    let mut iter = parser.into_iter();

    while let Some(event) = iter.next() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let mut heading_events = Vec::new();
                let mut heading_text = String::new();

                for e in iter.by_ref() {
                    match e {
                        Event::End(TagEnd::Heading(..)) => break,

                        Event::Text(ref t) | Event::Code(ref t) => {
                            heading_text.push_str(t);
                            heading_events.push(e);
                        }

                        _ => heading_events.push(e),
                    }
                }

                let tag = level_tag(level);
                id_strings.push(heading_text.clone());
                let id = slugify(&heading_text);
                events.push(Event::Html(CowStr::from(format!(r#"<{tag} id="{id}">"#))));
                events.extend(heading_events);
                events.push(Event::Html(CowStr::from(format!("</{tag}>"))));
            }

            other => events.push(other),
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());

    Some((html_output, id_strings))
}
#[component]
pub fn PostList() -> impl IntoView {
    let posts = LocalResource::new(|| async move {
        let files = ["Copy"];
        let mut posts = Vec::new();

        for &path in &files {
            let Some(window) = web_sys::window() else {
                log_1(&"can't get window".into());
                continue;
            };

            let Ok(resp) =
                JsFuture::from(window.fetch_with_str(&format!("./assets/posts/{path}.md"))).await
            else {
                log_1(&"fetch failed".into());
                continue;
            };

            let Ok(resp) = resp.dyn_into::<Response>() else {
                log_1(&"response cast failed".into());
                continue;
            };

            if !resp.ok() {
                log_1(&format!("HTTP {}", resp.status()).into());
                continue;
            }

            let Ok(buffer) = JsFuture::from(resp.array_buffer().unwrap()).await else {
                log_1(&"array_buffer failed".into());
                continue;
            };

            let md = Uint8Array::new(&buffer).to_vec();

            let Ok(markdown) = String::from_utf8(md) else {
                log_1(&"utf8 conversion failed".into());
                continue;
            };

            log_1(&markdown.clone().into());

            let Some(front) = extract_front_matter(&markdown) else {
                log_1(&"can't parse front matter".into());
                continue;
            };

            posts.push((path.to_string(), front));
        }

        let mut grouped: BTreeMap<String, Vec<(String, FrontMatter)>> = BTreeMap::new();

        for (slug, front) in posts {
            let month = front.date.format("%B %Y").to_string();
            grouped.entry(month).or_default().push((slug, front));
        }

        grouped.into_iter().rev().collect::<Vec<_>>()
    });

    view! {
        <Suspense fallback=|| {
            view! { <p>"Loading posts..."</p> }
        }>
            {move || {
                posts
                    .get()
                    .map(|grouped| {
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
                                                                                view! { <span class="post-tag">{tag}</span> }
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
                    })
            }}
        </Suspense>
    }
}

#[component]
pub fn PostRoot() -> impl IntoView {
    view! { <Outlet /> }
}

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params_map();

    let markdown = LocalResource::new(move || {
        let slug = params.read().get("slug").unwrap_or_default();

        async move {
            let window = web_sys::window()?;

            let path = format!("./assets/posts/{slug}.md");

            let response = JsFuture::from(window.fetch_with_str(&path))
                .await
                .ok()?
                .dyn_into::<Response>()
                .ok()?;

            if !response.ok() {
                return None;
            }

            let buffer = JsFuture::from(response.array_buffer().ok()?).await.ok()?;

            let bytes = Uint8Array::new(&buffer).to_vec();

            String::from_utf8(bytes).ok()
        }
    });
    view! {
        <Suspense fallback=|| {
            view! { <p>"Loading..."</p> }
        }>
            {move || {
                markdown
                    .get()
                    .map(|md| {
                        md.map(|md| {
                            let front = extract_front_matter(&md).unwrap();
                            let (html, ids) = markdown_to_html(&md).unwrap();

                            view! {
                                <StaticPage>
                                    <article class="post">
                                        <header class="post-header">
                                            <h1>{front.title}</h1>

                                            <div class="post-meta">
                                                <span>{front.date.format("%B %-d, %Y").to_string()}</span>
                                                <span>"•"</span>
                                                <span>{front.read_time}" min read"</span>
                                            </div>

                                            <div class="post-tags">
                                                {front
                                                    .tags
                                                    .clone()
                                                    .unwrap_or_default()
                                                    .into_iter()
                                                    .map(|tag| view! { <span class="post-tag">{tag}</span> })
                                                    .collect_view()}
                                            </div>

                                            <div class="post-excerpt">
                                                <span>{front.excerpt}</span>
                                            </div>
                                        </header>

                                        <SideBarPage ids=ids>
                                            <div class="post-content" inner_html=html />
                                        </SideBarPage>
                                    </article>
                                </StaticPage>
                            }
                        })
                    })
            }}
        </Suspense>
    }
}
