use yew::{function_component, html, Html};
use crate::components::column::Column;

#[function_component(Board)]
pub fn board() -> Html {
    html! { 
        <div>
            <Column/>
            <Column/>
            <Column/>
        </div> 
    }
}