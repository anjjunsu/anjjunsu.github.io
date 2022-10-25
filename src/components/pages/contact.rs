use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div>
            <h1>{ "Contact" }</h1>
            <div>{"Contact body text"}</div>
        </div>
    }
}
