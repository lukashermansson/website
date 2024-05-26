use crate::error_template::AppError;
use crate::error_template::ErrorTemplate;
use chrono::NaiveDate;
use leptos::*;
use leptos_router::{use_params_map, A};
use serde::{Deserialize, Serialize};

#[server()]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    let paths = std::fs::read_dir("./projects").unwrap();

    let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
    let mut projects = futures::future::join_all(paths
        .into_iter()
        .map(|p| async {
            let f_entry = p.unwrap();
            let content = tokio::fs::read_to_string(f_entry.path()).await.unwrap();
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
        .collect::<Vec<_>>()).await;
    projects.sort_by(|a, b| b.date.cmp(&a.date));
    return Ok(projects);
}

#[server()]
pub async fn get_project(name: String) -> Result<(String, String), ServerFnError> {
    let mut path = PathBuf::from("./projects/file");
    path.set_file_name(name);
    path.set_extension("mdx");
    let file =  tokio::fs::read_to_string(path).await
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
    let once = create_resource(|| (), |_| async move { get_projects().await });
    view! {
        <Title text="Lukas Hermansson"/>
        <Meta property="og:title" content="Projects"/>
        <Meta property="og:description" content="Lukas Hermansson's projects listing"/>
        <Meta property="og:image" content="https://www.lukashermansson.me/assets/og-card.jpg"/>
        <Meta property="og:type" content="website"/>
        <div class="m-auto md:w-3/5 w-100 max-md:m-2  flex flex-col text-gray-400 ">
            <div class="place-content-around grid mt-2 gap-4 grid-flow-row grid-cols-1 lg:grid-cols-2">
                <Suspense fallback=move || {
                    view! { <ProjectsPlaceholder/> }
                }>
                    {move || match once.get() {
                        None => view! { <ProjectsPlaceholder/> },
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
                                                        {&n.name}
                                                        <span class="italic block float-right opacity-75 font-light ml-1">
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
                                                                    <p class="inline-block bg-slate-900 rounded m-1 p-1">
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
                            }
                                .into_view()
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
    let id = move || params.with(|params| params.get("id").cloned()).unwrap();
    let resource = create_blocking_resource(id, |arg| async move { get_project(arg).await });

    view! {
        <div class="m-auto md:w-3/5 w-100 max-md:m-2 flex flex-col text-gray-400 ">
            <Suspense fallback=move || {
                view! {
                    <div class="animate-pulse ">
                        <div class="h-7 my-6 bg-gray-400 w-64 rounded"></div>
                        <div role="status" class="space-y-2.5 animate-pulse max-w-lg">
                            <div class="flex items-center w-full">
                                <div class="h-2.5 bg-gray-200 rounded-full bg-gray-700 w-32"></div>
                                <div class="h-2.5 ms-2 bg-gray-300 rounded-full bg-gray-600 w-24"></div>
                                <div class="h-2.5 ms-2 bg-gray-300 rounded-full bg-gray-600 w-full"></div>
                            </div>
                            <div class="flex items-center w-full max-w-[480px]">
                                <div class="h-2.5 bg-gray-200 rounded-full bg-gray-700 w-full"></div>
                                <div class="h-2.5 ms-2 bg-gray-300 rounded-full bg-gray-600 w-full"></div>
                                <div class="h-2.5 ms-2 bg-gray-300 rounded-full bg-gray-600 w-24"></div>
                            </div>
                            <div class="flex items-center w-full max-w-[400px]">
                                <div class="h-2.5 bg-gray-300 rounded-full bg-gray-600 w-full"></div>
                                <div class="h-2.5 ms-2 bg-gray-200 rounded-full bg-gray-700 w-80"></div>
                                <div class="h-2.5 ms-2 bg-gray-300 rounded-full bg-gray-600 w-full"></div>
                            </div>
                            <div class="flex items-center w-full max-w-[480px]">
                                <div class="h-2.5 ms-2 bg-gray-200 rounded-full bg-gray-700 w-full"></div>
                                <div class="h-2.5 ms-2 bg-gray-300 rounded-full bg-gray-600 w-full"></div>
                                <div class="h-2.5 ms-2 bg-gray-300 rounded-full bg-gray-600 w-24"></div>
                            </div>
                            <div class="flex items-center w-full max-w-[440px]">
                                <div class="h-2.5 ms-2 bg-gray-300 rounded-full bg-gray-600 w-32"></div>
                                <div class="h-2.5 ms-2 bg-gray-300 rounded-full bg-gray-600 w-24"></div>
                                <div class="h-2.5 ms-2 bg-gray-200 rounded-full bg-gray-700 w-full"></div>
                            </div>
                            <div class="flex items-center w-full max-w-[360px]">
                                <div class="h-2.5 ms-2 bg-gray-300 rounded-full bg-gray-600 w-full"></div>
                                <div class="h-2.5 ms-2 bg-gray-200 rounded-full bg-gray-700 w-80"></div>
                                <div class="h-2.5 ms-2 bg-gray-300 rounded-full bg-gray-600 w-full"></div>
                            </div>
                            <span class="sr-only">Loading...</span>
                        </div>
                    </div>
                }
            }>
                {move || match resource.get() {
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







