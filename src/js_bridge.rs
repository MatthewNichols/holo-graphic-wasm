
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen(module = "/js/canvas-writer.js")]
extern {
    pub fn sayHello(name: &str);
    pub fn init(canvasId: &str, height: i32, width: i32);
    pub fn clear(colorCode: &str);
    pub fn drawCircle(centerX: i32, centerY: i32, radius: i32, colorR: u8, colorG: u8,colorB: u8, colorA: f32);
}