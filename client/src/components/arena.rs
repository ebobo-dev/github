use sycamore::prelude::*;
use web_sys::{wasm_bindgen::UnwrapThrowExt, MessageEvent, WebSocket};

#[component]
pub async fn Dashboard<G: Html>() -> View<G> {
    let ws = WebSocket::new("ws://ebobo.shuttleapp.rs").unwrap_throw();
    let message = create_signal(String::new());

    view! {
        p { "welcome to the arena" }

        button(on:click=move |_| {
            ws.send_with_str("").unwrap_throw();
        }) { "matchmake" }
        
        div { 
           (message.get_clone())
        }
    }
}
