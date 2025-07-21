use leptos::{
    IntoView,
    attr::global::{ClassAttribute, OnAttribute},
    component,
    prelude::{ElementChild, Get, Update, signal},
    view,
};

// According to https://github.com/leptos-rs/leptos/issues/3172,
// the current leptos version looks having issues with `#[must_use_candidate]`.
#[allow(clippy::must_use_candidate)]
#[component]
pub fn Button<'a>(label: &'a str) -> impl IntoView {
    let (count_left, set_count_left) = signal(0);
    let (count_right, set_count_right) = signal(0);

    view! {
        // Wrapper with vertical spacing between buttons
        <div class="flex flex-col gap-2">
            <p> "Count Left: " { move || count_left.get() } </p>
            <p> "Count Right: " { move || count_right.get() } </p>

            // Method 1: Use Tailwind utility classes directly
            <button class="bg-primary text-white font-sans px-4 py-2 rounded-xl"
                on:click=move |_| { set_count_left.update(|v| *v += 1); }
            >
                {label} " (utility)"
            </button>

            // Method 2: Use a custom class defined in input.frontary.css
            <button class="btn-primary"
                on:click=move |_| { set_count_right.update(|v| *v += 1); }
            >
                {label} " (custom)"
            </button>

        </div>
    }
}
