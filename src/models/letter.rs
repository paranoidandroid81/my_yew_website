use yew::prelude::*;

pub struct Letter {
    pub val: String,
    pub class: String,
}

pub enum UpdateFromGuess {
    Incorrect,
    CorrectWithPlace,
    CorrectWithoutPlace,
}

#[derive(PartialEq, Properties)]
pub struct LetterProps {
    pub letter: String,
}


impl Component for Letter {
    type Message = UpdateFromGuess;
    type Properties = LetterProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {val: ctx.props().letter.clone(), class: "newLetter".to_string()}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={&self.class}>
                if self.class != "newLetter" {
                    {&self.val}
                }
            </div>
        }
    }
}