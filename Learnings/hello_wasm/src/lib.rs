use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub fn show_message() {
    let document = window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    let val = document.create_element("h1").unwrap();
    val.set_inner_html("Hello, world from Rust!");

    body.append_child(&val).unwrap();
}
