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

    link.set_attribute("href", &msg).unwrap();
    link.set_inner_html(EMAIL);
    footer.append_child(&link).unwrap();
    body.append_child(&footer).unwrap();

    Ok(())
}
