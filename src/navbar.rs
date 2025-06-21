use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="flex grow align-center top-0 left-0 bg-slate-700">
            <A href="/" {..} class="m-1">
                <img
                    src="/assets/logo.webp"
                    class="rounded-full"
                    alt="Logo"
                    height="64"
                    width="64"
                />
            </A>
            <div class="flex justify-end grow">
                <A
                    href="/"
        {..}
                     class="flex items-center text-gray-300 hover:text-gray-300 m-4 font-bold aria-current_page:text-gray-100"
                >
                    Home
                </A>
                <A
                    href="/projects"
    {..}
                     class="flex items-center text-gray-300 hover:text-gray-300 m-4 font-bold aria-current_page:text-gray-100"
                >
                    Projects
                </A>
            </div>
        </nav>
    }
}
