use crate::error_template::AppError;
use crate::error_template::ErrorTemplate;
use crate::navbar::Navbar;
use crate::projects::Project;
use crate::projects::Projects;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (is_routing, set_is_routing) = create_signal(false);

    view! {
        <Stylesheet id="leptos" href="/pkg/website-2.css"/>
        <Html lang="en"/>
        <Meta name="description" content="Lukas Hermansson's personal website"/>
        // sets the document title
        <Title text="Lukas Hermansson"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        } set_is_routing>
            <div class="routing-progress">
                <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
            </div>
            <Navbar/>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/projects" view=Projects/>
                    <Route path="/projects/:id" view=Project/>
                </Routes>
            </main>

            <Footer/>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Meta property="og:title" content="Lukas Hermansson" />
        <Meta property="og:description" content="Lukas Hermansson's personal website" />
        <Meta property="og:type" content="website" />
        <Meta property="og:image" content="https://www.lukashermansson.me/assets/og-card.jpg" />
        <Title text="Lukas Hermansson"/>
        <div class="m-auto md:w-3/5 w-100 flex flex-col text-gray-400 ">
            <h2 class="font-bold text-3xl text-center m-6">My code-values</h2>
            <div class="text-center">
                <p class="m-2">
                    I value code-quality, code that is easy to maintain with good separation and structure, stuff that facilitates longlived code-bases that are easy to change.
                </p>
                <p class="m-2">
                    I value unit tests on the code I work with, partly because of the testing and the tests that are produced but also because of the higher code quality produced that arise from test driven development.
                </p>
                <p class="m-2">
                    I am passionate about code that is self-documenting on a micro and a marco perspective, from tests, carefully chosen symbols and a clear structure.
                </p>
            </div>
            <h2 class="font-bold text-3xl text-center m-6">My programming languages</h2>
            <div class="flex place-content-around flex-wrap gap-4">
                <ProgrammingLang language_name="Javascript" url="/assets/javascript.svg"/>
                <ProgrammingLang language_name="Typescript" url="/assets/Typescript-logo.svg"/>
                <ProgrammingLang language_name="C#" url="/assets/c-sharp.svg"/>
                <ProgrammingLang language_name="Rust" url="/assets/Rust-Icon.svg"/>
                <ProgrammingLang language_name="Java" url="/assets/java.svg"/>
                <ProgrammingLang language_name="CSS" url="/assets/CSS3-logo.svg"/>
                <ProgrammingLang language_name="HTML" url="/assets/HTML5_Badge.svg"/>
                <ProgrammingLang language_name="Sass" url="/assets/Sass-logo.svg"/>
                <ProgrammingLang language_name="Kotlin" url="/assets/Kotlin-Icon.svg"/>
                <ProgrammingLang language_name="PHP" url="/assets/PHP-logo.svg"/>
                <ProgrammingLang language_name="SQL" url="/assets/MySQL-logo.svg"/>
            </div>
        </div>
    }
}
#[component]
fn ProgrammingLang(language_name: &'static str, url: &'static str) -> impl IntoView {
    view! {
        <div class="p-3 flex flex-col items-center rounded shadow-md shadow-gray-950 bg-slate-800 min-w-[190px]">
            <img alt=language_name src=url class="h-12"/>
            <p>{language_name}</p>
        </div>
    }
}
#[component]
fn Footer() -> impl IntoView {
    view! {
        <hr class="border-dashed border-0 border-b-2 w-full border-gray-500 mt-5"/>
        <footer class="flex p-1 items-center flex-col justify-center text-gray-400 mt-7">
            <div class="text-center">
                <hr class="border-dashed border-0 border-b-2 w-full border-gray-500"/>
                "Copyright © 2023 Lukas Hermansson"
                <br/>
                Made with Leptos and Rust
                <hr class="border-dashed border-0 border-b-2 w-full border-gray-500"/>
            </div>
            <div class="flex gap-2 my-4">
                <a href="https://github.com/lukashermansson" aria-label="github">
                    <svg
                        class="w-8 h-8"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 16 16"
                        width="1em"
                        height="1em"
                        fill="currentColor"
                    >
                        <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"></path>
                    </svg>
                </a>
                <a href="https://www.linkedin.com/in/lukas-hermansson-25502018a/" aria-label="linkedin">
                    <svg
                        class="w-8 h-8"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 16 16"
                        width="1em"
                        height="1em"
                        fill="currentColor"
                    >
                        <path d="M0 1.146C0 .513.526 0 1.175 0h13.65C15.474 0 16 .513 16 1.146v13.708c0 .633-.526 1.146-1.175 1.146H1.175C.526 16 0 15.487 0 14.854V1.146zm4.943 12.248V6.169H2.542v7.225h2.401zm-1.2-8.212c.837 0 1.358-.554 1.358-1.248-.015-.709-.52-1.248-1.342-1.248-.822 0-1.359.54-1.359 1.248 0 .694.521 1.248 1.327 1.248h.016zm4.908 8.212V9.359c0-.216.016-.432.08-.586.173-.431.568-.878 1.232-.878.869 0 1.216.662 1.216 1.634v3.865h2.401V9.25c0-2.22-1.184-3.252-2.764-3.252-1.274 0-1.845.7-2.165 1.193v.025h-.016a5.54 5.54 0 0 1 .016-.025V6.169h-2.4c.03.678 0 7.225 0 7.225h2.4z"></path>
                    </svg>
                </a>
            </div>
        </footer>
    }
}
