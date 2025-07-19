mod home;
use home::HomeApp;
use leptos::{mount::mount_to_body, view};

fn main() {
    mount_to_body(|| view! { <HomeApp /> });
}
