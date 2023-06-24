use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    document.set_title("ebobo.dev");

    let header = document.create_element("h1").unwrap();
    header.set_inner_html("Paula Brillant!");
    body.append_child(&header).unwrap();

    let footer = document.create_element("footer").unwrap();
    let link = document.create_element("a").unwrap();

    const EMAIL: &str = "generated@ebobo.dev";

    let msg = format!("mailto:{}", EMAIL);

    let _ = link.set_attribute("href", &msg);
    link.set_inner_html(EMAIL);
    let _ = footer.append_child(&link);
    let _ = body.append_child(&footer);

    Ok(())
}
