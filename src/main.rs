use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main>
            <h1>{ "Jeongwoo's Rust Quiz" }</h1>
            <p>{ "준비되셨나요? 배포 가즈아!" }</p>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
