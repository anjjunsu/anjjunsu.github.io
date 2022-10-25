use crate::components::pages::{
    contact::Contact, experience::Experience, home::Home, junsu::Junsu, projects::Projects,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/junsu")]
    Junsu,
    #[at("/experience")]
    Experience,
    #[at("/projects")]
    Projects,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Junsu => html! { <Junsu /> },
        Route::Experience => html! { <Experience /> },
        Route::Projects => html! { <Projects /> },
        Route::Contact => html! { <Contact /> },
        Route::NotFound => html! { <h1>{"404: Not Found"}</h1> },
    }
}
