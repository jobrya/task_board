use yew::prelude::*;

mod components;

use components::{
    project_header::ProjectHeader,
    board::Board,
};

#[function_component]
fn App() -> Html {
    html! {
        <main>
            <ProjectHeader/>
            <Board/>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}