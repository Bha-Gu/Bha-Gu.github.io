use crate::page_template::StaticPage;
use leptos::prelude::*;

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero" id="home">
            <div class="hero-content">
                <h1 class="hero-title">Bhaumikaditya Guleria</h1>
                <h2 class="hero-subtitle">Rust Software Developer</h2>

                <div class="cta-buttons">
                    <a href="#projects" class="btn btn-primary">
                        <i class="fas fa-code"></i> View My Work
                    </a>
                    <a href="#contact" class="btn btn-secondary">
                        <i class="fas fa-envelope"></i> Get In Touch
                    </a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <section class="hero" id="home">
            <div class="hero-content">
                <h1 class="hero-title">Bhaumikaditya Guleria</h1>
                <h2 class="hero-subtitle">Rust Software Developer</h2>
                <p class="hero-description">
                    Passionate developer with extensive experience in building scalable web applications
                    using modern technologies. Specialized in microservice architecture, API development,
                    and full-stack solutions.
                </p>
                <div class="cta-buttons">
                    <a href="#projects" class="btn btn-primary">
                        <i class="fas fa-code"></i> View My Work
                    </a>
                    <a href="#contact" class="btn btn-secondary">
                        <i class="fas fa-envelope"></i> Get In Touch
                    </a>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    // let ids = vec!["Home".into()];
    view! {
        <StaticPage>
            <Hero />
        </StaticPage>
    }
}
