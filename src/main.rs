use yew::prelude::*;

mod components;

use components::{
    column::Column,
    project_header::ProjectHeader,
};

#[function_component]
fn App() -> Html {
    html! {
        <main>
            <ProjectHeader/>
            <Column/>
            <Column/>
            <Column/>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}