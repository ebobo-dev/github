use sycamore::prelude::*;
use web_sys::{wasm_bindgen::UnwrapThrowExt, WebSocket};

#[component]
pub async fn Dashboard<G: Html>() -> View<G> {
    let ws = WebSocket::new("ws://ebobo.shuttleapp.rs/fight").unwrap_throw();
    let message = create_signal(String::new());

    view! {
        p { "welcome to the arena" }

        button(on:click=move |_| {
            ws.send_with_str("").unwrap_throw(); // TODO: send message
        }) { "matchmake" }
        
        div { 
           (message.get_clone())
        }
    }
}
