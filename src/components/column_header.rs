use yew::{function_component, html, Html};

#[function_component(ColumnHeader)]
pub fn column_header() -> Html {
    html! { 
        <h2> {"column header"} </h2>
    }
}