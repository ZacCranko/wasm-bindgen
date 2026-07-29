//! Runtime coverage for user-written trait bounds on a per-monomorphisation
//! generic import (`#[wasm_bindgen(generic_per_mono)]`).
//!
//! Bounds are part of the declared signature's contract, so they must be carried
//! through codegen rather than dropped. Two routes are exercised, because they
//! reach the generated wrapper differently:
//! - inline bounds (`fn f<T: Trait>`), which travel with the parameter list;
//! - `where` predicates, which have no such carrier and must be re-emitted.
//!
//! A bound must also reach the monomorphised shim, whose ABI signature can
//! project an associated type off a bounded parameter (`T::Wire`); without the
//! bound in scope there, that projection does not resolve. `compile-fail`
//! coverage for callers that violate a bound lives in
//! `crates/macro/ui-tests/generic-per-mono-bounds.rs`.

use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

/// Projects a type parameter onto the type that actually crosses the ABI.
trait Wire {
    type Wire;
    fn lift(wire: Self::Wire) -> Self;
}

impl Wire for u32 {
    type Wire = u32;
    fn lift(wire: u32) -> u32 {
        wire
    }
}

impl Wire for String {
    type Wire = String;
    fn lift(wire: String) -> String {
        wire
    }
}

#[wasm_bindgen(module = "tests/wasm/generic_import_bounds.js")]
extern "C" {
    #[wasm_bindgen(js_name = takeLog)]
    fn take_log() -> String;

    // Inline bound.
    #[wasm_bindgen(generic_per_mono, js_name = record)]
    fn record_inline<T: Copy + Clone>(x: T);

    // `where` predicate.
    #[wasm_bindgen(generic_per_mono, js_name = record)]
    fn record_where<T>(x: T)
    where
        T: Clone;

    // Inline and `where` bounds on the same signature, across two parameters.
    #[wasm_bindgen(generic_per_mono, js_name = sum)]
    fn sum_bounded<T: Copy, U>(a: T, b: U) -> f64
    where
        U: Copy;

    // The bound's associated type appears in argument and return position, so
    // the bound has to reach the shim for `T::Wire`'s ABI to resolve.
    #[wasm_bindgen(generic_per_mono, js_name = echo)]
    fn echo_wire<T>(x: T::Wire) -> T::Wire
    where
        T: Wire;

    // Higher-ranked `where` predicate.
    #[wasm_bindgen(generic_per_mono, js_name = echo)]
    fn echo_hrtb<T>(x: T) -> T
    where
        for<'a> &'a T: Clone,
        T: Clone;
}

/// A caller that must itself satisfy the import's bounds to name `T::Wire`.
fn wire_round_trip<T>(wire: T::Wire) -> T
where
    T: Wire,
    T::Wire: IntoWasmAbi + FromWasmAbi + WasmDescribe,
{
    T::lift(echo_wire::<T>(wire))
}

#[wasm_bindgen_test]
fn generic_import_inline_bound() {
    let _ = take_log();
    record_inline(1u32);
    record_inline(2.5f64);
    record_inline(true);
    assert_eq!(take_log(), "1,2.5,true");
}

#[wasm_bindgen_test]
fn generic_import_where_bound() {
    let _ = take_log();
    record_where(7u32);
    record_where(String::from("s"));
    assert_eq!(take_log(), "7,s");
}

#[wasm_bindgen_test]
fn generic_import_inline_and_where_bounds() {
    assert_eq!(sum_bounded(2u32, 3u32), 5.0);
    assert_eq!(sum_bounded(2.5f64, 4u8), 6.5);
}

#[wasm_bindgen_test]
fn generic_import_associated_type_through_bound() {
    assert_eq!(wire_round_trip::<u32>(9u32), 9u32);
    assert_eq!(
        wire_round_trip::<String>(String::from("assoc")),
        String::from("assoc")
    );
}

#[wasm_bindgen_test]
fn generic_import_higher_ranked_bound() {
    assert_eq!(echo_hrtb(4u32), 4u32);
    assert_eq!(echo_hrtb(String::from("h")), String::from("h"));
}
