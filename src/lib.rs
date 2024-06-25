use wasm_bindgen::prelude::*;

// Web ke console mein message print karne ke liye
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Ye function "Hello, world!" return karta hai
fn hello_world() -> String {
    "Hello, world!".to_string()
}

#[wasm_bindgen]
pub fn run_main() {
    let message = hello_world();
    log(&message);  // Console mein message print karta hai
}