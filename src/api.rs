use wasm_bindgen::prelude::*;
use spin_sleep::LoopHelper;
use wasm_bindgen_futures::spawn_local;
use futures_channel::oneshot;
// wasm-pack build --target nodejs
#[wasm_bindgen]
pub async fn test() -> Result<JsValue, JsValue>   {
  let (tx, rx) = oneshot::channel::<u32>();
  let this = JsValue::null();
  spawn_local(async {
    testAsync().await;
    tx.send(1).unwrap();
  });
  return Ok(JsValue::from(rx.await.unwrap()))
}
pub async fn testAsync() {
  for i in 0..10000000 {
    println!("{}", i)
  }
}
#[wasm_bindgen]
pub async fn test2() -> Result<JsValue, JsValue> {
  let this = JsValue::from(2);
  return Ok(this)
}
