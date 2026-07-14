# Execution Plan: `logan/generic-descriptors` review remediation

Derived from the correlated 3-agent review of `1a2860ea1..HEAD` (interpreter-based
per-monomorphisation generic imports). Items are ordered by priority. Each item lists
the problem, the change, and how to verify.

---

## P0 — Functional / correctness bugs

### 1. Fix variadic generic shim codegen (broken JS)
- **Problem:** `crates/cli/tests/reference/generic-import.bg.js:41` emits
  `variadicLog(arg0 >>> 0, ...arg1 >>> 0)`. This parses as `...(arg1 >>> 0)` and
  spreads a scalar `Number`, throwing `TypeError` at runtime. Root cause: the
  variadic path folds the scalar cast into the spread arg without parenthesizing,
  and there is no guard against `variadic` on a non-iterable/scalar generic param.
- **Change:**
  - In `crates/cli-support/src/js/mod.rs` (`variadic_args`), ensure the last-arg
    expression is parenthesized before `...`, i.e. `...(expr)`, and does not fold a
    scalar cast into the spread.
  - Add a macro-side guard (`crates/macro-support/src/{parser,codegen}.rs`) rejecting
    `variadic` combined with a scalar/non-iterable generic param, with a clear
    diagnostic.
  - Add a compile-fail case to `crates/macro/ui-tests/` covering `variadic` + scalar
    generic (+ `.stderr`).
- **Verify:** `cargo test -p wasm-bindgen-cli` (reference regen), `cargo test` ui-tests;
  regenerate `generic-import.bg.js` and confirm valid JS.

### 2. Bump `SCHEMA_VERSION`
- **Problem:** `crates/shared/src/lib.rs:16` is still `"0.2.122"` while a new `generic:
  bool` schema field changed the wire format and `crates/shared/Cargo.toml` is
  `"0.2.126"`. Mismatched-build macro/CLI silently misparse descriptors.
- **Change:** Set `SCHEMA_VERSION` to `"0.2.126"` (match `Cargo.toml`). Confirm the
  approved hash in `schema_hash_approval.rs` still matches.
- **Verify:** `cargo test -p wasm-bindgen-shared` (schema_version test).

---

## P1 — Test infrastructure & maintainability

### 3. Update the reference-test sanitizer regex
- **Problem:** `crates/cli/tests/wasm-bindgen/reference.rs:102-103` still sanitizes
  `__wbindgen_cast_[0-9a-f]{16}`; names are now `__wbindgen_generic_*`, so the regex
  is dead.
- **Change:** Update pattern + format string to `__wbindgen_generic_`.
- **Verify:** `cargo test -p wasm-bindgen-cli` reference tests pass.

### 4. De-duplicate import-finalize logic
- **Problem:** `crates/cli-support/src/wit/mod.rs:1021-1045` (in `bind_generic_imports`)
  is a verbatim copy of the tail of `import_function`; order-sensitive
  catch/variadic/`import_map.insert` code will drift.
- **Change:** Extract `fn finish_import_binding(&mut self, id, aux, catch, variadic,
  assert_no_shim)` and call from both sites.
- **Verify:** `cargo build -p wasm-bindgen-cli-support`; reference tests unchanged.

---

## P2 — Robustness & correctness gaps

### 5. Clean diagnostic for nested/reference generic shapes
- **Problem:** `crates/macro-support/src/codegen.rs:2617-2632` only guards a direct
  `&T`; `Option<&T>`, `(T, &T)`, `[&T; N]`, `Box<&T>` fall through to a confusing
  trait-bound error.
- **Change:** Recurse to detect any `&<generic>` occurrence and bail with the same
  clear diagnostic. Add ui-test coverage.

### 6. Handle or reject `reexport` / intrinsic in the generic path
- **Problem:** `crates/cli-support/src/wit/mod.rs:865-924` generic branch omits
  `import.reexport` and `PLACEHOLDER_MODULE`/`Intrinsic` handling; combining reexport
  with a generic import silently drops the binding.
- **Change:** Handle `import.reexport`, or `bail!` with a clear "unsupported for
  generic imports" message.

### 7. Avoid redundant full-module traversal per build
- **Problem:** `bind_generic_imports` runs a second `handle_duplicate_imports`
  (`dfs_pre_order_mut` over the whole module) on essentially every build now that
  `wbg_cast` is pervasive.
- **Change:** Skip the traversal when the duplicate map is empty; investigate merging
  with an existing module pass.
- **Verify:** reference tests unchanged; sanity-check build time on a large module.

---

## P3 — Coverage & cleanup

### 8. Expand generic-import test coverage
- Add a second monomorphisation for `mix`, `set`, `of`; add a string/`&str` type
  param, a multi-param `<T, U>` case, and a two-mono method/static.
  (`crates/cli/tests/reference/generic-import.rs` + regenerate snapshots.)

### 9. Naming / comment consistency (cosmetic)
- Reconcile `cast`→`generic` naming: update "Cast intrinsic" comments in
  `intrinsic.rs`/`wit/mod.rs`; fix stale comment `interpreter/mod.rs:50`.
- Make the `sorted.sort_by` in `wit/mod.rs:343` a total order (tie-break on full key)
  and document/confirm the reserved index `0`.
- Consider renaming the `generic` schema field to `generic_per_mono` for consistency
  with the AST field.
- Remove redundant clone at `codegen.rs:2557`.

---

## Sequencing & verification
1. Land P0 (#1, #2) — regenerate reference snapshots, run full `cargo test`.
2. Land P1 (#3, #4).
3. Land P2 (#5, #6, #7).
4. Land P3 (#8, #9).

Global verification after each phase:
```
cargo test --workspace
cargo test -p wasm-bindgen-cli        # reference/ui snapshots
cargo test -p wasm-bindgen-shared     # schema_version
cargo build --workspace
```
