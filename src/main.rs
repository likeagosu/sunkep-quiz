mod components;
mod types;

use components::{community::Community, home::Home, quiz::Quiz};
use types::Scene;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let current_scene = use_state(|| Scene::Home);

    let go_scene = |s: Scene| {
        let scene = current_scene.clone();
        Callback::from(move |_| scene.set(s))
    };

    html! {
        <div style="background-color: #282c34; color: white; min-height: 100vh;">
            <nav style="background: none; color: white; border: none; cursor: pointer;">
                <button onclick={go_scene(Scene::Home)}>{ "🏠 Home" }</button>
                <button onclick={go_scene(Scene::Quiz)}>{ "📝 Quiz" }</button>
                <button onclick={go_scene(Scene::Community)}>{ "💬 Community" }</button>
            </nav>

            <main style="padding: 20px;">
                {
                    match *current_scene {
                        Scene::Home => html! { <Home /> },
                        Scene::Quiz => {
                            let raw_data = include_str!("../quiz.json");
                            let quiz_data: Vec<crate::types::QuizItem> = serde_json::from_str(raw_data).unwrap();
                            html! { <Quiz data={quiz_data} /> }
                        },
                        Scene::Community => html! { <Community /> },
                    }
                }
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
