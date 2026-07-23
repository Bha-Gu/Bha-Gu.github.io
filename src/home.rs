use crate::page_template::{SideBarPage, StaticPage};
use alloc::string::ToString;
use leptos::prelude::*;

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero" id="home">
            <div class="hero-content">
                <p class="hero-greeting">"Hi, I'm"</p>

                <h1 class="hero-title">"Bhaumikaditya Guleria"</h1>

                <h2 class="hero-subtitle">"Rust Software Developer"</h2>

                // <p class="hero-description">
                // "I build reliable backend services, systems software, and modern web applications using Rust.
                // I'm passionate about performance, type safety, and writing software that lasts."
                // </p>

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

            <h6>"Work in Progress"</h6>
        // <p>
        // "I'm a software developer with a strong interest in systems programming,
        // backend engineering, and modern web development using Rust."
        // </p>

        // <p>
        // "I enjoy solving challenging problems, learning how software works under
        // the hood, and building applications that are fast, reliable, and easy
        // to maintain."
        // </p>
        </section>
    }
}

#[component]
pub fn Skills() -> impl IntoView {
    let skills: [(&str, &[&str]); _] = [
        ("Languages", &["Rust", "Python"]),
        ("Frontend", &["Leptos"]),
        (
            "Libraries & Frameworks",
            &["Polars", "PyO3", "Pandas", "Numpy"],
        ),
        (
            "Machine Learning",
            &["Data Analysis", "Logical Analysis of Data (LAD)"],
        ),
        (
            "Cybersecurity",
            &["Intrusion Detection Systems (IDS)", "Network Security"],
        ),
        ("Tools", &["Git", "Docker", "Linux"]),
    ];
    let skills = skills
        .into_iter()
        .map(|x| {
            view! {
                <div class="skill-category">
                    <h3>{x.0}</h3>
                    <div>{x.1.iter().map(|&x| view! { <span>{x}</span> }).collect_view()}</div>
                </div>
            }
        })
        .flat_map(|x| [view! { <hr /> }.into_any(), x.into_any()])
        .skip(1)
        .collect_view();

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

            // <article class="project-card">
            // <h3>"IDS-LAD: Intrusion detection system using logical analysis of data"</h3>
            // <p>
            // "Intrusion Detection Systems are essential for networks to ensure the confidentiality, integrity, and availability of data. In today’s increasingly interconnected world, IDS remain a critical component of cybersecurity strategies. Traditional IDS, and even recent Deep Learning based approaches, often suffer from high false positive rates, computational inefficiencies, and limited adaptability. To address these challenges, We proposed a LAD-based IDS model in which we introduced a binarization technique that efficiently extracts critical patterns from heterogeneous and imbalanced network traffic data. The proposed model is evaluated using well-known datasets, such as NSL-KDD and KDD-Cup99, to ensure a rigorous assessment against diverse attack scenarios. Experimental evaluation reveals that the IDS-LAD-based method achieved an accuracy of 80.026% and 99.872%, precision of 80.921% and 99.870%, recall of 81.260% and 99.782%, and an F2-Score of 80.498% and 99.865% on the NSL-KDD and KDD-Cup99 datasets, respectively."
            // </p>
            // <div class="project-tech-stack">
            // <span>"IDS"</span>
            // <span>"LAD"</span>
            // <span>"Binarization"</span>
            // <span>"Anomaly Detection"</span>
            // <span>"Network traffic analysis"</span>
            // </div>
            <div class="project-grid">// <br />
            // <div class="project-links">
            // <a
            // href="https://link.springer.com/article/10.1007/s10586-025-05724-z"
            // target="_blank"
            // rel="noopener noreferrer"
            // >
            // "Springer"
            // </a>
            // </div>

            // </article>
            </div>

        </section>
    }
}
#[component]
pub fn Papers() -> impl IntoView {
    view! {
        <section class="papers" id="papers">

            <h2>"Published Papers"</h2>

            <div class="paper-grid">

                <article class="paper-card">

                    <h3>"IDS-LAD: Intrusion detection system using logical analysis of data"</h3>

                    <p>
                        "Intrusion Detection Systems are essential for networks to ensure the confidentiality, integrity, and availability of data. In today’s increasingly interconnected world, IDS remain a critical component of cybersecurity strategies. Traditional IDS, and even recent Deep Learning based approaches, often suffer from high false positive rates, computational inefficiencies, and limited adaptability. To address these challenges, We proposed a LAD-based IDS model in which we introduced a binarization technique that efficiently extracts critical patterns from heterogeneous and imbalanced network traffic data. The proposed model is evaluated using well-known datasets, such as NSL-KDD and KDD-Cup99, to ensure a rigorous assessment against diverse attack scenarios. Experimental evaluation reveals that the IDS-LAD-based method achieved an accuracy of 80.026% and 99.872%, precision of 80.921% and 99.870%, recall of 81.260% and 99.782%, and an F2-Score of 80.498% and 99.865% on the NSL-KDD and KDD-Cup99 datasets, respectively."
                    </p>

                    <div class="paper-tags">
                        <span>"IDS"</span>
                        <span>"LAD"</span>
                        <span>"Binarization"</span>
                        <span>"Anomaly Detection"</span>
                        <span>"Network traffic analysis"</span>
                    </div>
                    <br />
                    <div class="paper-links">
                        <a
                            href="https://link.springer.com/article/10.1007/s10586-025-05724-z"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            // <img src="path/to/image.svg" alt="Description of the image" width="100" height="100"/>
                            <img class="light" src="assets/vectors/springer.svg" alt="Springer" />
                            <img
                                class="dark"
                                src="assets/vectors/springer-dark.svg"
                                alt="Springer"
                            />
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
    let ids = [
        "About".to_string(),
        "Skills".to_string(),
        "Papers".to_string(),
        "Education".to_string(),
    ];
    view! {
        <StaticPage>
            <Hero />
        </StaticPage>
        <SideBarPage ids=ids.to_vec()>
            <div class="portfolio">
                <About />
                <Skills />
                // <Projects />
                <Papers />
                <Education />
            </div>
        </SideBarPage>
        <StaticPage>
            <Contact />
        </StaticPage>
    }
}
