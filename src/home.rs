use crate::page_template::{SideBarPage, StaticPage};
use leptos::prelude::*;

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero" id="home">
            <div class="hero-content">
                <p class="hero-greeting">"Hi, I'm"</p>

                <h1 class="hero-title">"Bhaumikaditya Guleria"</h1>

                <h2 class="hero-subtitle">"Rust Software Developer"</h2>

                <p class="hero-description">
                    "I build reliable backend services, systems software, and modern web applications using Rust.
                    I'm passionate about performance, type safety, and writing software that lasts."
                </p>

                <div class="cta-buttons">
                    <a href="#projects" class="btn btn-primary">
                        <i class="fas fa-code"></i>
                        "Projects"
                    </a>

                    <a href="#contact" class="btn btn-secondary">
                        <i class="fas fa-envelope"></i>
                        "Contact"
                    </a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <section class="about" id="about">

            <h2>"About Me"</h2>

            <p>
                "I'm a software developer with a strong interest in systems programming,
                backend engineering, and modern web development using Rust."
            </p>

            <p>
                "I enjoy solving challenging problems, learning how software works under
                the hood, and building applications that are fast, reliable, and easy
                to maintain."
            </p>

        </section>
    }
}

#[component]
pub fn Skills() -> impl IntoView {
    let skills = vec![
        ("Languages", vec!["Rust", "Python"]),
        ("Frontend", vec!["Leptos", "HTML5", "CSS3"]),
        (
            "Libraries & Frameworks",
            vec!["Polars", "PyO3", "Pandas", "Numpy"],
        ),
        (
            "Machine Learning",
            vec![
                "Machine Learning",
                "Data Analysis",
                "Logical Analysis of Data (LAD)",
            ],
        ),
        (
            "Cybersecurity",
            vec!["Intrusion Detection Systems (IDS)", "Network Security"],
        ),
        ("Tools", vec!["Git", "Docker", "Linux"]),
    ];
    let skills = skills
        .into_iter()
        .map(|x| {
            view! {
                <div class="skill-category">
                    <h4>{x.0}</h4>
                    <div>
                        {x.1.into_iter().map(|x| view! { <span>{x}</span> }).collect::<Vec<_>>()}
                    </div>
                </div>
            }
        })
        .flat_map(|x| vec![view! {<hr/>}.into_any(), x.into_any()])
        .skip(1)
        .collect::<Vec<_>>();

    view! {
        <section class="skills" id="skills">

            <h2>"Skills"</h2>

            <div class="skill-grid">

                {skills}

            </div>

        </section>
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <section class="projects" id="projects">

            <h2>"Featured Projects"</h2>

            <div class="project-grid">

                <article class="project-card">

                    <h3>"LAD-Based Intrusion Detection System"</h3>

                    <p>
                        "Developed an intrusion detection system using Logical Analysis
                        of Data (LAD) to improve the classification of malicious network
                        traffic. The project introduces an efficient binarization
                        technique for heterogeneous and imbalanced datasets and evaluates
                        the model on the NSL-KDD and KDD-Cup99 benchmarks. The core
                        algorithm was implemented in Rust with Polars and exposed to
                        Python using PyO3 for experimentation and evaluation."
                    </p>

                    <div class="project-tech-stack">
                        <span>"Rust"</span>
                        <span>"Polars"</span>
                        <span>"PyO3"</span>
                        <span>"Python"</span>
                        <span>"Pandas"</span>
                        <span>"Numpy"</span>
                        <span>"Machine Learning"</span>
                        <span>"Network Security"</span>
                        <span>"LAD"</span>
                    </div>
                    <br />
                    <div class="project-links">
                        <a
                            href="https://link.springer.com/article/10.1007/s10586-025-05724-z"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            "Published Paper"
                        </a>
                    </div>

                </article>

            </div>

        </section>
    }
}
#[component]
pub fn Education() -> impl IntoView {
    view! {
        <section class="education" id="education">

            <h2>"Education"</h2>

            <article class="education-card">

                <h3>"Master of Technology (M.Tech) – Computer Science & Engineering"</h3>

                <p class="institution">
                    "National Institute of Technology Uttarakhand (NIT Uttarakhand)"
                </p>

                <p class="duration">"2023 – 2025"</p>

                <p>"CGPA: 8.23"</p>

                <p>"Specialization: Artificial Intelligence & Machine Learning (AI & ML)"</p>

                <p>
                    "Major Project: Intrusion Detection System using Logical Analysis of Data (LAD)."
                </p>

            </article>

            <article class="education-card">

                <h3>
                    "Bachelor of Technology (B.Tech) – Electronics & Communication Engineering"
                </h3>

                <p class="institution">
                    "Jawaharlal Nehru Government Engineering College (JNGEC), Sundernagar
                    | Himachal Pradesh Technical University (HPTU)"
                </p>

                <p class="duration">"2019 – 2023"</p>

                <p>"CGPA: 6.67"</p>

            </article>

        </section>
    }
}

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section class="contact" id="contact">

            <h2>"Let's Connect"</h2>

            <p>
                "Whether you have an interesting project, an internship opportunity,
                or simply want to talk about Rust, I'd be happy to hear from you."
            </p>

            <div class="contact-links">

                <a href="mailto:bhaumik050402@gmail.com">"Email"</a>

                <a href="https://github.com/Bha-Gu">"GitHub"</a>

                <a href="https://linkedin.com/in/bhaumikaditya-guleria-555aa937a">"LinkedIn"</a>

            </div>

        </section>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <StaticPage>
            <Hero />
        </StaticPage>
        <SideBarPage ids=vec![
            "About".to_string(),
            "Skills".to_string(),
            "Projects".to_string(),
            "Education".to_string(),
        ]>
            <div class="portfolio">
                <About />
                <Skills />
                <Projects />
                <Education />
            </div>
        </SideBarPage>
        <StaticPage>
            <Contact />
        </StaticPage>
    }
}
