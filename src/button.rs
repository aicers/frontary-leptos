use leptos::{IntoView, component, prelude::ElementChild, view};

// According to https://github.com/leptos-rs/leptos/issues/3172,
// the current leptos version looks having issues with `#[must_use_candidate]`.
#[allow(clippy::must_use_candidate)]
#[component]
pub fn Button<'a>(label: &'a str) -> impl IntoView {
    view! {
        <button>
            {label}
        </button>
    }
}
