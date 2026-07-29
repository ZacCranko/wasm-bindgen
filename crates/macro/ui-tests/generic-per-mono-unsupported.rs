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

    // A bare shared reference to a generic type parameter (`&T`) *is* now
    // supported, but a mutable reference (`&mut T`) is not.
    #[wasm_bindgen(generic_per_mono)]
    fn mut_ref_to_generic<T>(x: &mut T);

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

    // `catch` hard-codes the error type to `JsValue` and monomorphises only the
    // `Ok` type, so a type parameter in the error position is rejected.
    #[wasm_bindgen(generic_per_mono, catch)]
    fn catch_generic_err<T>(x: T) -> Result<JsValue, T>;

    // `slice_to_array` needs a concrete element type: `VectorRefIntoWasmAbi` is
    // implemented per concrete ABI shape, so no bound makes `&[T]` work.
    #[wasm_bindgen(generic_per_mono, slice_to_array)]
    fn slice_to_array_generic_elem<T>(xs: &[T], other: T);

    // Also rejected when only nested inside the element type...
    #[wasm_bindgen(generic_per_mono, slice_to_array)]
    fn slice_to_array_nested_elem<T>(xs: &[Vec<T>], other: T);

    // ...and through the `Option<&[T]>` form.
    #[wasm_bindgen(generic_per_mono, slice_to_array)]
    fn slice_to_array_option_elem<T>(xs: Option<&[T]>, other: T);
}

// `slice_to_array` is inherited from the enclosing block, and the same rejection
// applies on the type-erasure generic path, which has no `generic_per_mono`.
#[wasm_bindgen(slice_to_array)]
extern "C" {
    fn erased_slice_to_array_generic_elem<T>(xs: &[T]);
}

fn main() {}
