use super::models::Game;
#[cfg(feature = "wasm")]
use wasm_bindgen::{prelude::*, JsCast};
#[cfg(feature = "wasm")]
use web_sys::{ErrorEvent, MessageEvent, WebSocket};


#[cfg(feature = "wasm")]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn create_game() -> JsValue {
    let new_game = Game { id: 1, state: 0 };

    return JsValue::from_serde(&new_game).unwrap();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn create_socket(host: String, port: u16) -> Result<(), JsValue> {
    // Connect to an game server
    let ws = WebSocket::new(&format!("ws://{}:{}/games/ws/", host, port))?;

    // create callback
    let cloned_ws = ws.clone();
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
         if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            console_log!("message event, received Text: {:?}", txt);
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }
    }) as Box<dyn FnMut(MessageEvent)>);

    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let onopen_callback = Closure::wrap(Box::new(move |_| {
        console_log!("socket opened");
        match cloned_ws.send_with_str("I'm not yet implemented") {
            Ok(_) => console_log!("message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}

// This is like the `main` function, except for JavaScript.
#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // Your code goes here!
    console_log!("Hello world!");

    Ok(())
}
