// Reference output for the experimental per-monomorphisation generic import
// path (`#[wasm_bindgen(generic_per_mono)]`). Each concrete instantiation is
// discovered by the descriptor interpreter and bound to its own manufactured
// `__wbindgen_generic_*` JS shim, rather than erasing type parameters to
// `JsValue`.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Free function, generic owned argument, unit return. Two instantiations
    // (`u32` and `f64`) each get their own shim with the correct marshalling.
    #[wasm_bindgen(generic_per_mono, js_name = log)]
    fn log_generic<T>(x: T);

    // Generic pass-through return `-> T`.
    #[wasm_bindgen(generic_per_mono, js_name = identity)]
    fn identity<T>(x: T) -> T;

    // A concrete argument mixed with a generic one.
    #[wasm_bindgen(generic_per_mono, js_name = mix)]
    fn mix<T>(label: u32, value: T);

    // `catch` produces a `handleError`-wrapped shim.
    #[wasm_bindgen(generic_per_mono, catch, js_name = tryLog)]
    fn try_log<T>(x: T) -> Result<(), JsValue>;

    // `variadic` spreads the final argument.
    #[wasm_bindgen(generic_per_mono, variadic, js_name = variadicLog)]
    fn variadic_log<T>(first: u32, rest: T);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Widget)]
    type Widget;

    // A generic method binds as an instance method call.
    #[wasm_bindgen(method, generic_per_mono, js_class = "Widget", js_name = set)]
    fn set<T>(this: &Widget, value: T);

    // A generic static method.
    #[wasm_bindgen(static_method_of = Widget, generic_per_mono, js_name = of)]
    fn of<T>(value: T) -> Widget;
}

#[wasm_bindgen]
pub fn run(widget: &Widget) -> Result<(), JsValue> {
    log_generic(1u32);
    log_generic(2.0f64);

    let _ = identity(3u32);
    let _ = identity(4.0f64);

    mix(5, 6u32);

    try_log(7u32)?;

    variadic_log(8, 9u32);

    widget.set(10u32);
    let _ = Widget::of(11u32);

    Ok(())
}
