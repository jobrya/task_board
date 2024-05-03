use yew::{function_component, html, Html};
use crate::components::{
    task_list::TaskList,
    task_list::Task,
    column_header::ColumnHeader
};

#[function_component(Column)]
pub fn column() -> Html {
    
    let tasks = vec![
        Task {
            id: 1,
            text: String::from("task text"),
            toggle: true,
        },
    ];


    html! { 
        <div class = "column">
            <ColumnHeader/>
            <TaskList tasks = {tasks}/>
        </div> 
    }
}