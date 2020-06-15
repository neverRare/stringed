use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Interpretter {
    interpretter: stringed_core::Interpretter<
        Box<dyn Fn() -> String>,
        Box<dyn Fn(&str) -> ()>,
    >,
}
#[wasm_bindgen]
impl Interpretter {
    pub fn new(input: js_sys::Function, output: js_sys::Function) -> Self {
        Self {
            interpretter: stringed_core::Interpretter::new(
                Box::new(move || input.call0(&JsValue::NULL).unwrap().as_string().unwrap()),
                Box::new(move |string| {
                    output.call1(&JsValue::NULL, &JsValue::from_str(string)).unwrap();
                }),
            ),
        }
    }
    pub fn run(&mut self, code: &str) -> String {
        match self.interpretter.run(code) {
            Ok(_) => "".to_string(),
            Err(reason) => reason,
        }
    }
}
