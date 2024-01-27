use crate::error_template::AppError;
use crate::error_template::ErrorTemplate;
use chrono::NaiveDate;
use leptos::Suspense;
use leptos::*;
use leptos_router::{use_params_map, A};
use serde::{Deserialize, Serialize};

#[server()]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    let paths = std::fs::read_dir("./projects").unwrap();

    let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
    return Ok(paths
        .into_iter()
        .map(|p| {
            let f_entry = p.unwrap();
            let content = std::fs::read_to_string(f_entry.path()).unwrap();
            let result = matter.parse(&content);
            let data = result.data.unwrap();
            Project {
                url: format!(
                    "/projects/{}",
                    f_entry
                        .file_name()
                        .into_string()
                        .unwrap()
                        .strip_suffix(".mdx")
                        .unwrap()
                )
                .to_string(),
                name: data["title"].as_string().unwrap(),
                date: NaiveDate::parse_from_str(
                    data["date"].as_string().unwrap().as_str(),
                    "%Y-%m-%d",
                )
                .unwrap(),
                description: data["description"].as_string().unwrap(),
                tags: data["tech"]
                    .as_vec()
                    .unwrap()
                    .iter()
                    .map(|p| p.as_string().unwrap())
                    .collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<_>>());
}

#[server()]
pub async fn get_project(name: String) -> Result<(String, String), ServerFnError> {
    let file = std::fs::read_to_string(format!("./projects/{}.mdx", name))
        .map_err(|_| ServerFnError::new("Not found"))?;

    let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
    let result = matter.parse(&file);
    return Ok((
        result.data.unwrap()["title"].as_string().unwrap(),
        markdown::to_html(&result.content),
    ));
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    url: String,
    name: String,
    date: NaiveDate,
    description: String,
    tags: Vec<String>,
}
use leptos_meta::Meta;
#[component]
pub fn Projects() -> impl IntoView {
    let once = create_blocking_resource(|| (), |_| async move { get_projects().await });
    view! {
        <div class="m-auto w-4/5 flex flex-col text-gray-400 ">
            <Suspense fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || match once.get() {
                    None => view! { <p>"Loading..."</p> }.into_view(),
                    Some(data) => {
                        view! {
                            <Title text="Lukas Hermansson"/>
                            <Meta property="og:title" content="Projects"/>
                            <Meta
                                property="og:description"
                                content="Lukas Hermansson's projects listing"
                            />
                            <Meta
                                property="og:image"
                                content="https://www.lukashermansson.me/assets/og-card.jpg"
                            />
                            <Meta property="og:type" content="website"/>
                            <div class="place-content-around grid mt-2 gap-4 grid-flow-row grid-cols-1 md:grid-cols-2">
                                {data
                                    .unwrap()
                                    .into_iter()
                                    .map(|n| {
                                        view! {
                                            <div class="p-3 flex flex-col rounded shadow-md shadow-gray-950 bg-slate-800 rounded shadow-md shadow-gray-950">
                                                <A href=n.url>
                                                    <h2 class="text-2xl font-bold">
                                                        {&n.name}
                                                        <span class="italic opacity-75 font-light ml-1">
                                                            {&n.date.to_string()}
                                                        </span>
                                                    </h2>
                                                    <p>{&n.description}</p>
                                                    <div>
                                                        {n
                                                            .tags
                                                            .into_iter()
                                                            .map(|n| {
                                                                view! {
                                                                    <p class="inline-block bg-slate-900 rounded m-1 p-1 ">
                                                                        {n}
                                                                    </p>
                                                                }
                                                            })
                                                            .collect::<Vec<_>>()}
                                                    </div>
                                                </A>
                                            </div>
                                        }
                                    })
                                    .collect::<Vec<_>>()}

                            </div>
                        }
                            .into_view()
                    }
                }}

            </Suspense>
        </div>
    }
}
use leptos_meta::Title;
#[component]
pub fn Project() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned()).unwrap();
    let once = create_blocking_resource(id, |arg| async move { get_project(arg).await });

    view! {
        <div class="m-auto w-4/5 flex flex-col text-gray-400 ">
            <Suspense fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || match once.get() {
                    None => view! { <p>"Loading..."</p> }.into_view(),
                    Some(data) => {
                        view! {
                            // the fallback receives a signal containing current errors
                            <ErrorBoundary fallback=|errors| {
                                view! { <ErrorTemplate errors/> }
                            }>

                                {match data {
                                    Err(_) => Err::<(), AppError>(AppError::NotFound).into_view(),
                                    Ok((title, data)) => {
                                        view! {
                                            <Meta property="og:title" content=format!("{}", &title)/>
                                            <Meta
                                                property="og:description"
                                                content=format!("project: {}", &title)
                                            />
                                            <Meta property="og:type" content="website"/>
                                            <Meta
                                                property="og:image"
                                                content="https://www.lukashermansson.me/assets/og-card.jpg"
                                            />
                                            <Title text=format!("Lukas Hermansson - {}", &title)/>
                                            <h1 class="text-4xl my-3 font-bold">{title}</h1>
                                            <div class="post" inner_html=data></div>
                                        }
                                            .into_view()
                                    }
                                }}

                            </ErrorBoundary>
                        }
                            .into_view()
                    }
                }}

            </Suspense>
        </div>
    }
}
