use leptos::prelude::*;

#[component]
fn ProgressBar(
    #[prop(default = 100)] max: u32,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
        max=max
        value=progress
        />
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    let exp_count = move || count.get() * count.get();
    let on_click_reset = move |_| set_count.set(0);
    view! {
        <button on:click=move |_| { *set_count.write() += 1 }>"click me:" {count}</button>
        <p>"double count:" {double_count}</p>
        <button on:click=on_click_reset>"reset"</button>
        <ProgressBar
        progress=count/>
        <ProgressBar progress=Signal::derive(double_count)/>
        <ProgressBar progress=Signal::derive(exp_count)/>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> })
}
