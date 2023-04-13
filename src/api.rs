use wasm_bindgen::prelude::*;
use spin_sleep::LoopHelper;

#[wasm_bindgen]
pub async fn test(f: &js_sys::Function) -> Result<JsValue, JsValue>   {

  let this = JsValue::null();
  for i in 0..10000000 {
    println!("{}", i)
  }
  f.call1(&this, &JsValue::from(String::from("ss")));

  return Ok(this)
}

#[wasm_bindgen]
pub async fn test2(f: &js_sys::Function) -> Result<JsValue, JsValue> {
  let this = JsValue::null();

  f.call1(&this, &JsValue::from(String::from("ss2")));

  return Ok(this)
}
