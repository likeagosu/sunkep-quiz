use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // 점수 상태를 관리합니다 (초기값 0)
    let score = use_state(|| 0);

    // 버튼을 눌렀을 때 실행될 함수
    let onclick = {
        let score = score.clone();
        Callback::from(move |_| {
            score.set(*score + 10); // 버튼 누르면 10점씩 증가!
        })
    };

    html! {
        <main style="text-align: center; font-family: 'Courier New', Courier, monospace;">
            <h1 style="color: #61afef;">{ "Rust Quiz Academy" }</h1>

            <div style="border: 2px solid #abb2bf; padding: 20px; margin: 20px; display: inline-block;">
                <p>{ "질문: Rust에서 가변 변수를 선언할 때 사용하는 키워드는?" }</p>

                <button {onclick} style="padding: 10px 20px; cursor: pointer;">
                    { "클릭해서 정답 확인 (및 점수 획득!)" }
                </button>
            </div>

            <h2>{ format!("현재 점수: {}점", *score) }</h2>

            <p style="font-size: 0.8em; color: gray;">
                { "정우 에디션 v0.1.0 - Arch Linux에서 컴파일됨" }
            </p>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
