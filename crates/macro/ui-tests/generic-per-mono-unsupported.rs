use wasm_bindgen::prelude::*;

// The experimental per-monomorphisation generic import path
// (`#[wasm_bindgen(generic_per_mono)]`) rejects a handful of shapes with a
// clear diagnostic, deferring them to the type-erasure generic path. Each
// function below exercises one of those aborting paths.
#[wasm_bindgen]
extern "C" {
    // Lifetime parameters are not supported.
    #[wasm_bindgen(generic_per_mono)]
    fn with_lifetime<'a, T>(x: T);

    // `generic_per_mono` requires at least one type parameter.
    #[wasm_bindgen(generic_per_mono)]
    fn without_type_param(x: u32);

    // References to a generic type parameter (`&T`) are not supported.
    #[wasm_bindgen(generic_per_mono)]
    fn ref_to_generic<T>(x: &T);

    // Nor is a reference to a generic parameter nested inside another type
    // (e.g. `Option<&T>`).
    #[wasm_bindgen(generic_per_mono)]
    fn nested_ref_to_generic<T>(x: Option<&T>);

    // Returning a reference is not supported.
    #[wasm_bindgen(generic_per_mono)]
    fn return_ref<T>(x: T) -> &JsValue;

    // A bare generic type parameter cannot be the `variadic` argument, since it
    // may monomorphise to a non-iterable scalar.
    #[wasm_bindgen(generic_per_mono, variadic)]
    fn variadic_scalar<T>(first: u32, rest: T);
}

fn main() {}
