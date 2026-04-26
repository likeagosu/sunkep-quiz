use crate::types::QuizItem;
use gloo_timers::callback::Timeout;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct QuizProps {
    pub data: Vec<QuizItem>,
}

#[function_component(Quiz)]
pub fn quiz(props: &QuizProps) -> Html {
    let current_idx = use_state(|| 0);
    let score = use_state(|| 0);
    let message = use_state(|| "".to_string());
    let is_processing = use_state(|| false);

    if let Some(item) = props.data.get(*current_idx) {
        html! {
            <div>
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
                                if *processing { return; }

                                processing.set(true);
                                if correct_answer == i {
                                    s.set(*s + 10);
                                    msg.set("정답입니다! +10점!".to_string());
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
}
