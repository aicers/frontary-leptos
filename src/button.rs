#[allow(unused_imports)]
use leptos::prelude::IntoMaybeErased;
use leptos::{IntoView, attr::global::ClassAttribute, component, prelude::ElementChild, view};

// According to https://github.com/leptos-rs/leptos/issues/3172,
// the current leptos version looks having issues with `#[must_use_candidate]`.
#[allow(clippy::must_use_candidate)]
#[component]
pub fn Button<'a>(label: &'a str) -> impl IntoView {
    view! {
        // Wrapper with vertical spacing between buttons
        <div class="flex flex-col gap-2">

            // Method 1: Use Tailwind utility classes directly
            <button class="bg-primary text-white font-sans px-4 py-2 rounded-xl">
                {label} " (utility)"
            </button>

            // Method 2: Use a custom class defined in input.frontary.css
            <button class="btn-primary">
                {label} " (custom)"
            </button>

        </div>
    }
}
