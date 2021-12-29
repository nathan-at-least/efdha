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

pub(crate) use dom_call;
