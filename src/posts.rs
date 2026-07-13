use crate::page_template::{SideBarPage, StaticPage};
use chrono::{DateTime, FixedOffset};
use gray_matter::{engine::YAML, Matter};
use leptos::prelude::*;
use leptos_router::components::{Outlet, A};
use leptos_router::hooks::use_params_map;
use pulldown_cmark::{html, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use rust_embed::RustEmbed;
use serde::Deserialize;
use slug::slugify;

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

fn level_tag(level: HeadingLevel) -> &'static str {
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

                while let Some(e) = iter.next() {
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
                                                            view! { <span class="post-tag">{tag.clone()}</span> }
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

#[component]
pub fn PostRoot() -> impl IntoView {
    view! { <Outlet /> }
}

#[component]
pub fn PostPage() -> impl IntoView {
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
                    let (html, ids) = markdown_to_html(&md).unwrap();
                    let html_copy = html.clone();

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
                                            .unwrap_or(vec![])
                                            .iter()
                                            .map(|tag| {
                                                view! { <span class="post-tag">{tag.clone()}</span> }
                                            })
                                            .collect_view()}
                                    </div>

                                    <div class="post-excerpt">
                                        <span>{front.excerpt}</span>
                                    </div>
                                </header>
                                <SideBarPage ids=ids.clone()>
                                    <div class="post-content" inner_html=html />
                                </SideBarPage>

                                <SideBarPage ids=ids>
                                    <div class="post-content" inner_html=html_copy />
                                </SideBarPage>
                            </article>
                        </StaticPage>
                    }
                })
                .unwrap()
        }}
    }
}
