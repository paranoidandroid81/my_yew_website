use yew::prelude::*;
mod models;
use crate::models::letter::Letter;

#[function_component(App)]
fn app() -> Html {
    let mut letters: Vec<String> = vec!();
    letters.push("E".to_string());
    letters.push("C".to_string());
    letters.push("A".to_string());
    letters.push("F".to_string());

    html! {
        <>
            <div class="container">
                <h1>{ "Welcome to Michael's Web Page!!!" }</h1>
                <div class="allLetters">
                { 
                    letters.into_iter().map(|curr| {
                        html!{<Letter letter={curr}/>}
                    }).collect::<Html>()
                }
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
