use crate::domcall::dom_call;
use crate::jscast;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::HtmlCanvasElement;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = dom_call!(web_sys::window())?;
    let document = dom_call!(window.document())?;
    let body = dom_call!(document.body())?;

    let canvas: HtmlCanvasElement = jscast(document.create_element("canvas")?)?;

    body.append_child(&canvas)?;

    Ok(())
}
