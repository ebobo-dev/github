use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Set the title
    document.set_title("ebobo.dev");

    // Create the header
    let header = document.create_element("h1").unwrap();
    header.set_inner_html("Paula Brillant");
    body.append_child(&header).unwrap();

    // Create the footer
    let footer: web_sys::Element = document.create_element("footer").unwrap();
    let link: web_sys::Element = document.create_element("a").unwrap();

    let email: &str = "generated@ebobo.dev";

    let msg = format!("mailto:{}", email);

    let _ = link.set_attribute("href", &msg);
    link.set_inner_html(email);
    let _ = footer.append_child(&link);
    let _ = body.append_child(&footer);

    Ok(())
}
