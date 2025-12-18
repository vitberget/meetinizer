use leptos::leptos_dom::logging::{console_debug_log, console_log};
use leptos::prelude::*;
use leptos::task::spawn_local;

fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    spawn_local(async {
        console_log("Send it!");

        let host = location().host().unwrap_or("http".to_string());
        let protocol = location().protocol().unwrap_or("localhost:8080".to_string());
        let url = format!("{protocol}//{host}");

        match reqwest::get(&url).await {
            Ok(response) => {
                console_log("Response!");
                console_debug_log(&format!("{status}", status = response.status()));
            }
            Err(e) => { console_debug_log(&format!("Err {e}")); }
        }
    });

    console_log("Hello");
    view! {
        <button
            on:click=move |_| set_count.update(|cnt| *cnt += 1)
            >
            "Click me: " {count}
        </button>
            <div>
            "Double up " {move || count.get() *2}
        </div>
    }
}
