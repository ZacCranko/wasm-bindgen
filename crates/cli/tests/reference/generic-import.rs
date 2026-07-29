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

    // A bare shared reference to a generic type parameter (`&T`). Copyable
    // referents (`&u32`, `&f64`) marshal by value; `&JsValue` marshals as an
    // externref. Each instantiation gets its own per-mono shim.
    #[wasm_bindgen(generic_per_mono, js_name = logRef)]
    fn log_ref<T>(x: &T);

    // `catch` produces a `handleError`-wrapped shim.
    #[wasm_bindgen(generic_per_mono, catch, js_name = tryLog)]
    fn try_log<T>(x: T) -> Result<(), JsValue>;

    // `variadic` spreads the final argument. The variadic argument must be a
    // concrete iterable (here `Vec<T>`), which marshals to a spreadable JS
    // array; a bare generic `T` is rejected because it may monomorphise to a
    // non-iterable scalar.
    #[wasm_bindgen(generic_per_mono, variadic, js_name = variadicLog)]
    fn variadic_log<T>(first: u32, rest: Vec<T>);

    // `slice_to_array` hands JS a plain `Array` it owns rather than a
    // typed-array view into wasm memory. The slice element type must be
    // concrete, so the rewrite is independent of which monomorphisation is
    // being generated.
    #[wasm_bindgen(generic_per_mono, slice_to_array, js_name = logSlice)]
    fn log_slice<T>(xs: &[u16], other: T);

    #[wasm_bindgen(generic_per_mono, slice_to_array, js_name = logOptSlice)]
    fn log_opt_slice<T>(xs: Option<&[u16]>, other: T);

    // A `String` element type takes the *other* ownership path: JS receives a
    // freshly allocated index buffer that it must free, unlike the primitive
    // case above which borrows the caller's slice.
    #[wasm_bindgen(generic_per_mono, slice_to_array, js_name = logStrSlice)]
    fn log_str_slice<T>(xs: &[String], other: T);

    #[wasm_bindgen(generic_per_mono, slice_to_array, js_name = logOptStrSlice)]
    fn log_opt_str_slice<T>(xs: Option<&[String]>, other: T);

    // `async` imports return a `Promise` across the ABI whatever they resolve
    // to, so the descriptor is an externref and the resolved value is converted
    // separately inside `JsFuture<T>`. That makes a monomorphised `-> T` work,
    // including for a `T` that is not itself handle-shaped.
    #[wasm_bindgen(generic_per_mono, js_name = asyncIdentity)]
    async fn async_identity<T>(x: T) -> T;

    // Same, but resolving to a concrete non-handle type.
    #[wasm_bindgen(generic_per_mono, js_name = asyncCount)]
    async fn async_count<T>(x: T) -> u32;

    // And through the `Ok` type of a `catch` import.
    #[wasm_bindgen(generic_per_mono, catch, js_name = asyncTry)]
    async fn async_try<T>(x: T) -> Result<T, JsValue>;
}

// `slice_to_array` is inheritable from the enclosing block and applies to every
// slice-shaped argument of every function it covers, `generic_per_mono` included.
#[wasm_bindgen(slice_to_array)]
extern "C" {
    #[wasm_bindgen(generic_per_mono, js_name = logBlockSlice)]
    fn log_block_slice<T>(xs: &[u16], other: T);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Widget)]
    type Widget;

    // A generic method binds as an instance method call. Two instantiations
    // prove distinct per-mono shims for the method path.
    #[wasm_bindgen(method, generic_per_mono, js_class = "Widget", js_name = set)]
    fn set<T>(this: &Widget, value: T);

    // A generic method taking a bare shared reference `&T`. Here `T`
    // monomorphises to the JS-handle `Widget`, so `&Widget` marshals via the
    // handle's `IntoWasmAbi for &Widget` impl.
    #[wasm_bindgen(method, generic_per_mono, js_class = "Widget", js_name = attach)]
    fn attach<T>(this: &Widget, other: &T);

    // A generic static method. Two instantiations prove distinct per-mono shims
    // for the static path.
    #[wasm_bindgen(static_method_of = Widget, generic_per_mono, js_name = of)]
    fn of<T>(value: T) -> Widget;
}

#[wasm_bindgen]
pub async fn run(widget: &Widget) -> Result<(), JsValue> {
    log_generic(1u32);
    log_generic(2.0f64);
    log_generic(String::from("three"));

    let _ = identity(3u32);
    let _ = identity(4.0f64);

    mix(5, 6u32);
    mix(6, 7.0f64);

    pair(1u32, 2.0f64);

    log_ref(&13u32);
    log_ref(&14.0f64);
    log_ref(&JsValue::from("fifteen"));

    try_log(7u32)?;

    variadic_log(8, vec![9u32, 10u32]);

    log_slice(&[1u16, 2u16], 9u32);
    log_slice(&[3u16, 4u16], 10.0f64);
    log_opt_slice(Some(&[5u16]), 11u32);
    log_opt_slice(None, 12u32);
    log_str_slice(&[String::from("a")], 13u32);
    log_opt_str_slice(Some(&[String::from("b")]), 18u32);
    log_opt_str_slice(None, 19u32);
    log_block_slice(&[6u16], 14u32);

    let _: u32 = async_identity(15u32).await;
    let _: String = async_identity(String::from("b")).await;
    let _: u32 = async_count(16u32).await;
    let _: u32 = async_try(17u32).await?;

    widget.set(10u32);
    widget.set(11.0f64);

    widget.attach(widget);

    let _ = Widget::of(11u32);
    let _ = Widget::of(12.0f64);

    Ok(())
}
