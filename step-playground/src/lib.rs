use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlElement;

#[wasm_bindgen(start)]
pub fn run() {
    let compile = Closure::wrap(Box::new(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let p = document.get_element_by_id("syntax_tree").unwrap();
        p.set_inner_html("From closure!!");
    }) as Box<dyn FnMut()>);

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document
        .get_element_by_id("button")
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(compile.as_ref().unchecked_ref()));

    compile.forget();
}
