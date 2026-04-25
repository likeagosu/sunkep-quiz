use gloo_timers::callback::Timeout;
use serde_json;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Scene {
    Home,
    Quiz,
    Community,
}

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
pub struct QuizItem {
    pub question: String,
    pub options: Vec<String>,
    pub answer_idx: usize,
}

// 별도의 모듈이나 함수로 퀴즈 데이터를 반환하게 만듭니다.
fn get_quiz_data() -> Vec<QuizItem> {
    vec![
        QuizItem {
            question: "Rust에서 불변 변수를 선언하는 키워드는?".into(),
            options: vec!["let".into(), "let mut".into(), "const".into()],
            answer_idx: 0,
        },
        QuizItem {
            question: "아치 리눅스의 기본 패키지 관리자는?".into(),
            options: vec!["apt".into(), "pacman".into(), "dnf".into()],
            answer_idx: 1,
        },
        // 여기에 문제를 계속 추가해도 main 코드는 깔끔하게 유지됩니다.
    ]
}

#[function_component(App)]
fn app() -> Html {
    // 1. 상태(State) 선언은 함수 최상단에 모읍니다.
    let current_scene = use_state(|| Scene::Home);
    let current_idx = use_state(|| 0);
    let score = use_state(|| 0);
    let message = use_state(|| "".to_string());
    let is_processing = use_state(|| false);

    let raw_data = include_str!("../quiz.json");
    let quiz_data: Vec<QuizItem> = serde_json::from_str(raw_data).unwrap();

    // 3. 네비게이션 콜백
    let go_home = {
        let scene = current_scene.clone();
        Callback::from(move |_| scene.set(Scene::Home))
    };
    let go_quiz = {
        let scene = current_scene.clone();
        Callback::from(move |_| scene.set(Scene::Quiz))
    };
    let go_community = {
        let scene = current_scene.clone();
        Callback::from(move |_| scene.set(Scene::Community))
    };

    html! {
        <div style="background-color: #282c34; color: white; min-height: 100vh;">
            // --- 상단 바 ---
            <nav style="display: flex; gap: 20px; padding: 20px; background: #21252b; border-bottom: 2px solid #61afef;">
                <button onclick={go_home} style="background: none; color: white; border: none; cursor: pointer;">{ "🏠 Home" }</button>
                <button onclick={go_quiz} style="background: none; color: white; border: none; cursor: pointer;">{ "📝 Quiz" }</button>
                <button onclick={go_community} style="background: none; color: white; border: none; cursor: pointer;">{ "💬 Community" }</button>
            </nav>

            <main style="padding: 20px;">
                {
                    match *current_scene {
                        Scene::Home => html! {
                            <div style="text-align: center; margin-top: 50px;">
                                <h1 class="sun-kep-title">{ "Sun-Kep community" }</h1>
                                <p class="sun-kep-desc">{ "귀찮음을 넘는 단 한 번의 행동" }</p>
                                <img src="sunkep.png"/>
                            </div>
                        },
                        Scene::Quiz => {
                            // 퀴즈 렌더링 로직
                            if let Some(item) = quiz_data.get(*current_idx) {
                                html! {
                                    <div style="max-width: 500px; margin: 0 auto; border: 2px solid #61afef; padding: 20px;">
                                        <h1 style="text-align: center;">{ "sunkep 에너지 절약 퀴즈" }</h1>
                                        <h3>{ format!("Q{}. {}", *current_idx + 1, item.question) }</h3>

                                        <div style="display: flex; flex-direction: column; gap: 10px;">
                                            {
                                                item.options.iter().enumerate().map(|(i, option)| {
                                                    let c_idx = current_idx.clone();
                                                    let s = score.clone();
                                                    let msg = message.clone();
                                                    let processing = is_processing.clone();
                                                    let correct_answer = item.answer_idx;

                                                    let on_click = Callback::from(move |_| {
                                                        if *processing {return;}

                                                        processing.set(true);
                                                        if correct_answer == i {
                                                            s.set(*s + 10);
                                                            msg.set("정답입니다! +10".to_string());
                                                        } else {
                                                            msg.set(format!("오답입니다.. 정답은 {}번 입니다.", correct_answer + 1));
                                                        }

                                                        let m_handle = msg.clone();
                                                        let c_handle = c_idx.clone();
                                                        let p_handle = processing.clone();
                                                        Timeout::new(2000, move || {
                                                            m_handle.set("".to_string());
                                                            c_handle.set(*c_handle + 1);
                                                            p_handle.set(false);
                                                        }).forget();
                                                    });

                                                    html! {
                                                        <button onclick={on_click} disabled={*is_processing} style="padding: 10px; cursor: pointer; background: #3e4451; color: white; border: 1px solid #5c6370;">
                                                            { format!("{}. {}", i + 1, option) }
                                                        </button>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </div>

                                        <div style="margin-top: 30px; text-align: center; color: #dabe71; min-height: 3em; font-weight: bold;">
                                            { (*message).clone() }
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {
                                    <div style="text-align: center;">
                                        <h2>{ format!("모든 문제를 풀었습니다! 점수는 {}점 입니다!", *score) }</h2>
                                        <button onclick={move |_| { current_idx.set(0); score.set(0); }} style="margin-top: 20px; padding: 10px;">
                                            { "다시 풀기" }
                                        </button>
                                    </div>
                                }
                            }
                        },
                        Scene::Community => html! {
                            <div style="text-align: center;">
                                <h2>{ "커뮤니티 게시판" }</h2>
                                <p style="color: #abb2bf;">{ "(DB 연결 후 실시간 채팅/게시글이 표시될 공간입니다)" }</p>
                                <div style="border: 1px dashed #5c6370; padding: 40px; margin-top: 20px;">
                                    { "현재 준비 중... 에너지 절약에 관심있는 모든 분들을 환영합니다!" }
                                </div>
                            </div>
                        },
                    }
                }
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
