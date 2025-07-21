use leptos::{
    IntoView,
    attr::global::{ClassAttribute, OnAttribute},
    component,
    control_flow::Show,
    prelude::{ElementChild, Get, GetUntracked, Set, signal},
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
            <Show when=move || true>
                <p>{ move || format!("Count Left: {}", count_left.get()) }</p>
            </Show>
            <Show when=move || true>
                <p>{ move || format!("Count Right: {}", count_right.get()) }</p>
            </Show>

            // <p>{ move || {
            //     let cnt = count_left.get();
            //     format!("Count Left: {cnt}")
            // }}</p>
            // <p>{ move || {
            //     let cnt = count_right.get();
            //     format!("Count Right: {cnt}")
            // }}</p>

            // Method 1: Use Tailwind utility classes directly
            <button class="bg-primary text-white font-sans px-4 py-2 rounded-xl"
                on:click=move |_| {set_count_left.set(count_left.get_untracked() + 1);}
            >
                {label} " (utility)"
            </button>

            // Method 2: Use a custom class defined in input.frontary.css
            <button class="btn-primary"
                on:click=move |_| {set_count_right.set(count_right.get_untracked() + 1);}
            >
                {label} " (custom)"
            </button>

        </div>
    }
}
