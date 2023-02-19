use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn fib_num(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_num(n - 1) + fib_num(n - 2)
    }
}
