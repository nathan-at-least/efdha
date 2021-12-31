macro_rules! log {
    ( $msg:expr ) => {
        $crate::log::log_str($msg);
    };

    ( $tmpl:expr, $( $arg:expr ),* ) => {
        $crate::log::log_str(format!($tmpl, $( $arg ),* ));
    }
}

pub(crate) use log;

pub(crate) fn log_str<S>(msg: S)
where
    S: AsRef<str>,
{
    let s = msg.as_ref();
    let jsv = wasm_bindgen::JsValue::from_str(s);
    web_sys::console::log_1(&jsv);
}
