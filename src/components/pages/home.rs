use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{ "Home page" }</h1>
            <div>{"Home page body text"}</div>
        </div>
    }
}
