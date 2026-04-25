use gloo_timers::callback::Timeout;
use yew::prelude::*;

// 퀴즈 데이터 구조체
struct QuizItem {
    question: String,
    options: Vec<String>,
    answer_idx: usize,
}

#[function_component(App)]
fn app() -> Html {
    let current_idx = use_state(|| 0);
    let score = use_state(|| 0);
    let message = use_state(|| "".to_string());

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

    html! {
        <div style="background-color: #282c34; color: white; min-height: 100vh; padding: 20px;">
            <h1 style="text-align: center;">{ "SunKep 에너지 절약 퀴즈" }</h1>

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
                                        let current_idx = current_idx.clone();
                                        let score = score.clone();
                                        let message = message.clone();

                                        let correct_answer = item.answer_idx;

                                        let on_click = Callback::from(move |_| {
                                            if correct_answer == i {
                                                score.set(*score + 10);
                                                message.set("정답입니다! +10".to_string());
                                            } else {
                                                message.set(format!{"오답입니다.. 정답은 {}번 입니다.", correct_answer+1}.to_string());
                                            }

                                            let m_handle = message.clone();
                                            let c_handle = current_idx.clone();
                                            gloo_timers::callback::Timeout::new(2000, move || {
                                                m_handle.set("".to_string());    // 메시지 지우기
                                                c_handle.set(*c_handle + 1);    // 다음 문제로 넘기기
                                            }).forget();
                                        });
                                        html! {
                                            <button onclick={on_click} style="padding: 10px; cursor: pointer; background: #3e4451; color: white; border: 1px solid #5c6370;">
                                                { format!("{}. {}", i + 1, option) }
                                            </button>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                            <div style="
                                margin-top: 30px;
                                padding: 15px;
                                text-align: center;
                                font-size: 1.2rem;
                                font-weight: bold;
                                color: #dabe71; /* 눈에 띄는 노란색 계열 */
                                min-height: 3em; /* 메시지가 없을 때도 자리를 차지하게 해서 화면 덜컹거림 방지 */
                                ">
                                { (*message).clone() }
                            </div>
                        </div>
                    }
                } else {
                    html! { <h2 style="text-align: center;">{ format!("모든 문제를 풀었습니다! 점수는 {}점 입니다!", *score) }</h2> }
                }
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
