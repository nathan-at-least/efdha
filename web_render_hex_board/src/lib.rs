use wasm_bindgen::prelude::*;

macro_rules! dom_call {
    ( $x:expr ) => {
        match $x {
            Some(v) => Ok(v),
            None => Err(JsValue::from_str(&format!(
                "DOM Error: {} returned None",
                stringify!($x)
            ))),
        }
    };
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = dom_call!(web_sys::window())?;
    let document = dom_call!(window.document())?;
    let body = dom_call!(document.body())?;

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;

    Ok(())
}
