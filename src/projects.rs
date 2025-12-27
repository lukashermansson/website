use chrono::NaiveDate;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use serde::{Deserialize, Serialize};
#[server()]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    use gray_matter::ParsedEntity;
    let paths = std::fs::read_dir("./projects").unwrap();

    let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
    let mut projects = futures::future::join_all(
        paths
            .into_iter()
            .map(|p| async {
                let f_entry = p.unwrap();
                let content = tokio::fs::read_to_string(f_entry.path()).await.unwrap();
                let result: ParsedEntity = matter.parse(&content).unwrap();
                let data = result.data.as_ref().unwrap();
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
            .collect::<Vec<_>>(),
    )
    .await;
    projects.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(projects)
}

#[server()]
pub async fn get_project(name: String) -> Result<(String, String), ServerFnError> {
    use gray_matter::ParsedEntity;
    let mut path = std::path::PathBuf::from("./projects/file");

    path.set_file_name(name);
    path.set_extension("mdx");
    let file = tokio::fs::read_to_string(path)
        .await
        .map_err(|_| ServerFnError::new("Not found"))?;

    let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
    let result: ParsedEntity = matter.parse(&file).unwrap();
    Ok((
        result.data.unwrap()["title"].as_string().unwrap(),
        markdown::to_html(&result.content),
    ))
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
    let once = Resource::new(|| (), |_| async move { get_projects().await });
    view! {
        <Title text="Lukas Hermansson"/>
        <Meta property="og:title" content="Projects"/>
        <Meta property="og:description" content="Lukas Hermansson's projects listing"/>
        <Meta property="og:image" content="https://www.lukashermansson.me/assets/og-card.jpg"/>
        <Meta property="og:type" content="website"/>
        <div class="m-auto md:w-3/5 w-full max-md:m-2  flex flex-col text-gray-400 ">
            <div class="place-content-around grid mt-2 gap-4 grid-flow-row grid-cols-1 lg:grid-cols-2">
                <Suspense>
                    {move || match once.get() {
                        None => view! { <ProjectsPlaceholder/> }.into_any(),
                        Some(data) => {
                            view! {
                                {data
                                    .unwrap()
                                    .into_iter()
                                    .map(|n| {
                                        view! {
                                            <div class="p-3 flex flex-col rounded shadow-md shadow-gray-950 bg-slate-800 rounded shadow-md shadow-gray-950">
                                                <A href=n.url>
                                                    <h2 class="text-2xl font-bold">
                                                        {n.name}
                                                        <span class="italic block float-right opacity-75 font-light ml-1">
                                                            {n.date.to_string()}
                                                        </span>
                                                    </h2>
                                                    <p>{n.description}</p>
                                                    <div>
                                                        {n
                                                            .tags
                                                            .into_iter()
                                                            .map(|n| {
                                                                view! {
                                                                    <p class="inline-block bg-slate-900 rounded m-1 p-1">{n}</p>
                                                                }
                                                            })
                                                            .collect::<Vec<_>>()}
                                                    </div>
                                                </A>
                                            </div>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            }
                                .into_any()
                        }
                    }}

                </Suspense>
            </div>
        </div>
    }
}
#[component]
fn projects_placeholder() -> impl IntoView {
    (0..6)
                                    .map(|_| {
                                        view! {
                                            <div class="p-3 flex flex-col rounded shadow-md shadow-gray-950 bg-slate-800 rounded shadow-md shadow-gray-950 animate-pulse">
                                                <div class="flex flex-row mb-2">
                                                    <div class="h-5 w-40 bg-gray-400 rounded"></div>
                                                    <div class="opacity-75 ml-1 h-5 w-32 bg-gray-400 rounded"></div>
                                                </div>
                                                <div class="h-3 max-w-30 bg-gray-400 rounded mb-2"></div>
                                                <div>
                                                    {(0..3)
                                                        .map(|_| {
                                                            view! {
                                                                <div class="inline-block bg-slate-900 rounded m-1 p-1 h-5 w-10"></div>
                                                            }
                                                        })
                                                        .collect::<Vec<_>>()}
                                                </div>
                                            </div>
                                        }
                                    })
                                    .collect::<Vec<_>>()
}
use leptos_meta::Title;
#[component]
pub fn Project() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id")).unwrap();
    let resource = Resource::new_blocking(id, |arg| async move { get_project(arg).await });

    view! {
        <div class="m-auto md:w-3/5 w-full max-md:m-2 flex flex-col text-gray-400 ">
            <Suspense>
                {move || match resource.get() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(data) => {
                        view! {
                            // the fallback receives a signal containing current errors
                            <ErrorBoundary fallback=|_errors| {
                                view! { "" }.into_any()
                            }>

                                {match data {
                                    Err(_) => view!{"error"}.into_any(),
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
                                            .into_any()
                                    }
                                }}

                            </ErrorBoundary>
                        }
                            .into_any()
                    }
                }}

            </Suspense>
        </div>
    }
}
