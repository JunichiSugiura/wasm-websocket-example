use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

macro_rules! console_log {
    ( $( $t:tt)* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct WebSocketClient {
    ws: WebSocket,
}

#[wasm_bindgen]
impl WebSocketClient {
    #[wasm_bindgen(constructor)]
    pub fn new(url: &str) -> WebSocketClient {
        let ws = WebSocket::new(url).expect("WebSocket:: Failed to initialize WebSocket");

        let handle_close = Closure::wrap(Box::new(move |_| {
            console_log!("WebSocket: CLOSE");
        }) as Box<dyn FnMut(JsValue)>);
        ws.set_onclose(Some(handle_close.as_ref().unchecked_ref()));
        handle_close.forget();

        let handle_error = Closure::wrap(Box::new(move |e: ErrorEvent| {
            console_log!("WebSocket: Error: {:?}", e);
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(handle_error.as_ref().unchecked_ref()));
        handle_error.forget();

        let handle_message = Closure::wrap(Box::new(move |e: MessageEvent| {
            let response = e
                .data()
                .as_string()
                .expect("WebSocket: Can't convert received data to string");

            console_log!("WebSocket: Message: {:?}", response);
        }) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(handle_message.as_ref().unchecked_ref()));
        handle_message.forget();

        let handle_open = Closure::wrap(Box::new(move |_| {
            console_log!("WebSocket: CONNECT");
        }) as Box<dyn FnMut(JsValue)>);
        ws.set_onopen(Some(handle_open.as_ref().unchecked_ref()));
        handle_open.forget();

        WebSocketClient { ws }
    }

    pub fn send(&self, message: &str) {
        if self.is_open() {
            &self
                .ws
                .send_with_str(message)
                .expect("WebSocket: Failed to send message");
        }
    }

    pub fn close(&self) {
        &self.ws.close();
    }
}

#[wasm_bindgen]
impl WebSocketClient {
    fn is_open(&self) -> bool {
        self.ws.ready_state() == WebSocket::OPEN
    }
}
