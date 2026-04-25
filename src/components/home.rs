use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div style="text-align: center; margin-top: 50px;">
            <h1 class="sun-kep-title" style="color: #61afef; font-size: 3rem;">
                { "Sun-Kep community" }
            </h1>
            <p class="sun-kep-desc" style="color: #abb2bf; font-size: 1.2rem;">
                { "귀찮음을 넘는 단 한 번의 행동" }
            </p>
            <div style="margin-top: 30px;">
                <img src="sunkep.png" alt="Sun-Kep Logo" style="max-width: 200px;" />
            </div>
        </div>
    }
}
