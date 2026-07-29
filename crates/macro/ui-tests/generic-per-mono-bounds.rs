// This is the only `generic_per_mono` ui-test whose codegen succeeds, so it is
// the only one that expands the shim's
// `#[cfg_attr(wasm_bindgen_unstable_test_coverage, ...)]`. That cfg is not
// declared for the trybuild crate; allow it so the snapshot stays stable.
#![allow(unexpected_cfgs)]

use wasm_bindgen::prelude::*;

// Trait bounds declared on a per-monomorphisation generic import
// (`#[wasm_bindgen(generic_per_mono)]`) are part of its contract: they are
// carried through codegen, so a caller that violates one is rejected, and the
// diagnostic points at the user's own bound. Both an inline bound and a `where`
// predicate are exercised, since they reach the generated wrapper by different
// routes.

// `u32` satisfies the `IntoWasmAbi`/`WasmDescribe` bounds the codegen
// synthesizes, so only the user-written bound can be what fails below.
trait Marker {}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(generic_per_mono)]
    fn inline_bound<T: Marker>(x: T);

    #[wasm_bindgen(generic_per_mono)]
    fn where_bound<T>(x: T)
    where
        T: Marker;
}

fn violates_inline_bound() {
    inline_bound(1u32);
}

fn violates_where_bound() {
    where_bound(2u32);
}

fn main() {}
