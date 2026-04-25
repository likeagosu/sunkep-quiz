use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // 현재 화면이 '홈'인지 '퀴즈'인지 상태를 관리합니다.
    let is_started = use_state(|| false);

    let on_start_click = {
        let is_started = is_started.clone();
        Callback::from(move |_| is_started.set(true))
    };

    html! {
        <div style="
            background-color: #282c34; 
            color: white; 
            min-height: 100vh; 
            display: flex; 
            flex-direction: column; 
            align-items: center; 
            justify-content: center;
            font-family: 'Courier New', Courier, monospace;
        ">
            if !*is_started {
                // --- 첫 화면 (Home) ---
                <div style="text-align: center; border: 4px solid #61afef; padding: 40px; background: #21252b;">
                    <h1 style="font-size: 3rem; margin-bottom: 20px; color: #e06c75;">{ "Sunkep Quiz" }</h1>
                    <p style="margin-bottom: 30px; color: #98c379;">{ "아치 리눅스 유저를 위한 Rust 퀴즈 에디션" }</p>

                    <button
                        onclick={on_start_click}
                        style="
                            padding: 15px 30px; 
                            font-size: 1.2rem; 
                            background-color: #61afef; 
                            color: #282c34; 
                            border: none; 
                            cursor: pointer;
                            font-weight: bold;
                        ">
                        { "퀴즈 시작하기" }
                    </button>
                </div>
            } else {
                // --- 퀴즈 화면 (나중에 여기에 문제를 넣을 거예요) ---
                <div style="text-align: center;">
                    <h2>{ "문제 1: Rust의 소유권 규칙 중 틀린 것은?" }</h2>
                    <button style="display: block; margin: 10px auto; padding: 10px 20px;">{ "1. 한 값의 소유자는 하나뿐이다." }</button>
                    <button style="display: block; margin: 10px auto; padding: 10px 20px;">{ "2. 소유자가 스코프를 벗어나면 값은 버려진다." }</button>

                    <button
                        onclick={move |_| is_started.set(false)}
                        style="margin-top: 50px; background: none; color: #e06c75; border: 1px solid #e06c75; cursor: pointer;">
                        { "처음으로 돌아가기" }
                    </button>
                </div>
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
