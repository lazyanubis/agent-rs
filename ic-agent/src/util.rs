#![allow(dead_code)]

use std::future::Future;
use std::time::Duration;

#[allow(unused_variables)]
pub async fn sleep(d: Duration) {
    #[cfg(not(all(target_family = "wasm", feature = "wasm-bindgen")))]
    tokio::time::sleep(d).await;
    /*
    #[cfg(all(target_family = "wasm", feature = "wasm-bindgen"))]
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |rs, rj| {
        if let Err(e) = web_sys::window()
            .expect("global window unavailable")
            .set_timeout_with_callback_and_timeout_and_arguments_0(&rs, d.as_millis() as _)
        {
            use wasm_bindgen::UnwrapThrowExt;
            rj.call1(&rj, &e).unwrap_throw();
        }
    }))
    .await
    .expect("unable to setTimeout");
    */

    // still panic
    // #[cfg(all(target_family = "wasm", feature = "wasm-bindgen"))]
    // wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |rs, rj| {
    //     if let Err(e) = js_sys::global()
    //         .get_window()
    //         .expect("global window unavailable")
    //         .set_timeout_with_callback_and_timeout_and_arguments_0(&rs, d.as_millis() as _)
    //     {
    //         use wasm_bindgen::UnwrapThrowExt;
    //         rj.call1(&rj, &e).unwrap_throw();
    //     }
    // }))
    // .await
    // .expect("unable to setTimeout");

    // still panic
    // #[cfg(all(target_family = "wasm", feature = "wasm-bindgen"))]
    // wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |rs, rj| {
    //     use js_sys::wasm_bindgen;
    //     use js_sys::wasm_bindgen::JsCast;
    //     let global = js_sys::global();
    //     let set_timeout =
    //         js_sys::Object::get_prototype_of(&wasm_bindgen::JsValue::from_str("setTimeout"))
    //             .dyn_into::<js_sys::Function>()
    //             .expect("setTimeout should be a function");
    //     let _ = set_timeout
    //         .call2(
    //             &global,
    //             &rs,
    //             &wasm_bindgen::prelude::JsValue::from(d.as_millis()),
    //         )
    //         .expect("Failed to call setTimeout");
    // }))
    // .await
    // .expect("unable to setTimeout");

    // try request http to mock sleep
    // #[cfg(all(target_family = "wasm", feature = "wasm-bindgen"))]
    {
        // fn now_ms() -> i64 {
        //     js_sys::Date::now() as i64
        // }
        // let start = now_ms();
        const URL: &str = "https://google.com";
        if let Ok(response) = reqwest::Client::new().get(URL).send().await {
            if let Ok(_text) = response.text().await {
                // let end = now_ms();
                // let spend = end - start;
                // #[allow(unused_variables)]
                // let message = format!("[{spend}ms] {URL}: {_text}");
                // js_sys::console_log(message);
            }
        }
    }

    #[cfg(all(target_family = "wasm", not(feature = "wasm-bindgen")))]
    const _: () =
        { panic!("Using ic-agent from WASM requires enabling the `wasm-bindgen` feature") };
}

#[cfg(all(target_family = "wasm", feature = "wasm-bindgen"))]
pub fn spawn(f: impl Future<Output = ()> + 'static) {
    wasm_bindgen_futures::spawn_local(f);
}

#[cfg(not(all(target_family = "wasm", feature = "wasm-bindgen")))]
pub fn spawn(f: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(f);
}

macro_rules! log {
    ($name:ident, $($t:tt)*) => { #[cfg(feature = "tracing")] { tracing::$name!($($t)*) } };
}
