use leptos::*;
use leptos_router::A;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="flex align-center bg-slate-700">
            <A href="/" class="m-1">
                <img src="/assets/logo.webp" class="rounded-full" alt="Logo" height="64" width="64"/>
            </A>
            <div class="flex justify-end grow">
                <A
                    href="/"
                    class="flex items-center text-gray-300 hover:text-gray-300 m-4 font-bold aria-current_page:text-gray-100"
                >
                    Home
                </A>
                <A
                    href="/projects"
                    class="flex items-center text-gray-300 hover:text-gray-300 m-4 font-bold aria-current_page:text-gray-100"
                >
                    Projects
                </A>
            </div>
        </nav>
    }
}
