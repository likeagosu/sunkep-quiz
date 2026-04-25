use yew::prelude::*;

// 퀴즈 데이터 구조체
struct QuizItem {
    question: String,
    options: Vec<String>,
    answer_idx: usize,
}

#[function_component(App)]
fn app() -> Html {
    // 1. 퀴즈 데이터 벡터 생성
    let quiz_data = vec![
        QuizItem {
            question: "Rust에서 불변 변수를 선언하는 키워드는?".to_string(),
            options: vec!["let".into(), "let mut".into(), "const".into()],
            answer_idx: 0,
        },
        QuizItem {
            question: "아치 리눅스의 기본 패키지 관리자는?".to_string(),
            options: vec!["apt".into(), "pacman".into(), "dnf".into()],
            answer_idx: 1,
        },
    ];

    // 현재 문제 번호 상태 (0부터 시작)
    let current_idx = use_state(|| 0);

    html! {
        <div style="background-color: #282c34; color: white; min-height: 100vh; padding: 20px;">
            <h1 style="text-align: center;">{ "Jeongwoo's Rust Quiz" }</h1>

            {
                // 현재 번호에 해당하는 문제 가져오기
                if let Some(item) = quiz_data.get(*current_idx) {
                    html! {
                        <div style="max-width: 500px; margin: 0 auto; border: 2px solid #61afef; padding: 20px;">
                            <h3>{ format!("Q{}. {}", *current_idx + 1, item.question) }</h3>

                            // 2. 벡터(options)를 반복문(iter)으로 돌려서 버튼 생성
                            <div style="display: flex; flex-direction: column; gap: 10px;">
                                {
                                    item.options.iter().enumerate().map(|(i, option)| {
                                        html! {
                                            <button style="padding: 10px; cursor: pointer; background: #3e4451; color: white; border: 1px solid #5c6370;">
                                                { format!("{}. {}", i + 1, option) }
                                            </button>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        </div>
                    }
                } else {
                    html! { <h2 style="text-align: center;">{ "모든 문제를 풀었습니다!" }</h2> }
                }
            }
        </div>
    }
}
