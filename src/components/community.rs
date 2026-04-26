use crate::types::Post;
use gloo_net::http::Request;
use web_sys::HtmlTextAreaElement;
use yew::platform::spawn_local;
use yew::prelude::*;

const SUPABASE_URL: &str = "https://kyzgtmejohhyzmbhiqmv.supabase.co/rest/v1/posts";
const ANON_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Imt5emd0bWVqb2hoeXptYmhpcW12Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3NzcxMzQ3NzEsImV4cCI6MjA5MjcxMDc3MX0.0nO70At6Yp5KkpdoHirpgOWUx7vI00oMNhnuAr4jNMM";

async fn fetch_posts() -> Vec<Post> {
    let response = Request::get(SUPABASE_URL)
        .header("apikey", ANON_KEY)
        .header("Authorization", &format!("Bearer {}", ANON_KEY))
        .send()
        .await
        .unwrap();
    response.json::<Vec<Post>>().await.unwrap_or_default()
}

async fn save_post(content: String) {
    let post = Post { content };
    let _ = Request::post(SUPABASE_URL)
        .header("apikey", ANON_KEY)
        .header("Authorization", &format!("Bearer {}", ANON_KEY))
        .header("Content-Type", "application/json")
        .json(&post)
        .unwrap()
        .send()
        .await;
}

#[function_component(Community)]
pub fn community() -> Html {
    let input_value = use_state(|| String::new());
    let posts = use_state(|| vec![]);

    // 초기 로딩
    {
        let posts = posts.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                posts.set(fetch_posts().await);
            });
            || ()
        });
    }

    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlTextAreaElement = e.target_unchecked_into();
            input_value.set(target.value());
        })
    };

    let on_submit = {
        let input_value = input_value.clone();
        let posts = posts.clone();
        Callback::from(move |_| {
            let content = (*input_value).clone();
            let posts = posts.clone();
            if !content.is_empty() {
                spawn_local(async move {
                    save_post(content).await;
                    posts.set(fetch_posts().await);
                });
                input_value.set(String::new());
            }
        })
    };

    html! {
        <div style="max-width: 800px; margin: 0 auto; padding: 20px;">
            <h2 style="color: #61afef;">{ "귀찮음을 넘는 단 한 번의 행동들" }</h2>

            <div style="background: #282c34; border: 1px solid #3e4451; padding: 20px; border-radius: 8px; margin-bottom: 30px;">
                <textarea
                    value={(*input_value).clone()}
                    oninput={on_input}
                    placeholder="오늘 당신의 작은 실천을 기록해 보세요..."
                    style="width: 100%; height: 80px; background: transparent; border: none; color: white; outline: none; resize: none;"
                />
                <div style="display: flex; justify-content: flex-end;">
                    <button onclick={on_submit} style="padding: 8px 20px; background: #61afef; color: #282c34; border: none; border-radius: 4px; font-weight: bold; cursor: pointer;">
                        { "기록하기" }
                    </button>
                </div>
            </div>

            <div style="display: flex; flex-direction: column; gap: 15px;">
                { for (*posts).iter().rev().map(|post| html! {
                    <div style="background: #21252b; border-left: 4px solid #98c379; padding: 15px; border-radius: 4px;">
                        <p style="margin-top: 5px; color: #dcdfe4; word-break: break-all; overflow-wrap: break-word;">{ &post.content }</p>
                    </div>
                }) }
            </div>
        </div>
    }
}
