use stringed_core::gen_interpretter::{GenInterpretter, Output as GenOutput};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum OutputStatus {
    Output = 0,
    Input = 1,
    Error = 3,
    Done = 4,
}
#[wasm_bindgen]
pub struct Output {
    pub status: OutputStatus,
    pub value: Option<String>,
}
#[wasm_bindgen]
pub struct Interpretter {
    interpretter: GenInterpretter,
}
#[wasm_bindgen]
impl Interpretter {
    pub fn start(code: &str) -> Self {
        Self {
            interpretter: GenInterpretter::start(code),
        }
    }
    pub fn next(&mut self, input: Option<&str>) -> Output {
        match self.interpretter.next(input) {
            GenOutput::Output(output) => Output {
                status: OutputStatus::Output,
                value: Some(output),
            },
            GenOutput::Input => Output {
                status: OutputStatus::Input,
                value: None,
            },
            GenOutput::Error(reason) => Output {
                status: OutputStatus::Error,
                value: Some(reason),
            },
            GenOutput::Done => Output {
                status: OutputStatus::Done,
                value: None,
            },
        }
    }
}
