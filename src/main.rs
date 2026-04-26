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
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 25px 40px; background: #21252b; border-bottom: 1px solid #3e4451;">
                <div style="display: flex; gap: 25px;">
                    <button onclick={go_scene(Scene::Home)}
                        style="background: none; color: #abb2bf; border: none; cursor: pointer; font-size: 1.1rem; font-family: 'GmarketSansBold'; transition: 0.3s;">
                        { "🏠 Home" }
                    </button>
                    <button onclick={go_scene(Scene::Quiz)}
                        style="background: none; color: #abb2bf; border: none; cursor: pointer; font-size: 1.1rem; font-family: 'GmarketSansBold'; transition: 0.3s;">
                        { "📝 Quiz" }
                    </button>
                    <button onclick={go_scene(Scene::Community)}
                        style="background: none; color: #abb2bf; border: none; cursor: pointer; font-size: 1.1rem; font-family: 'GmarketSansBold'; transition: 0.3s;">
                        { "💬 Community" }
                    </button>
                </div>
                <div style="display: flex; gap: 12px; align-items: center;">
                    <a href="https://www.instagram.com/sun_kep" class="sns-box" target="_blank">
                        <img src="../assets/Instagram_icon.png" style="width: 20px; height: 20px;" />
                        <span style="color: #abb2bf; font-size: 0.85rem; font-family: 'Pretendard';">
                            { "공식 인스타그램" }
                        </span>
                    </a>

                    <a href="https://www.youtube.com/@%EC%A0%84%EB%A0%A5%EC%9D%84_%EC%95%84%EA%BB%B4%EC%9A%94" class="sns-box" target="_blank">
                        <img src="../assets/Youtube_logo.png" style="width: 20px; height: 20px;" />
                        <span style="color: #abb2bf; font-size: 0.85rem; font-family: 'Pretendard';">
                            { "공식 유튜브" }
                        </span>
                    </a>
                </div>
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
