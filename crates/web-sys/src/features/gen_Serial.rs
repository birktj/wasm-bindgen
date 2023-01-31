#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = EventTarget , extends = :: js_sys :: Object , js_name = Serial , typescript_type = "Serial")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Serial` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Serial)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Serial`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type Serial;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "Serial" , js_name = onconnect)]
    #[doc = "Getter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Serial/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Serial`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onconnect(this: &Serial) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , setter , js_class = "Serial" , js_name = onconnect)]
    #[doc = "Setter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Serial/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Serial`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onconnect(this: &Serial, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "Serial" , js_name = ondisconnect)]
    #[doc = "Getter for the `ondisconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Serial/ondisconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Serial`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ondisconnect(this: &Serial) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , setter , js_class = "Serial" , js_name = ondisconnect)]
    #[doc = "Setter for the `ondisconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Serial/ondisconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Serial`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_ondisconnect(this: &Serial, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "Serial" , js_name = getPorts)]
    #[doc = "The `getPorts()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Serial/getPorts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Serial`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_ports(this: &Serial) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "Serial" , js_name = requestPort)]
    #[doc = "The `requestPort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Serial/requestPort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Serial`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_port(this: &Serial) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SerialPortRequestOptions")]
    # [wasm_bindgen (method , structural , js_class = "Serial" , js_name = requestPort)]
    #[doc = "The `requestPort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Serial/requestPort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Serial`, `SerialPortRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_port_with_options(
        this: &Serial,
        options: &SerialPortRequestOptions,
    ) -> ::js_sys::Promise;
}