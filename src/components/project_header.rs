use yew::{function_component, html, Html};

#[function_component(ProjectHeader)]
pub fn project_header() -> Html {
    html! { 
        <div>
            <h2> {"project header"} </h2>
        </div> 
    }
}