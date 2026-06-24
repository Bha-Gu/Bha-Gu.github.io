use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::{A, Outlet, ParentRoute, Route, Router, Routes};
use leptos_router::hooks::use_params_map;
use leptos_router::path;

use web_sys::window;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (dark_mode, set_dark_mode) = signal(true);

    let toggle_theme = move |_| {
        let new_theme = !dark_mode.get();
        set_dark_mode.set(new_theme);

        if let Some(document) = window().and_then(|w| w.document()) {
            if let Some(html) = document.document_element() {
                let theme = if new_theme { "dark" } else { "light" };

                let _ = html.set_attribute("data-theme", theme);
            }
        }
    };

    view! {
    <button
        class=move || {
            if dark_mode.get() {
                "navbar__theme-switch dark"
            } else {
                "navbar__theme-switch"
            }
        }
        on:click=toggle_theme
        aria-label="Toggle theme"
    >
        <span class="navbar__theme-slider"></span>

        <span class="navbar__theme-option">
            "☀️"
        </span>

        <span class="navbar__theme-option">
            "🌙"
        </span>
    </button>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    let navbar_list = vec!["Home", "Contact"];

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
                    <a href=format!("#{}", id.clone()) class="sidebar__link">
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
    <p>"
 Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum ipsum dui, scelerisque a cursus vel, blandit at magna. Mauris maximus eu arcu sed faucibus. Donec porta rutrum libero vitae accumsan. Suspendisse erat risus, faucibus id placerat in, porta at turpis. Cras elit dolor, facilisis vel viverra quis, tincidunt et dui. Fusce id neque non mauris mollis vehicula sit amet eget lacus. Cras sed fringilla nibh. Fusce placerat lacus tempor, pellentesque leo sit amet, dapibus ex. Pellentesque vel accumsan risus. Nulla vel arcu nec libero ultrices blandit. Nunc mollis, mi ac porta rutrum, elit velit luctus est, ut interdum lacus tortor a elit. Suspendisse maximus luctus venenatis.

Aliquam erat volutpat. Fusce sit amet suscipit nulla. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Vestibulum eget condimentum dui. Aliquam vulputate id leo quis blandit. Nulla consectetur massa sit amet tincidunt scelerisque. Quisque et nibh erat. Duis ornare gravida tempus. Pellentesque maximus nisi pulvinar, vulputate nunc non, interdum ligula. Morbi mattis vitae magna id bibendum. Donec eget porta est. Duis maximus neque ex, molestie pretium ex ullamcorper sed.

Integer eu tellus tempor, dapibus lectus quis, malesuada velit. Curabitur a arcu quis nisl dictum tristique. Vivamus non elementum dolor. Phasellus malesuada, nibh nec blandit tincidunt, nulla urna convallis lorem, hendrerit mollis mi magna vel lacus. Aliquam et pellentesque nibh. Etiam ornare in nisi eu varius. Curabitur ut placerat risus, finibus ultricies sapien. Vivamus viverra libero metus, ut dictum justo imperdiet et. Aenean justo purus, elementum et mattis eget, accumsan ac nunc. Sed aliquam est vel ipsum venenatis commodo ut et sapien. Vivamus efficitur nibh sed mauris vehicula, posuere facilisis ex posuere. Vestibulum pellentesque purus eget lacinia iaculis. Quisque tincidunt mauris turpis, ac commodo sapien maximus id. Sed rutrum imperdiet nunc. Praesent lacinia, metus vel dignissim porttitor, libero eros bibendum quam, quis viverra odio purus eu est. Cras lobortis magna et tellus tempor euismod.

Integer efficitur, tortor semper venenatis dignissim, ipsum ligula finibus nisi, eget elementum lectus nisl nec sapien. Sed volutpat rutrum posuere. Donec venenatis ultrices nibh vel blandit. Nam eu dictum mauris. Curabitur luctus facilisis efficitur. Duis dictum velit in lorem cursus convallis. Nullam commodo iaculis mollis. Nulla laoreet nunc non ligula pellentesque, eu ultricies ligula aliquet. Nunc dignissim diam nisl, ut pellentesque quam faucibus sed. Nam et metus id libero sodales laoreet eget et massa. Nullam dolor augue, convallis eu consectetur id, commodo non tellus. Integer in efficitur orci.

Phasellus et leo ac nibh euismod bibendum. Ut at enim porta, gravida urna ut, condimentum nisi. Vestibulum ornare tellus eget enim consequat mattis. Proin quis felis elementum, fringilla neque eu, tempus enim. Aenean at pharetra eros. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Ut lobortis, magna id congue lacinia, tellus massa facilisis libero, id egestas enim leo sed mi. Maecenas accumsan, est in mattis porttitor, dolor lectus ultrices justo, nec convallis orci dolor non elit. Morbi in posuere lectus. Nam vitae eros et quam maximus mattis. Quisque vel urna quam. Praesent quis tellus at mauris dapibus commodo sed vel est. 
        "</p>
                <h1 id="About">"Bhaumikaditya Guleria"</h1>

            </SideBarPage>
        }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <StaticPage>
            <h1>"About Me"</h1>

    <p>"
 Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum ipsum dui, scelerisque a cursus vel, blandit at magna. Mauris maximus eu arcu sed faucibus. Donec porta rutrum libero vitae accumsan. Suspendisse erat risus, faucibus id placerat in, porta at turpis. Cras elit dolor, facilisis vel viverra quis, tincidunt et dui. Fusce id neque non mauris mollis vehicula sit amet eget lacus. Cras sed fringilla nibh. Fusce placerat lacus tempor, pellentesque leo sit amet, dapibus ex. Pellentesque vel accumsan risus. Nulla vel arcu nec libero ultrices blandit. Nunc mollis, mi ac porta rutrum, elit velit luctus est, ut interdum lacus tortor a elit. Suspendisse maximus luctus venenatis.

Aliquam erat volutpat. Fusce sit amet suscipit nulla. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Vestibulum eget condimentum dui. Aliquam vulputate id leo quis blandit. Nulla consectetur massa sit amet tincidunt scelerisque. Quisque et nibh erat. Duis ornare gravida tempus. Pellentesque maximus nisi pulvinar, vulputate nunc non, interdum ligula. Morbi mattis vitae magna id bibendum. Donec eget porta est. Duis maximus neque ex, molestie pretium ex ullamcorper sed.

Integer eu tellus tempor, dapibus lectus quis, malesuada velit. Curabitur a arcu quis nisl dictum tristique. Vivamus non elementum dolor. Phasellus malesuada, nibh nec blandit tincidunt, nulla urna convallis lorem, hendrerit mollis mi magna vel lacus. Aliquam et pellentesque nibh. Etiam ornare in nisi eu varius. Curabitur ut placerat risus, finibus ultricies sapien. Vivamus viverra libero metus, ut dictum justo imperdiet et. Aenean justo purus, elementum et mattis eget, accumsan ac nunc. Sed aliquam est vel ipsum venenatis commodo ut et sapien. Vivamus efficitur nibh sed mauris vehicula, posuere facilisis ex posuere. Vestibulum pellentesque purus eget lacinia iaculis. Quisque tincidunt mauris turpis, ac commodo sapien maximus id. Sed rutrum imperdiet nunc. Praesent lacinia, metus vel dignissim porttitor, libero eros bibendum quam, quis viverra odio purus eu est. Cras lobortis magna et tellus tempor euismod.

Integer efficitur, tortor semper venenatis dignissim, ipsum ligula finibus nisi, eget elementum lectus nisl nec sapien. Sed volutpat rutrum posuere. Donec venenatis ultrices nibh vel blandit. Nam eu dictum mauris. Curabitur luctus facilisis efficitur. Duis dictum velit in lorem cursus convallis. Nullam commodo iaculis mollis. Nulla laoreet nunc non ligula pellentesque, eu ultricies ligula aliquet. Nunc dignissim diam nisl, ut pellentesque quam faucibus sed. Nam et metus id libero sodales laoreet eget et massa. Nullam dolor augue, convallis eu consectetur id, commodo non tellus. Integer in efficitur orci.

Phasellus et leo ac nibh euismod bibendum. Ut at enim porta, gravida urna ut, condimentum nisi. Vestibulum ornare tellus eget enim consequat mattis. Proin quis felis elementum, fringilla neque eu, tempus enim. Aenean at pharetra eros. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Ut lobortis, magna id congue lacinia, tellus massa facilisis libero, id egestas enim leo sed mi. Maecenas accumsan, est in mattis porttitor, dolor lectus ultrices justo, nec convallis orci dolor non elit. Morbi in posuere lectus. Nam vitae eros et quam maximus mattis. Quisque vel urna quam. Praesent quis tellus at mauris dapibus commodo sed vel est. 
        "</p>

        </StaticPage>
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
                </Routes>
            </main>
        </Router>
    }
}
