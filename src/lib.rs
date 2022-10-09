
use stylist::{yew::styled_component, style};
use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    let name = "Brooks";
    let obj = MyObject {
        username: name.to_owned(),
        favorite_language: "Rust".to_owned(),
    };

    log!(serde_json::to_string_pretty(&obj).unwrap());
    let class= "my_titles";
    let message = None::<&str>; //Some("I am a message");

    let tasks = vec!["sleep", "eat", "code", "repeat"];

    let style = style!(
        r#"
            color: black;
        "#
    ).unwrap();

    html! {
        <>
        <h1 class={style}>{ "Hello World!"}</h1>
        if class == "my_titles" {
            <p>{"Hello"}</p>
        }
        else {
            <p>{"Goodbye"}</p>
        }

        if let Some(message) = message {
            <p>{message}</p>
        }
        else {
            <p>{"No message"}</p>
        }

        <ul>
            {to_li(tasks)}
        </ul>

        </>
    }
}

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

fn to_li(task: Vec<&str>) -> Html {
    task.iter().map(|task| html!{<li>{task}</li>}).collect::<Html>()
}