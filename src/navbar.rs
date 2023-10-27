use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="flex align-center bg-slate-700">
            <a href="/" class="m-1">
            <img src="/assets/logo.jpg" class="rounded-full" alt="Logo" height="64" width="64"/>
            </a>
            <div class="flex justify-end grow">
                <a href="/" class="flex items-center text-gray-400 hover:text-gray-300 m-4 font-bold">Home</a>
            </div>
        </nav>
    }
}
