use yew::{function_component, Html, html, use_state};

#[function_component]
fn HelloWorld() -> Html {
    html! {
        <div>
            <h1>{ "Hello, World!" }</h1>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;

            counter.set(value);
        }
    };

    html! {
        <div>
            <HelloWorld />

            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
