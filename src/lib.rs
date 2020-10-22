use wasm_bindgen::prelude::*;

mod rotate;

pub use rotate::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn rotate_right(vec_input: &JsValue, steps: i32) -> JsValue {
    let vec_input_ser: Vec2<i32> = vec_input.into_serde().expect("Error de-serializing");
    web_sys::console::log_1(&"serializing complete".into());
    let rotated = (0..steps).fold(vec_input_ser, |v, _| rotate_right_2d(v));
    web_sys::console::log_1(&"rotation complete".into());
    return JsValue::from_serde(&rotated).unwrap();
}
