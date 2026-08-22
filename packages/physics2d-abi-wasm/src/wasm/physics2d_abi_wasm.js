/* @ts-self-types="./physics2d_abi_wasm.d.ts" */

/**
 * @returns {number}
 */
export function abi_capabilities() {
    const ret = wasm.abi_capabilities();
    return ret >>> 0;
}

/**
 * @returns {number}
 */
export function abi_version() {
    const ret = wasm.abi_version();
    return ret >>> 0;
}

/**
 * @returns {number}
 */
export function create_abi() {
    const ret = wasm.create_abi();
    return ret >>> 0;
}

/**
 * @param {number} abi
 * @returns {number}
 */
export function create_world(abi) {
    const ret = wasm.create_world(abi);
    return ret >>> 0;
}

/**
 * @param {number} abi
 * @returns {boolean}
 */
export function destroy_abi(abi) {
    const ret = wasm.destroy_abi(abi);
    return ret !== 0;
}

/**
 * @param {number} abi
 * @param {number} world
 * @returns {boolean}
 */
export function destroy_world(abi, world) {
    const ret = wasm.destroy_world(abi, world);
    return ret !== 0;
}

/**
 * @param {number} abi
 * @param {number} world
 * @param {Uint8Array} commands
 * @param {number} byte_length
 * @param {number} command_count
 * @param {Uint32Array} result
 * @returns {boolean}
 */
export function execute(abi, world, commands, byte_length, command_count, result) {
    const ptr0 = passArray8ToWasm0(commands, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    var ptr1 = passArray32ToWasm0(result, wasm.__wbindgen_malloc);
    var len1 = WASM_VECTOR_LEN;
    const ret = wasm.execute(abi, world, ptr0, len0, byte_length, command_count, ptr1, len1, result);
    return ret !== 0;
}

/**
 * @param {number} abi
 * @param {number} world
 * @param {Uint32Array} counts
 * @returns {boolean}
 */
export function query(abi, world, counts) {
    var ptr0 = passArray32ToWasm0(counts, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.query(abi, world, ptr0, len0, counts);
    return ret !== 0;
}

/**
 * @param {number} abi
 * @param {number} world
 * @param {Uint32Array} selection
 * @param {boolean} has_selection
 * @param {Uint32Array} ids
 * @param {Uint32Array} flags
 * @param {Float64Array} values
 * @param {Uint32Array} counts
 * @returns {boolean}
 */
export function read_bodies(abi, world, selection, has_selection, ids, flags, values, counts) {
    const ptr0 = passArray32ToWasm0(selection, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    var ptr1 = passArray32ToWasm0(ids, wasm.__wbindgen_malloc);
    var len1 = WASM_VECTOR_LEN;
    var ptr2 = passArray32ToWasm0(flags, wasm.__wbindgen_malloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = passArrayF64ToWasm0(values, wasm.__wbindgen_malloc);
    var len3 = WASM_VECTOR_LEN;
    var ptr4 = passArray32ToWasm0(counts, wasm.__wbindgen_malloc);
    var len4 = WASM_VECTOR_LEN;
    const ret = wasm.read_bodies(abi, world, ptr0, len0, has_selection, ptr1, len1, ids, ptr2, len2, flags, ptr3, len3, values, ptr4, len4, counts);
    return ret !== 0;
}

/**
 * @param {number} abi
 * @param {number} world
 * @param {Uint32Array} counts
 * @returns {boolean}
 */
export function read_contacts(abi, world, counts) {
    var ptr0 = passArray32ToWasm0(counts, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.read_contacts(abi, world, ptr0, len0, counts);
    return ret !== 0;
}

/**
 * @param {number} abi
 * @param {number} world
 * @param {Uint32Array} ids
 * @param {Uint32Array} flags
 * @param {Float64Array} values
 * @param {Uint32Array} counts
 * @returns {boolean}
 */
export function read_joints(abi, world, ids, flags, values, counts) {
    var ptr0 = passArray32ToWasm0(ids, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    var ptr1 = passArray32ToWasm0(flags, wasm.__wbindgen_malloc);
    var len1 = WASM_VECTOR_LEN;
    var ptr2 = passArrayF64ToWasm0(values, wasm.__wbindgen_malloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = passArray32ToWasm0(counts, wasm.__wbindgen_malloc);
    var len3 = WASM_VECTOR_LEN;
    const ret = wasm.read_joints(abi, world, ptr0, len0, ids, ptr1, len1, flags, ptr2, len2, values, ptr3, len3, counts);
    return ret !== 0;
}

/**
 * @param {number} abi
 * @param {number} world
 * @param {number} dt
 * @param {boolean} has_hooks
 * @returns {number}
 */
export function step(abi, world, dt, has_hooks) {
    const ret = wasm.step(abi, world, dt, has_hooks);
    return ret >>> 0;
}

/**
 * @param {number} abi
 * @param {number} world
 * @returns {number}
 */
export function world_status(abi, world) {
    const ret = wasm.world_status(abi, world);
    return ret >>> 0;
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_copy_to_typed_array_4db0cbe2cc60dbee: function(arg0, arg1, arg2) {
            new Uint8Array(arg2.buffer, arg2.byteOffset, arg2.byteLength).set(getArrayU8FromWasm0(arg0, arg1));
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
        "./physics2d_abi_wasm_bg.js": import0,
    };
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedFloat64ArrayMemory0 = null;
function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
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

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8, 8) >>> 0;
    getFloat64ArrayMemory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedFloat64ArrayMemory0 = null;
    cachedUint32ArrayMemory0 = null;
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
        module_or_path = new URL('physics2d_abi_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
