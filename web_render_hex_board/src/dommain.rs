use crate::domcall::dom_call;
use crate::{jscast, log};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let rend = Renderer::init()?;
    log!("Initialized: {:#?}", &rend);
    rend.rerender();
    dom_call!(rend.doc.get_element_by_id("loading"))?.remove();
    Ok(())
}

#[derive(Debug)]
struct Renderer {
    doc: Document,
    body: HtmlElement,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

impl Renderer {
    fn init() -> Result<Self, JsValue> {
        log!("Renderer::init");

        let window = dom_call!(web_sys::window())?;
        let doc = dom_call!(window.document())?;
        let body = dom_call!(doc.body())?;
        let canvas: HtmlCanvasElement = jscast(doc.create_element("canvas")?)?;
        let ctx: CanvasRenderingContext2d = jscast(dom_call!(canvas.get_context("2d")?)?)?;

        body.append_child(&canvas)?;

        Ok(Renderer {
            doc,
            body,
            canvas,
            ctx,
        })
    }

    fn rerender(&self) {
        let w = self.body.client_width();
        let h = self.body.client_height();
        self.canvas.set_width(u32::try_from(w).unwrap());
        self.canvas.set_height(u32::try_from(h).unwrap());

        self.ctx.rect(
            0f64,
            0f64,
            f64::try_from(w).unwrap() * 0.98,
            f64::try_from(h).unwrap() * 0.90,
        );
        self.ctx.stroke();
        self.ctx.rect(10f64, 10f64, 30f64, 20f64);
        self.ctx.stroke();
    }
}
