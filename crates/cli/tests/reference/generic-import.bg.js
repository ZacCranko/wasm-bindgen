/**
 * @param {any} widget
 */
export function run(widget) {
    const ret = wasm.run(widget);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}
export function __wbg___wbindgen_throw_344f42d3211c4765(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbindgen_generic_0000000000000000(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_generic_0000000000000001(arg0, arg1) {
    arg0.attach(arg1);
}
export function __wbindgen_generic_0000000000000002(arg0) {
    const ret = identity(arg0);
    return ret;
}
export function __wbindgen_generic_0000000000000003(arg0) {
    const ret = identity(arg0 >>> 0);
    return ret;
}
export function __wbindgen_generic_0000000000000004(arg0, arg1, arg2) {
    let v0;
    if (arg0 !== 0) {
        v0 = Array.from(getArrayU16FromWasm0(arg0, arg1));
    }
    logOptSlice(v0, arg2 >>> 0);
}
export function __wbindgen_generic_0000000000000005(arg0) {
    logRef(arg0);
}
export function __wbindgen_generic_0000000000000006(arg0) {
    logRef(arg0);
}
export function __wbindgen_generic_0000000000000007(arg0) {
    logRef(arg0 >>> 0);
}
export function __wbindgen_generic_0000000000000008(arg0, arg1, arg2) {
    var v0 = Array.from(getArrayU16FromWasm0(arg0, arg1));
    logSlice(v0, arg2);
}
export function __wbindgen_generic_0000000000000009(arg0, arg1, arg2) {
    var v0 = Array.from(getArrayU16FromWasm0(arg0, arg1));
    logSlice(v0, arg2 >>> 0);
}
export function __wbindgen_generic_000000000000000a(arg0) {
    log(arg0);
}
export function __wbindgen_generic_000000000000000b(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        log(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
}
export function __wbindgen_generic_000000000000000c(arg0) {
    log(arg0 >>> 0);
}
export function __wbindgen_generic_000000000000000d(arg0, arg1) {
    mix(arg0 >>> 0, arg1);
}
export function __wbindgen_generic_000000000000000e(arg0, arg1) {
    mix(arg0 >>> 0, arg1 >>> 0);
}
export function __wbindgen_generic_000000000000000f(arg0) {
    const ret = Widget.of(arg0);
    return ret;
}
export function __wbindgen_generic_0000000000000010(arg0) {
    const ret = Widget.of(arg0 >>> 0);
    return ret;
}
export function __wbindgen_generic_0000000000000011(arg0, arg1) {
    pair(arg0 >>> 0, arg1);
}
export function __wbindgen_generic_0000000000000012(arg0, arg1) {
    arg0.set(arg1);
}
export function __wbindgen_generic_0000000000000013(arg0, arg1) {
    arg0.set(arg1 >>> 0);
}
export function __wbindgen_generic_0000000000000014() { return handleError(function (arg0) {
    tryLog(arg0 >>> 0);
}, arguments); }
export function __wbindgen_generic_0000000000000015(arg0, arg1, arg2) {
    var v0 = getArrayU32FromWasm0(arg1, arg2).slice();
    wasm.__wbindgen_free(arg1, arg2 * 4, 4);
    variadicLog(arg0 >>> 0, ...(v0));
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
}
function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function getArrayU16FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint16ArrayMemory0().subarray(ptr / 2, ptr / 2 + len);
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint16ArrayMemory0 = null;
function getUint16ArrayMemory0() {
    if (cachedUint16ArrayMemory0 === null || cachedUint16ArrayMemory0.byteLength === 0) {
        cachedUint16ArrayMemory0 = new Uint16Array(wasm.memory.buffer);
    }
    return cachedUint16ArrayMemory0;
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
