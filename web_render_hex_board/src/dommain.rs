use crate::domcall::dom_call;
use crate::jscast;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = dom_call!(web_sys::window())?;
    let document = dom_call!(window.document())?;
    let body = dom_call!(document.body())?;

    let canvas: HtmlCanvasElement = jscast(document.create_element("canvas")?)?;
    let ctx: CanvasRenderingContext2d = jscast(dom_call!(canvas.get_context("2d")?)?)?;
    ctx.rect(10f64, 10f64, 30f64, 20f64);
    ctx.stroke();

    body.append_child(&canvas)?;

    dom_call!(document.get_element_by_id("loading"))?.remove();
    Ok(())
}
