use wasm_bindgen::{JsCast, JsValue};

pub(crate) fn jscast<T, U>(v: T) -> Result<U, JsValue>
where
    T: JsCast + std::fmt::Debug,
    U: JsCast,
{
    v.dyn_into::<U>().map_err(|orig| {
        JsValue::from_str(&format!(
            "Failed to cast {:?} into {}",
            orig,
            std::any::type_name::<U>()
        ))
    })
}
