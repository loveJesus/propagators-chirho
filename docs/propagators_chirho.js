/* @ts-self-types="./propagators_chirho.d.ts" */

/**
 * A standalone interval for use in JavaScript.
 */
export class WasmIntervalChirho {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmIntervalChirho.prototype);
        obj.__wbg_ptr = ptr;
        WasmIntervalChirhoFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmIntervalChirhoFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmintervalchirho_free(ptr, 0);
    }
    /**
     * Adds two intervals.
     * @param {WasmIntervalChirho} other_chirho
     * @returns {WasmIntervalChirho}
     */
    add(other_chirho) {
        _assertClass(other_chirho, WasmIntervalChirho);
        const ret = wasm.wasmintervalchirho_add(this.__wbg_ptr, other_chirho.__wbg_ptr);
        return WasmIntervalChirho.__wrap(ret);
    }
    /**
     * Returns true if the interval contains a value.
     * @param {number} value_chirho
     * @returns {boolean}
     */
    contains(value_chirho) {
        const ret = wasm.wasmintervalchirho_contains(this.__wbg_ptr, value_chirho);
        return ret !== 0;
    }
    /**
     * Creates an exact value interval.
     * @param {number} value_chirho
     * @returns {WasmIntervalChirho}
     */
    static exact(value_chirho) {
        const ret = wasm.wasmintervalchirho_exact(value_chirho);
        return WasmIntervalChirho.__wrap(ret);
    }
    /**
     * Gets the high bound.
     * @returns {number}
     */
    get hi_chirho() {
        const ret = wasm.wasmintervalchirho_hi_chirho(this.__wbg_ptr);
        return ret;
    }
    /**
     * Intersects two intervals.
     * @param {WasmIntervalChirho} other_chirho
     * @returns {WasmIntervalChirho}
     */
    intersect(other_chirho) {
        _assertClass(other_chirho, WasmIntervalChirho);
        const ret = wasm.wasmintervalchirho_intersect(this.__wbg_ptr, other_chirho.__wbg_ptr);
        return WasmIntervalChirho.__wrap(ret);
    }
    /**
     * Returns true if the interval is empty (contradiction).
     * @returns {boolean}
     */
    isEmpty() {
        const ret = wasm.wasmintervalchirho_isEmpty(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Returns true if this is an exact value.
     * @returns {boolean}
     */
    isExact() {
        const ret = wasm.wasmintervalchirho_isExact(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Gets the low bound.
     * @returns {number}
     */
    get lo_chirho() {
        const ret = wasm.wasmintervalchirho_lo_chirho(this.__wbg_ptr);
        return ret;
    }
    /**
     * Multiplies two intervals.
     * @param {WasmIntervalChirho} other_chirho
     * @returns {WasmIntervalChirho}
     */
    mul(other_chirho) {
        _assertClass(other_chirho, WasmIntervalChirho);
        const ret = wasm.wasmintervalchirho_mul(this.__wbg_ptr, other_chirho.__wbg_ptr);
        return WasmIntervalChirho.__wrap(ret);
    }
    /**
     * Creates a new interval.
     * @param {number} lo_chirho
     * @param {number} hi_chirho
     */
    constructor(lo_chirho, hi_chirho) {
        const ret = wasm.wasmintervalchirho_new_chirho(lo_chirho, hi_chirho);
        this.__wbg_ptr = ret >>> 0;
        WasmIntervalChirhoFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Subtracts two intervals.
     * @param {WasmIntervalChirho} other_chirho
     * @returns {WasmIntervalChirho}
     */
    sub(other_chirho) {
        _assertClass(other_chirho, WasmIntervalChirho);
        const ret = wasm.wasmintervalchirho_sub(this.__wbg_ptr, other_chirho.__wbg_ptr);
        return WasmIntervalChirho.__wrap(ret);
    }
    /**
     * Returns the width (hi - lo).
     * @returns {number}
     */
    width() {
        const ret = wasm.wasmintervalchirho_width(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmIntervalChirho.prototype[Symbol.dispose] = WasmIntervalChirho.prototype.free;

/**
 * A WebAssembly-compatible propagator network.
 *
 * This wraps the arena-based network for efficient WASM execution.
 */
export class WasmNetworkChirho {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmNetworkChirhoFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmnetworkchirho_free(ptr, 0);
    }
    /**
     * Adds an addition constraint: a + b = c
     * @param {number} a_chirho
     * @param {number} b_chirho
     * @param {number} c_chirho
     */
    addAdder(a_chirho, b_chirho, c_chirho) {
        wasm.wasmnetworkchirho_addAdder(this.__wbg_ptr, a_chirho, b_chirho, c_chirho);
    }
    /**
     * Adds a multiplication constraint: a * b = c
     * @param {number} a_chirho
     * @param {number} b_chirho
     * @param {number} c_chirho
     */
    addMultiplier(a_chirho, b_chirho, c_chirho) {
        wasm.wasmnetworkchirho_addMultiplier(this.__wbg_ptr, a_chirho, b_chirho, c_chirho);
    }
    /**
     * Adds a square constraint: a² = b
     * @param {number} a_chirho
     * @param {number} b_chirho
     */
    addSquarer(a_chirho, b_chirho) {
        wasm.wasmnetworkchirho_addSquarer(this.__wbg_ptr, a_chirho, b_chirho);
    }
    /**
     * Returns the number of cells.
     * @returns {number}
     */
    cellCount() {
        const ret = wasm.wasmnetworkchirho_cellCount(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Gets the exact value if the cell is exact (lo == hi).
     * @param {number} cell_id_chirho
     * @returns {number | undefined}
     */
    getExact(cell_id_chirho) {
        const ret = wasm.wasmnetworkchirho_getExact(this.__wbg_ptr, cell_id_chirho);
        return ret[0] === 0 ? undefined : ret[1];
    }
    /**
     * Gets the high bound of a cell's interval.
     * @param {number} cell_id_chirho
     * @returns {number | undefined}
     */
    getHi(cell_id_chirho) {
        const ret = wasm.wasmnetworkchirho_getHi(this.__wbg_ptr, cell_id_chirho);
        return ret[0] === 0 ? undefined : ret[1];
    }
    /**
     * Gets the low bound of a cell's interval.
     * @param {number} cell_id_chirho
     * @returns {number | undefined}
     */
    getLo(cell_id_chirho) {
        const ret = wasm.wasmnetworkchirho_getLo(this.__wbg_ptr, cell_id_chirho);
        return ret[0] === 0 ? undefined : ret[1];
    }
    /**
     * Returns true if any cell has a contradiction.
     * @returns {boolean}
     */
    hasContradiction() {
        const ret = wasm.wasmnetworkchirho_hasContradiction(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Returns true if the cell is in contradiction.
     * @param {number} cell_id_chirho
     * @returns {boolean}
     */
    isContradiction(cell_id_chirho) {
        const ret = wasm.wasmnetworkchirho_isContradiction(this.__wbg_ptr, cell_id_chirho);
        return ret !== 0;
    }
    /**
     * Creates a new cell and returns its ID.
     * @returns {number}
     */
    makeCell() {
        const ret = wasm.wasmnetworkchirho_makeCell(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Creates a new empty network.
     */
    constructor() {
        const ret = wasm.wasmnetworkchirho_new_chirho();
        this.__wbg_ptr = ret >>> 0;
        WasmNetworkChirhoFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Runs propagation until fixpoint.
     */
    propagate() {
        wasm.wasmnetworkchirho_propagate(this.__wbg_ptr);
    }
    /**
     * Returns the number of propagation steps.
     * @returns {number}
     */
    propagationCount() {
        const ret = wasm.wasmnetworkchirho_propagationCount(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Returns the number of propagators.
     * @returns {number}
     */
    propagatorCount() {
        const ret = wasm.wasmnetworkchirho_propagatorCount(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Sets a cell to an exact value.
     * @param {number} cell_id_chirho
     * @param {number} value_chirho
     */
    setExact(cell_id_chirho, value_chirho) {
        wasm.wasmnetworkchirho_setExact(this.__wbg_ptr, cell_id_chirho, value_chirho);
    }
    /**
     * Sets a cell to an interval.
     * @param {number} cell_id_chirho
     * @param {number} lo_chirho
     * @param {number} hi_chirho
     */
    setInterval(cell_id_chirho, lo_chirho, hi_chirho) {
        wasm.wasmnetworkchirho_setInterval(this.__wbg_ptr, cell_id_chirho, lo_chirho, hi_chirho);
    }
}
if (Symbol.dispose) WasmNetworkChirho.prototype[Symbol.dispose] = WasmNetworkChirho.prototype.free;

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_be289d5034ed271b: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./propagators_chirho_bg.js": import0,
    };
}

const WasmIntervalChirhoFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmintervalchirho_free(ptr >>> 0, 1));
const WasmNetworkChirhoFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmnetworkchirho_free(ptr >>> 0, 1));

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
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

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('propagators_chirho_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
