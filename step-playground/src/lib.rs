use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
    log("Hello, STEP!");
}

#[wasm_bindgen]
pub fn compile() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let p = document.get_element_by_id("syntax_tree").unwrap();
    p.set_inner_html("Rewrite from wasm");
}
