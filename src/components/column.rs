use yew::{function_component, html, Html};

#[function_component(Column)]
pub fn column() -> Html {
    html! { 
        <div>
            <h2> {"column header"} </h2>
            <ul> {"list element"} </ul>
            <ul> {"list element"} </ul>
            <ul> {"list element"} </ul>
        </div> 
    }
}