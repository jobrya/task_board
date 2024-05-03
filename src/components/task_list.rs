use yew::prelude::*;
use yew::{function_component, html, Html};

#[derive(Clone, PartialEq)]
pub struct Task {
    pub id: usize,
    pub text: String,
    pub toggle: bool,
}

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    pub tasks: Vec<Task>,
}

#[function_component(TaskList)]
pub fn item_list(TaskListProps { tasks }: &TaskListProps) -> Html {
    tasks.iter().map(|task| html! {
        <ul key={task.id}>{format!("{}",task.text)}</ul>
    }).collect()
}