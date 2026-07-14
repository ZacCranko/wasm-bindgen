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

    // A concrete argument mixed with a generic one. Two instantiations exercise
    // per-mono shim manufacture for a mixed signature.
    #[wasm_bindgen(generic_per_mono, js_name = mix)]
    fn mix<T>(label: u32, value: T);

    // Multiple generic type parameters in a single import.
    #[wasm_bindgen(generic_per_mono, js_name = pair)]
    fn pair<T, U>(a: T, b: U);

    // `catch` produces a `handleError`-wrapped shim.
    #[wasm_bindgen(generic_per_mono, catch, js_name = tryLog)]
    fn try_log<T>(x: T) -> Result<(), JsValue>;

    // `variadic` spreads the final argument. The variadic argument must be a
    // concrete iterable (here `Vec<T>`), which marshals to a spreadable JS
    // array; a bare generic `T` is rejected because it may monomorphise to a
    // non-iterable scalar.
    #[wasm_bindgen(generic_per_mono, variadic, js_name = variadicLog)]
    fn variadic_log<T>(first: u32, rest: Vec<T>);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Widget)]
    type Widget;

    // A generic method binds as an instance method call. Two instantiations
    // prove distinct per-mono shims for the method path.
    #[wasm_bindgen(method, generic_per_mono, js_class = "Widget", js_name = set)]
    fn set<T>(this: &Widget, value: T);

    // A generic static method. Two instantiations prove distinct per-mono shims
    // for the static path.
    #[wasm_bindgen(static_method_of = Widget, generic_per_mono, js_name = of)]
    fn of<T>(value: T) -> Widget;
}

#[wasm_bindgen]
pub fn run(widget: &Widget) -> Result<(), JsValue> {
    log_generic(1u32);
    log_generic(2.0f64);
    log_generic(String::from("three"));

    let _ = identity(3u32);
    let _ = identity(4.0f64);

    mix(5, 6u32);
    mix(6, 7.0f64);

    pair(1u32, 2.0f64);

    try_log(7u32)?;

    variadic_log(8, vec![9u32, 10u32]);

    widget.set(10u32);
    widget.set(11.0f64);

    let _ = Widget::of(11u32);
    let _ = Widget::of(12.0f64);

    Ok(())
}
