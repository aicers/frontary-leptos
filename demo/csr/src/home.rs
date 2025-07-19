use frontary_leptos::Button;
use leptos::{IntoView, component, prelude::ElementChild, view};

#[component]
pub fn HomeApp() -> impl IntoView {
    view! {
        <div>
            <h1>"Home App"</h1>
            <Button label="Click me" />
        </div>
    }
}
