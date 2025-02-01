const lAudioContext = (typeof AudioContext !== 'undefined' ? AudioContext : (typeof webkitAudioContext !== 'undefined' ? webkitAudioContext : undefined));
let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_export_0(addHeapObject(e));
    }
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;

function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

let cachedUint8ClampedArrayMemory0 = null;

function getUint8ClampedArrayMemory0() {
    if (cachedUint8ClampedArrayMemory0 === null || cachedUint8ClampedArrayMemory0.byteLength === 0) {
        cachedUint8ClampedArrayMemory0 = new Uint8ClampedArray(wasm.memory.buffer);
    }
    return cachedUint8ClampedArrayMemory0;
}

function getClampedArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ClampedArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedUint32ArrayMemory0 = null;

function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => {
    wasm.__wbindgen_export_4.get(state.dtor)(state.a, state.b)
});

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_4.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state);
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}
function __wbg_adapter_34(arg0, arg1, arg2) {
    wasm.__wbindgen_export_5(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_45(arg0, arg1) {
    wasm.__wbindgen_export_6(arg0, arg1);
}

function __wbg_adapter_56(arg0, arg1, arg2) {
    wasm.__wbindgen_export_7(arg0, arg1, isLikeNone(arg2) ? 0 : addHeapObject(arg2));
}

function __wbg_adapter_71(arg0, arg1, arg2, arg3) {
    wasm.__wbindgen_export_8(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

const __wbindgen_enum_GamepadMappingType = ["", "standard"];

const __wbindgen_enum_GpuCompilationMessageType = ["error", "warning", "info"];

const __wbindgen_enum_GpuDeviceLostReason = ["unknown", "destroyed"];

const __wbindgen_enum_GpuErrorFilter = ["validation", "out-of-memory", "internal"];

const __wbindgen_enum_GpuIndexFormat = ["uint16", "uint32"];

const __wbindgen_enum_GpuTextureFormat = ["r8unorm", "r8snorm", "r8uint", "r8sint", "r16uint", "r16sint", "r16float", "rg8unorm", "rg8snorm", "rg8uint", "rg8sint", "r32uint", "r32sint", "r32float", "rg16uint", "rg16sint", "rg16float", "rgba8unorm", "rgba8unorm-srgb", "rgba8snorm", "rgba8uint", "rgba8sint", "bgra8unorm", "bgra8unorm-srgb", "rgb9e5ufloat", "rgb10a2uint", "rgb10a2unorm", "rg11b10ufloat", "rg32uint", "rg32sint", "rg32float", "rgba16uint", "rgba16sint", "rgba16float", "rgba32uint", "rgba32sint", "rgba32float", "stencil8", "depth16unorm", "depth24plus", "depth24plus-stencil8", "depth32float", "depth32float-stencil8", "bc1-rgba-unorm", "bc1-rgba-unorm-srgb", "bc2-rgba-unorm", "bc2-rgba-unorm-srgb", "bc3-rgba-unorm", "bc3-rgba-unorm-srgb", "bc4-r-unorm", "bc4-r-snorm", "bc5-rg-unorm", "bc5-rg-snorm", "bc6h-rgb-ufloat", "bc6h-rgb-float", "bc7-rgba-unorm", "bc7-rgba-unorm-srgb", "etc2-rgb8unorm", "etc2-rgb8unorm-srgb", "etc2-rgb8a1unorm", "etc2-rgb8a1unorm-srgb", "etc2-rgba8unorm", "etc2-rgba8unorm-srgb", "eac-r11unorm", "eac-r11snorm", "eac-rg11unorm", "eac-rg11snorm", "astc-4x4-unorm", "astc-4x4-unorm-srgb", "astc-5x4-unorm", "astc-5x4-unorm-srgb", "astc-5x5-unorm", "astc-5x5-unorm-srgb", "astc-6x5-unorm", "astc-6x5-unorm-srgb", "astc-6x6-unorm", "astc-6x6-unorm-srgb", "astc-8x5-unorm", "astc-8x5-unorm-srgb", "astc-8x6-unorm", "astc-8x6-unorm-srgb", "astc-8x8-unorm", "astc-8x8-unorm-srgb", "astc-10x5-unorm", "astc-10x5-unorm-srgb", "astc-10x6-unorm", "astc-10x6-unorm-srgb", "astc-10x8-unorm", "astc-10x8-unorm-srgb", "astc-10x10-unorm", "astc-10x10-unorm-srgb", "astc-12x10-unorm", "astc-12x10-unorm-srgb", "astc-12x12-unorm", "astc-12x12-unorm-srgb"];

const __wbindgen_enum_PremultiplyAlpha = ["none", "premultiply", "default"];

const __wbindgen_enum_RequestMode = ["same-origin", "no-cors", "cors", "navigate"];

const __wbindgen_enum_ResizeObserverBoxOptions = ["border-box", "content-box", "device-pixel-content-box"];

const __wbindgen_enum_VisibilityState = ["hidden", "visible"];

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
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
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_Window_565e23de2b798128 = function(arg0) {
        const ret = getObject(arg0).Window;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_Window_96e58b3552ce6bb1 = function(arg0) {
        const ret = getObject(arg0).Window;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_Window_cf5b693340a7c469 = function(arg0) {
        const ret = getObject(arg0).Window;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_WorkerGlobalScope_354364d1b0bd06e5 = function(arg0) {
        const ret = getObject(arg0).WorkerGlobalScope;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_WorkerGlobalScope_c3f6af5fa60d6b9c = function(arg0) {
        const ret = getObject(arg0).WorkerGlobalScope;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_abort_05026c983d86824c = function(arg0) {
        getObject(arg0).abort();
    };
    imports.wbg.__wbg_activeElement_ea31ecc5423c6046 = function(arg0) {
        const ret = getObject(arg0).activeElement;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_addEventListener_b9481c2c2cab6047 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_addListener_16cb1f9da98d0a95 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).addListener(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_altKey_d5409f5ddaa29593 = function(arg0) {
        const ret = getObject(arg0).altKey;
        return ret;
    };
    imports.wbg.__wbg_altKey_d54599b3b6b6cf22 = function(arg0) {
        const ret = getObject(arg0).altKey;
        return ret;
    };
    imports.wbg.__wbg_animate_310f1b8e5b1fdb56 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).animate(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_appendChild_d22bc7af6b96b3f1 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_arrayBuffer_d0ca2ad8bda0039b = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).arrayBuffer();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_axes_d78d107fb9a34d3d = function(arg0) {
        const ret = getObject(arg0).axes;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_beginComputePass_90d5303e604970cb = function(arg0, arg1) {
        const ret = getObject(arg0).beginComputePass(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_beginRenderPass_9739520c601001c3 = function(arg0, arg1) {
        const ret = getObject(arg0).beginRenderPass(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_blockSize_6464e214800294a9 = function(arg0) {
        const ret = getObject(arg0).blockSize;
        return ret;
    };
    imports.wbg.__wbg_blur_51f415004ecbe327 = function() { return handleError(function (arg0) {
        getObject(arg0).blur();
    }, arguments) };
    imports.wbg.__wbg_body_8d7d8c4aa91dcad8 = function(arg0) {
        const ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_brand_67e4f3d950154d0a = function(arg0, arg1) {
        const ret = getObject(arg1).brand;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_brands_1ec54d613da9db02 = function(arg0) {
        const ret = getObject(arg0).brands;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_buffer_61b7ce01341d7f88 = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_buffer_dc5dbfa8d5fb28cf = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_button_12b22015f2d5993d = function(arg0) {
        const ret = getObject(arg0).button;
        return ret;
    };
    imports.wbg.__wbg_buttons_568290cbe933b39a = function(arg0) {
        const ret = getObject(arg0).buttons;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_buttons_e83cec0abc6f937f = function(arg0) {
        const ret = getObject(arg0).buttons;
        return ret;
    };
    imports.wbg.__wbg_call_500db948e69c7330 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_b0d8e36992d9900d = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_cancelAnimationFrame_5f7904867f6ab804 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).cancelAnimationFrame(arg1);
    }, arguments) };
    imports.wbg.__wbg_cancelIdleCallback_511311272bf3c490 = function(arg0, arg1) {
        getObject(arg0).cancelIdleCallback(arg1 >>> 0);
    };
    imports.wbg.__wbg_cancel_3b69067dcdf7ffb7 = function(arg0) {
        getObject(arg0).cancel();
    };
    imports.wbg.__wbg_catch_d0fc80129c999ab3 = function(arg0, arg1) {
        const ret = getObject(arg0).catch(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_clearBuffer_6164fc25d22b25cc = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).clearBuffer(getObject(arg1), arg2, arg3);
    };
    imports.wbg.__wbg_clearBuffer_cfcaaf1fb2baa885 = function(arg0, arg1, arg2) {
        getObject(arg0).clearBuffer(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_clearTimeout_af66bc7e0dd9b02b = function(arg0, arg1) {
        getObject(arg0).clearTimeout(arg1);
    };
    imports.wbg.__wbg_clipboardData_d188aa1b339f8637 = function(arg0) {
        const ret = getObject(arg0).clipboardData;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_clipboard_b634c08b310bca2e = function(arg0) {
        const ret = getObject(arg0).clipboard;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_close_aa3f739f6c647087 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).close();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_close_b102ec699b1075d4 = function(arg0) {
        getObject(arg0).close();
    };
    imports.wbg.__wbg_code_878e1961e18ba92f = function(arg0, arg1) {
        const ret = getObject(arg1).code;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_configure_2414aed971d368cd = function(arg0, arg1) {
        getObject(arg0).configure(getObject(arg1));
    };
    imports.wbg.__wbg_connect_b5b9b381c6b728c5 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).connect(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_connected_bc8d146bbcff76c7 = function(arg0) {
        const ret = getObject(arg0).connected;
        return ret;
    };
    imports.wbg.__wbg_contains_12e954301a3dcdc7 = function(arg0, arg1) {
        const ret = getObject(arg0).contains(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_contentRect_6fadfee6731ac5df = function(arg0) {
        const ret = getObject(arg0).contentRect;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_copyBufferToBuffer_1ba67191114656a1 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).copyBufferToBuffer(getObject(arg1), arg2, getObject(arg3), arg4, arg5);
    };
    imports.wbg.__wbg_copyBufferToTexture_878d31d479e48f28 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).copyBufferToTexture(getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_copyExternalImageToTexture_7878d196c0b60d39 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).copyExternalImageToTexture(getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_copyTextureToBuffer_6a8fe0e90f0a663d = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).copyTextureToBuffer(getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_copyTextureToTexture_0a06a393d6726b4a = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).copyTextureToTexture(getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_copyToChannel_392b6d20f7a9be4e = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).copyToChannel(getArrayF32FromWasm0(arg1, arg2), arg3);
    }, arguments) };
    imports.wbg.__wbg_createBindGroupLayout_1d93b6d41c87ba9d = function(arg0, arg1) {
        const ret = getObject(arg0).createBindGroupLayout(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createBindGroup_61cd07ec9d423432 = function(arg0, arg1) {
        const ret = getObject(arg0).createBindGroup(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createBufferSource_40fde2c8ce87b45f = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).createBufferSource();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createBuffer_4c730c8729630f69 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = getObject(arg0).createBuffer(arg1 >>> 0, arg2 >>> 0, arg3);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createBuffer_963aa00d5fe859e4 = function(arg0, arg1) {
        const ret = getObject(arg0).createBuffer(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createCommandEncoder_f0e1613e9a2dc1eb = function(arg0, arg1) {
        const ret = getObject(arg0).createCommandEncoder(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createComputePipeline_b9616b9fe2f4eb2f = function(arg0, arg1) {
        const ret = getObject(arg0).createComputePipeline(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createElement_89923fcb809656b7 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createImageBitmap_64eab7825c875d8a = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).createImageBitmap(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createObjectURL_296ad2113ed20fe0 = function() { return handleError(function (arg0, arg1) {
        const ret = URL.createObjectURL(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_createPipelineLayout_56c6cf983f892d2b = function(arg0, arg1) {
        const ret = getObject(arg0).createPipelineLayout(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createQuerySet_c14be802adf7c207 = function(arg0, arg1) {
        const ret = getObject(arg0).createQuerySet(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createRenderBundleEncoder_8e4bdffea72f8c1f = function(arg0, arg1) {
        const ret = getObject(arg0).createRenderBundleEncoder(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createRenderPipeline_079a88a0601fcce1 = function(arg0, arg1) {
        const ret = getObject(arg0).createRenderPipeline(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createSampler_ef5578990df3baf7 = function(arg0, arg1) {
        const ret = getObject(arg0).createSampler(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createShaderModule_17f451ea25cae47c = function(arg0, arg1) {
        const ret = getObject(arg0).createShaderModule(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createTexture_01cc1cd2fea732d9 = function(arg0, arg1) {
        const ret = getObject(arg0).createTexture(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createView_04701884291e1ccc = function(arg0, arg1) {
        const ret = getObject(arg0).createView(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_crypto_ed58b8e10a292839 = function(arg0) {
        const ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_ctrlKey_5a324c8556fbce1c = function(arg0) {
        const ret = getObject(arg0).ctrlKey;
        return ret;
    };
    imports.wbg.__wbg_ctrlKey_5c308955b0d5492d = function(arg0) {
        const ret = getObject(arg0).ctrlKey;
        return ret;
    };
    imports.wbg.__wbg_currentTime_00c5c555e14a5405 = function(arg0) {
        const ret = getObject(arg0).currentTime;
        return ret;
    };
    imports.wbg.__wbg_data_8980cafa6731c6b5 = function(arg0, arg1) {
        const ret = getObject(arg1).data;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_decode_b1b153ae6e9ef819 = function(arg0) {
        const ret = getObject(arg0).decode();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_deltaMode_b2e9bb0dca5cf196 = function(arg0) {
        const ret = getObject(arg0).deltaMode;
        return ret;
    };
    imports.wbg.__wbg_deltaX_5c26d3b55d406732 = function(arg0) {
        const ret = getObject(arg0).deltaX;
        return ret;
    };
    imports.wbg.__wbg_deltaY_1683a859ce933add = function(arg0) {
        const ret = getObject(arg0).deltaY;
        return ret;
    };
    imports.wbg.__wbg_destination_6ff3c21ce0beee02 = function(arg0) {
        const ret = getObject(arg0).destination;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_destroy_35f94012e5bb9c17 = function(arg0) {
        getObject(arg0).destroy();
    };
    imports.wbg.__wbg_destroy_767d9dde1008e293 = function(arg0) {
        getObject(arg0).destroy();
    };
    imports.wbg.__wbg_destroy_c6af4226dda95dbd = function(arg0) {
        getObject(arg0).destroy();
    };
    imports.wbg.__wbg_devicePixelContentBoxSize_f91b326c21f7e3d5 = function(arg0) {
        const ret = getObject(arg0).devicePixelContentBoxSize;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_devicePixelRatio_973abafaa5e8254b = function(arg0) {
        const ret = getObject(arg0).devicePixelRatio;
        return ret;
    };
    imports.wbg.__wbg_disconnect_6e7f07912b7a73c6 = function(arg0) {
        getObject(arg0).disconnect();
    };
    imports.wbg.__wbg_disconnect_c350961ad9f8057a = function(arg0) {
        getObject(arg0).disconnect();
    };
    imports.wbg.__wbg_dispatchWorkgroupsIndirect_8b25efab93a7a433 = function(arg0, arg1, arg2) {
        getObject(arg0).dispatchWorkgroupsIndirect(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_dispatchWorkgroups_c102fa81b955935d = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).dispatchWorkgroups(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0);
    };
    imports.wbg.__wbg_document_f11bc4f7c03e1745 = function(arg0) {
        const ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_drawIndexedIndirect_34484fc6227c7bc8 = function(arg0, arg1, arg2) {
        getObject(arg0).drawIndexedIndirect(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_drawIndexedIndirect_5a7c30bb5f1d5b67 = function(arg0, arg1, arg2) {
        getObject(arg0).drawIndexedIndirect(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_drawIndexed_115af1449b52a948 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).drawIndexed(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5 >>> 0);
    };
    imports.wbg.__wbg_drawIndexed_a587cce4c317791f = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).drawIndexed(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5 >>> 0);
    };
    imports.wbg.__wbg_drawIndirect_036d71498a21f1a3 = function(arg0, arg1, arg2) {
        getObject(arg0).drawIndirect(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_drawIndirect_a1d7c5e893aa5756 = function(arg0, arg1, arg2) {
        getObject(arg0).drawIndirect(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_draw_5351b12033166aca = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).draw(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
    };
    imports.wbg.__wbg_draw_e2a7c5d66fb2d244 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).draw(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
    };
    imports.wbg.__wbg_end_0ac71677a5c1717a = function(arg0) {
        getObject(arg0).end();
    };
    imports.wbg.__wbg_end_6f776519f1faa582 = function(arg0) {
        getObject(arg0).end();
    };
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_export_3(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_error_bc396fc38839dd25 = function(arg0, arg1) {
        console.error(getObject(arg0), getObject(arg1));
    };
    imports.wbg.__wbg_error_e98e6aadd08e0b94 = function(arg0) {
        const ret = getObject(arg0).error;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_eval_cd0c386c3899dd07 = function() { return handleError(function (arg0, arg1) {
        const ret = eval(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_exec_2630a18b88ec1a61 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).exec(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_executeBundles_8e6c0614da2805d4 = function(arg0, arg1) {
        getObject(arg0).executeBundles(getObject(arg1));
    };
    imports.wbg.__wbg_exitFullscreen_c9c2c41beb64ef2f = function(arg0) {
        getObject(arg0).exitFullscreen();
    };
    imports.wbg.__wbg_exitPointerLock_bbd1b0a5297d428d = function(arg0) {
        getObject(arg0).exitPointerLock();
    };
    imports.wbg.__wbg_features_1b464383ea8a7691 = function(arg0) {
        const ret = getObject(arg0).features;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_features_e5fbbc2760867852 = function(arg0) {
        const ret = getObject(arg0).features;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_fetch_58cc163d77b34fb1 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).fetch(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_fetch_6a5bc16a35f71316 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).fetch(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_fetch_e26fdd92ea39f634 = function(arg0, arg1) {
        const ret = getObject(arg0).fetch(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_files_576e546a364f9971 = function(arg0) {
        const ret = getObject(arg0).files;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_finish_20711371c58df61c = function(arg0) {
        const ret = getObject(arg0).finish();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_finish_34b2c54329c8719f = function(arg0, arg1) {
        const ret = getObject(arg0).finish(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_finish_a9ab917e756ea00c = function(arg0, arg1) {
        const ret = getObject(arg0).finish(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_finish_e0a6c97c0622f843 = function(arg0) {
        const ret = getObject(arg0).finish();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_focus_35fe945f7268dd62 = function() { return handleError(function (arg0) {
        getObject(arg0).focus();
    }, arguments) };
    imports.wbg.__wbg_fullscreenElement_d6cbbb5d5f0362c4 = function(arg0) {
        const ret = getObject(arg0).fullscreenElement;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_getBindGroupLayout_4a94df6108ac6667 = function(arg0, arg1) {
        const ret = getObject(arg0).getBindGroupLayout(arg1 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getBindGroupLayout_80e803d942962f6a = function(arg0, arg1) {
        const ret = getObject(arg0).getBindGroupLayout(arg1 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getBoundingClientRect_05c4b9e3701bb372 = function(arg0) {
        const ret = getObject(arg0).getBoundingClientRect();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getCoalescedEvents_1372a1922f78401a = function(arg0) {
        const ret = getObject(arg0).getCoalescedEvents;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getCoalescedEvents_f51eabe0483efd6f = function(arg0) {
        const ret = getObject(arg0).getCoalescedEvents();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getCompilationInfo_2af3ecdfeda551a3 = function(arg0) {
        const ret = getObject(arg0).getCompilationInfo();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getComputedStyle_8e58bbd76370e2b1 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).getComputedStyle(getObject(arg1));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getContext_5eaf5645cd6acb46 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getContext_76b028baa6dafdeb = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getCurrentTexture_5a79cda2ff36e1ee = function(arg0) {
        const ret = getObject(arg0).getCurrentTexture();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getData_a7e2258d0a97ce17 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = getObject(arg1).getData(getStringFromWasm0(arg2, arg3));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_getElementById_dcc9f1f3cfdca0bc = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_getGamepads_e8ce23216a222f56 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).getGamepads();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getMappedRange_932dd043ae22ee0a = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).getMappedRange(arg1, arg2);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getOwnPropertyDescriptor_da0bd3d3d60cf5c1 = function(arg0, arg1) {
        const ret = Object.getOwnPropertyDescriptor(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getPreferredCanvasFormat_de73c02773a5209e = function(arg0) {
        const ret = getObject(arg0).getPreferredCanvasFormat();
        return (__wbindgen_enum_GpuTextureFormat.indexOf(ret) + 1 || 96) - 1;
    };
    imports.wbg.__wbg_getPropertyValue_66c16bac362c6d90 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = getObject(arg1).getPropertyValue(getStringFromWasm0(arg2, arg3));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_getRandomValues_bcb4912f16000dc4 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_get_198bfcfce5b3a38f = function(arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_get_9901e5f7f90821fc = function(arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_get_9aa3dff3f0266054 = function(arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_gpu_87871e8f7ace8fee = function(arg0) {
        const ret = getObject(arg0).gpu;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_has_624cbf0451d880e8 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).has(getStringFromWasm0(arg1, arg2));
        return ret;
    };
    imports.wbg.__wbg_height_08fd44318e18021d = function(arg0) {
        const ret = getObject(arg0).height;
        return ret;
    };
    imports.wbg.__wbg_hidden_d735f73b279e259f = function(arg0) {
        const ret = getObject(arg0).hidden;
        return ret;
    };
    imports.wbg.__wbg_id_5171f0812b638d4b = function(arg0, arg1) {
        const ret = getObject(arg1).id;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_index_229b6e37ebc563dc = function(arg0) {
        const ret = getObject(arg0).index;
        return ret;
    };
    imports.wbg.__wbg_inlineSize_60da5bea0a6275d2 = function(arg0) {
        const ret = getObject(arg0).inlineSize;
        return ret;
    };
    imports.wbg.__wbg_instanceof_DomException_aafa436ce5fecc64 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof DOMException;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_GpuAdapter_0731153d2b08720b = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof GPUAdapter;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_GpuCanvasContext_d14121c7bd72fcef = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof GPUCanvasContext;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_GpuDeviceLostInfo_a3677ebb8241d800 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof GPUDeviceLostInfo;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_GpuOutOfMemoryError_391d9a08edbfa04b = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof GPUOutOfMemoryError;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_GpuValidationError_f4d803c383da3c92 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof GPUValidationError;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlButtonElement_44ea79d0fd2116a3 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLButtonElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlCanvasElement_f764441ef5ddb63f = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLCanvasElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlElement_d94ed69c6883a691 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlInputElement_47b3e827f364773c = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLInputElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Object_0d0cec232ff037c4 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Object;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Response_d3453657e10c4300 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Response;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Window_d2514c6a7ee7ba60 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Window;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_isComposing_34930e03980aa623 = function(arg0) {
        const ret = getObject(arg0).isComposing;
        return ret;
    };
    imports.wbg.__wbg_isComposing_e6c0e75812df1d39 = function(arg0) {
        const ret = getObject(arg0).isComposing;
        return ret;
    };
    imports.wbg.__wbg_isIntersecting_03f2dfd4beb70720 = function(arg0) {
        const ret = getObject(arg0).isIntersecting;
        return ret;
    };
    imports.wbg.__wbg_isSecureContext_56a612937252b7c4 = function(arg0) {
        const ret = getObject(arg0).isSecureContext;
        return ret;
    };
    imports.wbg.__wbg_is_e442492d1fb7967b = function(arg0, arg1) {
        const ret = Object.is(getObject(arg0), getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_keyCode_e673401ed53dfc2c = function(arg0) {
        const ret = getObject(arg0).keyCode;
        return ret;
    };
    imports.wbg.__wbg_key_9a40d4f6defa675b = function(arg0, arg1) {
        const ret = getObject(arg1).key;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_label_2082ab37d2ad170d = function(arg0, arg1) {
        const ret = getObject(arg1).label;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_length_65d1cd11729ced11 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_length_9df32f7add647235 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_length_c4528fc455e58194 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_length_d65cf0786bfc5739 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_limits_2dd632c891786ddf = function(arg0) {
        const ret = getObject(arg0).limits;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_limits_f6411f884b0b2d62 = function(arg0) {
        const ret = getObject(arg0).limits;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_lineNum_0246de1e072ffe19 = function(arg0) {
        const ret = getObject(arg0).lineNum;
        return ret;
    };
    imports.wbg.__wbg_location_0d3ce589878cba8a = function(arg0) {
        const ret = getObject(arg0).location;
        return ret;
    };
    imports.wbg.__wbg_log_0cc1b7768397bcfe = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.log(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3), getStringFromWasm0(arg4, arg5), getStringFromWasm0(arg6, arg7));
        } finally {
            wasm.__wbindgen_export_3(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_log_cb9e190acc5753fb = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.log(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_export_3(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_lost_6e4d29847ce2a34a = function(arg0) {
        const ret = getObject(arg0).lost;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_mapAsync_37f5e03edf2e1352 = function(arg0, arg1, arg2, arg3) {
        const ret = getObject(arg0).mapAsync(arg1 >>> 0, arg2, arg3);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_mapping_95e5c8da16bcf133 = function(arg0) {
        const ret = getObject(arg0).mapping;
        return (__wbindgen_enum_GamepadMappingType.indexOf(ret) + 1 || 3) - 1;
    };
    imports.wbg.__wbg_mark_7438147ce31e9d4b = function(arg0, arg1) {
        performance.mark(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_matchMedia_4adca948756a5784 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).matchMedia(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_matches_a69a36077c4f07ad = function(arg0) {
        const ret = getObject(arg0).matches;
        return ret;
    };
    imports.wbg.__wbg_maxBindGroups_768ca5e8623bf450 = function(arg0) {
        const ret = getObject(arg0).maxBindGroups;
        return ret;
    };
    imports.wbg.__wbg_maxBindingsPerBindGroup_057972d600d69719 = function(arg0) {
        const ret = getObject(arg0).maxBindingsPerBindGroup;
        return ret;
    };
    imports.wbg.__wbg_maxBufferSize_e237b44f19a5a62b = function(arg0) {
        const ret = getObject(arg0).maxBufferSize;
        return ret;
    };
    imports.wbg.__wbg_maxChannelCount_66dd2366c7432e92 = function(arg0) {
        const ret = getObject(arg0).maxChannelCount;
        return ret;
    };
    imports.wbg.__wbg_maxColorAttachmentBytesPerSample_d6c7b4051d22c6d6 = function(arg0) {
        const ret = getObject(arg0).maxColorAttachmentBytesPerSample;
        return ret;
    };
    imports.wbg.__wbg_maxColorAttachments_7a18ba24c05edcfd = function(arg0) {
        const ret = getObject(arg0).maxColorAttachments;
        return ret;
    };
    imports.wbg.__wbg_maxComputeInvocationsPerWorkgroup_b99c2f3611633992 = function(arg0) {
        const ret = getObject(arg0).maxComputeInvocationsPerWorkgroup;
        return ret;
    };
    imports.wbg.__wbg_maxComputeWorkgroupSizeX_adb26da9ed7f77f7 = function(arg0) {
        const ret = getObject(arg0).maxComputeWorkgroupSizeX;
        return ret;
    };
    imports.wbg.__wbg_maxComputeWorkgroupSizeY_cc217559c98be33b = function(arg0) {
        const ret = getObject(arg0).maxComputeWorkgroupSizeY;
        return ret;
    };
    imports.wbg.__wbg_maxComputeWorkgroupSizeZ_66606a80e2cf2309 = function(arg0) {
        const ret = getObject(arg0).maxComputeWorkgroupSizeZ;
        return ret;
    };
    imports.wbg.__wbg_maxComputeWorkgroupStorageSize_cb6235497b8c4997 = function(arg0) {
        const ret = getObject(arg0).maxComputeWorkgroupStorageSize;
        return ret;
    };
    imports.wbg.__wbg_maxComputeWorkgroupsPerDimension_6bf550b5f21d57cf = function(arg0) {
        const ret = getObject(arg0).maxComputeWorkgroupsPerDimension;
        return ret;
    };
    imports.wbg.__wbg_maxDynamicStorageBuffersPerPipelineLayout_c6ac20334e328b47 = function(arg0) {
        const ret = getObject(arg0).maxDynamicStorageBuffersPerPipelineLayout;
        return ret;
    };
    imports.wbg.__wbg_maxDynamicUniformBuffersPerPipelineLayout_aa8f14a74b440f01 = function(arg0) {
        const ret = getObject(arg0).maxDynamicUniformBuffersPerPipelineLayout;
        return ret;
    };
    imports.wbg.__wbg_maxSampledTexturesPerShaderStage_db7c4922cc60144a = function(arg0) {
        const ret = getObject(arg0).maxSampledTexturesPerShaderStage;
        return ret;
    };
    imports.wbg.__wbg_maxSamplersPerShaderStage_538705fe2263e710 = function(arg0) {
        const ret = getObject(arg0).maxSamplersPerShaderStage;
        return ret;
    };
    imports.wbg.__wbg_maxStorageBufferBindingSize_32178c0f5f7f85cb = function(arg0) {
        const ret = getObject(arg0).maxStorageBufferBindingSize;
        return ret;
    };
    imports.wbg.__wbg_maxStorageBuffersPerShaderStage_9f67e9eae0089f77 = function(arg0) {
        const ret = getObject(arg0).maxStorageBuffersPerShaderStage;
        return ret;
    };
    imports.wbg.__wbg_maxStorageTexturesPerShaderStage_57239664936031cf = function(arg0) {
        const ret = getObject(arg0).maxStorageTexturesPerShaderStage;
        return ret;
    };
    imports.wbg.__wbg_maxTextureArrayLayers_db5d4e486c78ae04 = function(arg0) {
        const ret = getObject(arg0).maxTextureArrayLayers;
        return ret;
    };
    imports.wbg.__wbg_maxTextureDimension1D_3475085ffacabbdc = function(arg0) {
        const ret = getObject(arg0).maxTextureDimension1D;
        return ret;
    };
    imports.wbg.__wbg_maxTextureDimension2D_7c8d5ecf09eb8519 = function(arg0) {
        const ret = getObject(arg0).maxTextureDimension2D;
        return ret;
    };
    imports.wbg.__wbg_maxTextureDimension3D_8bd976677a0f91d4 = function(arg0) {
        const ret = getObject(arg0).maxTextureDimension3D;
        return ret;
    };
    imports.wbg.__wbg_maxUniformBufferBindingSize_95b1a54e7e4a0f0f = function(arg0) {
        const ret = getObject(arg0).maxUniformBufferBindingSize;
        return ret;
    };
    imports.wbg.__wbg_maxUniformBuffersPerShaderStage_5f475d9a453af14d = function(arg0) {
        const ret = getObject(arg0).maxUniformBuffersPerShaderStage;
        return ret;
    };
    imports.wbg.__wbg_maxVertexAttributes_4c48ca2f5d32f860 = function(arg0) {
        const ret = getObject(arg0).maxVertexAttributes;
        return ret;
    };
    imports.wbg.__wbg_maxVertexBufferArrayStride_2233f6933ecc5a16 = function(arg0) {
        const ret = getObject(arg0).maxVertexBufferArrayStride;
        return ret;
    };
    imports.wbg.__wbg_maxVertexBuffers_c47e508cd7348554 = function(arg0) {
        const ret = getObject(arg0).maxVertexBuffers;
        return ret;
    };
    imports.wbg.__wbg_measure_fb7825c11612c823 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        let deferred0_0;
        let deferred0_1;
        let deferred1_0;
        let deferred1_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            deferred1_0 = arg2;
            deferred1_1 = arg3;
            performance.measure(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
        } finally {
            wasm.__wbindgen_export_3(deferred0_0, deferred0_1, 1);
            wasm.__wbindgen_export_3(deferred1_0, deferred1_1, 1);
        }
    }, arguments) };
    imports.wbg.__wbg_media_276296e08383c7dc = function(arg0, arg1) {
        const ret = getObject(arg1).media;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_message_0762358e59db7ed6 = function(arg0, arg1) {
        const ret = getObject(arg1).message;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_message_7957ab09f64c6822 = function(arg0, arg1) {
        const ret = getObject(arg1).message;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_message_b163994503433c9e = function(arg0, arg1) {
        const ret = getObject(arg1).message;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_message_d19bdb65f0a41cf5 = function(arg0, arg1) {
        const ret = getObject(arg1).message;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_messages_da071582f72bc978 = function(arg0) {
        const ret = getObject(arg0).messages;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_metaKey_90fbd812345a7e0c = function(arg0) {
        const ret = getObject(arg0).metaKey;
        return ret;
    };
    imports.wbg.__wbg_metaKey_de1f08a4d1e84bd1 = function(arg0) {
        const ret = getObject(arg0).metaKey;
        return ret;
    };
    imports.wbg.__wbg_minStorageBufferOffsetAlignment_51b4801fac3a58de = function(arg0) {
        const ret = getObject(arg0).minStorageBufferOffsetAlignment;
        return ret;
    };
    imports.wbg.__wbg_minUniformBufferOffsetAlignment_5d62a77924b2335f = function(arg0) {
        const ret = getObject(arg0).minUniformBufferOffsetAlignment;
        return ret;
    };
    imports.wbg.__wbg_movementX_2e17c283b7c8114b = function(arg0) {
        const ret = getObject(arg0).movementX;
        return ret;
    };
    imports.wbg.__wbg_movementY_311d0d64dcbbb4a1 = function(arg0) {
        const ret = getObject(arg0).movementY;
        return ret;
    };
    imports.wbg.__wbg_msCrypto_0a36e2ec3a343d26 = function(arg0) {
        const ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_name_37e12d7b980bc5bd = function(arg0, arg1) {
        const ret = getObject(arg1).name;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_navigator_0fe968937104eaa7 = function(arg0) {
        const ret = getObject(arg0).navigator;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_navigator_d6db0fcbd7865e08 = function(arg0) {
        const ret = getObject(arg0).navigator;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_0565c3001775c60a = function() { return handleError(function (arg0) {
        const ret = new ResizeObserver(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_254fa9eac11932ae = function() {
        const ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_37eac52f406c308d = function(arg0, arg1, arg2, arg3) {
        const ret = new RegExp(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_3d446df9155128ef = function(arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_71(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            const ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_new_3d662e5b2b6548c5 = function() { return handleError(function (arg0, arg1) {
        const ret = new Worker(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_3ff5b33b1ce712df = function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_463f1efab237d7e0 = function() { return handleError(function (arg0) {
        const ret = new IntersectionObserver(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_5f48f21d4be11586 = function() { return handleError(function () {
        const ret = new AbortController();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_688846f374351c92 = function() {
        const ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_a01d9d610b795c1f = function() { return handleError(function () {
        const ret = new FileReader();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_c60de43ba24f2df6 = function() { return handleError(function () {
        const ret = new Image();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_f099e287c6799c7c = function() { return handleError(function () {
        const ret = new MessageChannel();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newnoargs_fd9e4bf8be2bc16d = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_ba35896968751d91 = function(arg0, arg1, arg2) {
        const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithcontextoptions_650227aa16d5f30e = function() { return handleError(function (arg0) {
        const ret = new lAudioContext(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newwithlength_34ce8f1051e74449 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithstrandinit_a1f6583f20e4faff = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = new Request(getStringFromWasm0(arg0, arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newwithstrsequenceandoptions_c12c1efe3dd90e2c = function() { return handleError(function (arg0, arg1) {
        const ret = new Blob(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newwithu8arraysequenceandoptions_75a3b40c32d6c988 = function() { return handleError(function (arg0, arg1) {
        const ret = new Blob(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newwithu8clampedarray_ad487d49f3d96ada = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = new ImageData(getClampedArrayU8FromWasm0(arg0, arg1), arg2 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_node_02999533c4ea02e3 = function(arg0) {
        const ret = getObject(arg0).node;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_now_2c95c9de01293173 = function(arg0) {
        const ret = getObject(arg0).now();
        return ret;
    };
    imports.wbg.__wbg_now_64d0bb151e5d3889 = function() {
        const ret = Date.now();
        return ret;
    };
    imports.wbg.__wbg_observe_4312463ceec4f579 = function(arg0, arg1) {
        getObject(arg0).observe(getObject(arg1));
    };
    imports.wbg.__wbg_observe_5777bdcf09e4bdde = function(arg0, arg1) {
        getObject(arg0).observe(getObject(arg1));
    };
    imports.wbg.__wbg_observe_71a44d88a2880088 = function(arg0, arg1, arg2) {
        getObject(arg0).observe(getObject(arg1), getObject(arg2));
    };
    imports.wbg.__wbg_of_437cdae2760f8b94 = function(arg0, arg1) {
        const ret = Array.of(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_of_924412d32367b13d = function(arg0) {
        const ret = Array.of(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_offsetX_d6957d052a0d4b11 = function(arg0) {
        const ret = getObject(arg0).offsetX;
        return ret;
    };
    imports.wbg.__wbg_offsetY_076edde514bf80f4 = function(arg0) {
        const ret = getObject(arg0).offsetY;
        return ret;
    };
    imports.wbg.__wbg_offset_336f14c993863b76 = function(arg0) {
        const ret = getObject(arg0).offset;
        return ret;
    };
    imports.wbg.__wbg_open_94e1e54493f5c069 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        const ret = getObject(arg0).open(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_performance_7a3ffd0b17f663ad = function(arg0) {
        const ret = getObject(arg0).performance;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_persisted_fa8c29c59197f243 = function(arg0) {
        const ret = getObject(arg0).persisted;
        return ret;
    };
    imports.wbg.__wbg_play_e7eb90a843114e0d = function(arg0) {
        getObject(arg0).play();
    };
    imports.wbg.__wbg_pointerId_85845d98372f1198 = function(arg0) {
        const ret = getObject(arg0).pointerId;
        return ret;
    };
    imports.wbg.__wbg_pointerType_4d6a147d076e7aae = function(arg0, arg1) {
        const ret = getObject(arg1).pointerType;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_popErrorScope_af0b22f136a861d6 = function(arg0) {
        const ret = getObject(arg0).popErrorScope();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_port1_bda6bd389f4cc28a = function(arg0) {
        const ret = getObject(arg0).port1;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_port2_b250432baff0d94d = function(arg0) {
        const ret = getObject(arg0).port2;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_postMessage_586e33893b92051e = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).postMessage(getObject(arg1), getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_postMessage_73c83cb8c6103c58 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).postMessage(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_postTask_9ba4c3cedae00b38 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).postTask(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_pressed_6ce8b91e53f226e3 = function(arg0) {
        const ret = getObject(arg0).pressed;
        return ret;
    };
    imports.wbg.__wbg_pressure_c345c07c94ad38cf = function(arg0) {
        const ret = getObject(arg0).pressure;
        return ret;
    };
    imports.wbg.__wbg_preventDefault_3c86e59772d015e6 = function(arg0) {
        getObject(arg0).preventDefault();
    };
    imports.wbg.__wbg_process_5c1d670bc53614b8 = function(arg0) {
        const ret = getObject(arg0).process;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_prototype_0f68911d93450df4 = function() {
        const ret = ResizeObserverEntry.prototype;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_pushErrorScope_b52914ff10ba6ce3 = function(arg0, arg1) {
        getObject(arg0).pushErrorScope(__wbindgen_enum_GpuErrorFilter[arg1]);
    };
    imports.wbg.__wbg_push_6edad0df4b546b2c = function(arg0, arg1) {
        const ret = getObject(arg0).push(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_querySelectorAll_2d037c571f099149 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).querySelectorAll(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_querySelector_7b4362006fdeda68 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).querySelector(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_queueMicrotask_2181040e064c0dc8 = function(arg0) {
        queueMicrotask(getObject(arg0));
    };
    imports.wbg.__wbg_queueMicrotask_5fc3e400ac3c03f4 = function(arg0) {
        queueMicrotask(takeObject(arg0));
    };
    imports.wbg.__wbg_queueMicrotask_ef9ac43769cbcc4f = function(arg0) {
        const ret = getObject(arg0).queueMicrotask;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_queue_bea4017efaaf9904 = function(arg0) {
        const ret = getObject(arg0).queue;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_randomFillSync_ab2cfe79ebbf2740 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).randomFillSync(takeObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_readAsArrayBuffer_db7f197b5b6b34cf = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).readAsArrayBuffer(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_reason_43acd39cce242b50 = function(arg0) {
        const ret = getObject(arg0).reason;
        return (__wbindgen_enum_GpuDeviceLostReason.indexOf(ret) + 1 || 3) - 1;
    };
    imports.wbg.__wbg_removeEventListener_a9ca9f05245321f0 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_removeListener_00f6b4d348de1d99 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).removeListener(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_removeProperty_ac500eb39cba5510 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = getObject(arg1).removeProperty(getStringFromWasm0(arg2, arg3));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_remove_530b4f3163f72a83 = function(arg0) {
        getObject(arg0).remove();
    };
    imports.wbg.__wbg_repeat_621b3806d8c52204 = function(arg0) {
        const ret = getObject(arg0).repeat;
        return ret;
    };
    imports.wbg.__wbg_requestAdapter_e6dcfac497cafa7a = function(arg0, arg1) {
        const ret = getObject(arg0).requestAdapter(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_requestAnimationFrame_169cbbda5861d9ca = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_requestDevice_03b802707d5a382c = function(arg0, arg1) {
        const ret = getObject(arg0).requestDevice(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_requestFullscreen_e1ad8752a4e33f7e = function(arg0) {
        const ret = getObject(arg0).requestFullscreen;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_requestFullscreen_f727b1f250ebb10a = function(arg0) {
        const ret = getObject(arg0).requestFullscreen();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_requestIdleCallback_6520e3e96167e941 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).requestIdleCallback(getObject(arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_requestIdleCallback_773c554fc8c5a310 = function(arg0) {
        const ret = getObject(arg0).requestIdleCallback;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_requestPointerLock_424ca2a6bd3e5542 = function(arg0) {
        getObject(arg0).requestPointerLock();
    };
    imports.wbg.__wbg_require_79b1e9274cde3c87 = function() { return handleError(function () {
        const ret = module.require;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_resolveQuerySet_811661fb23f3b699 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).resolveQuerySet(getObject(arg1), arg2 >>> 0, arg3 >>> 0, getObject(arg4), arg5 >>> 0);
    };
    imports.wbg.__wbg_resolve_0bf7c44d641804f9 = function(arg0) {
        const ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_result_b7f693658f393a91 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).result;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_resume_fa7be90350810c5a = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).resume();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_revokeObjectURL_efa90403a85af243 = function() { return handleError(function (arg0, arg1) {
        URL.revokeObjectURL(getStringFromWasm0(arg0, arg1));
    }, arguments) };
    imports.wbg.__wbg_scheduler_10edeee35a76c7c6 = function(arg0) {
        const ret = getObject(arg0).scheduler;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_scheduler_6d71128f17fbca34 = function(arg0) {
        const ret = getObject(arg0).scheduler;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setAttribute_148e0e65e20e5f27 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_setBindGroup_62a3045b0921e429 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).setBindGroup(arg1 >>> 0, getObject(arg2), getArrayU32FromWasm0(arg3, arg4), arg5, arg6 >>> 0);
    };
    imports.wbg.__wbg_setBindGroup_6c0fd18e9a53a945 = function(arg0, arg1, arg2) {
        getObject(arg0).setBindGroup(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_setBindGroup_7f3b61f1f482133b = function(arg0, arg1, arg2) {
        getObject(arg0).setBindGroup(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_setBindGroup_bf767a5aa46a33ce = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).setBindGroup(arg1 >>> 0, getObject(arg2), getArrayU32FromWasm0(arg3, arg4), arg5, arg6 >>> 0);
    };
    imports.wbg.__wbg_setBindGroup_c4aaff14063226b4 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).setBindGroup(arg1 >>> 0, getObject(arg2), getArrayU32FromWasm0(arg3, arg4), arg5, arg6 >>> 0);
    };
    imports.wbg.__wbg_setBindGroup_f82e771dc1b69093 = function(arg0, arg1, arg2) {
        getObject(arg0).setBindGroup(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_setBlendConstant_016723821cfb3aa4 = function(arg0, arg1) {
        getObject(arg0).setBlendConstant(getObject(arg1));
    };
    imports.wbg.__wbg_setIndexBuffer_286a40afdff411b7 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).setIndexBuffer(getObject(arg1), __wbindgen_enum_GpuIndexFormat[arg2], arg3);
    };
    imports.wbg.__wbg_setIndexBuffer_7efd0b7a40c65fb9 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setIndexBuffer(getObject(arg1), __wbindgen_enum_GpuIndexFormat[arg2], arg3, arg4);
    };
    imports.wbg.__wbg_setIndexBuffer_e091a9673bb575e2 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).setIndexBuffer(getObject(arg1), __wbindgen_enum_GpuIndexFormat[arg2], arg3);
    };
    imports.wbg.__wbg_setIndexBuffer_f0759f00036f615f = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setIndexBuffer(getObject(arg1), __wbindgen_enum_GpuIndexFormat[arg2], arg3, arg4);
    };
    imports.wbg.__wbg_setPipeline_ba92070b8ee81cf9 = function(arg0, arg1) {
        getObject(arg0).setPipeline(getObject(arg1));
    };
    imports.wbg.__wbg_setPipeline_c344f76bae58c4d6 = function(arg0, arg1) {
        getObject(arg0).setPipeline(getObject(arg1));
    };
    imports.wbg.__wbg_setPipeline_d76451c50a121598 = function(arg0, arg1) {
        getObject(arg0).setPipeline(getObject(arg1));
    };
    imports.wbg.__wbg_setPointerCapture_e566a9828fac9a43 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).setPointerCapture(arg1);
    }, arguments) };
    imports.wbg.__wbg_setProperty_0eb9705cf1b05650 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_setScissorRect_0b6ee0852ef0b6b9 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setScissorRect(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
    };
    imports.wbg.__wbg_setStencilReference_34fd3d59673a5a9d = function(arg0, arg1) {
        getObject(arg0).setStencilReference(arg1 >>> 0);
    };
    imports.wbg.__wbg_setTimeout_8d2afdcdb34b4e5a = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setTimeout_e60aa33ae1e92d5a = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).setTimeout(getObject(arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setVertexBuffer_06a90dc78e1ad9c4 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setVertexBuffer(arg1 >>> 0, getObject(arg2), arg3, arg4);
    };
    imports.wbg.__wbg_setVertexBuffer_1540e9118b6c451d = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).setVertexBuffer(arg1 >>> 0, getObject(arg2), arg3);
    };
    imports.wbg.__wbg_setVertexBuffer_5166eedc06450701 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setVertexBuffer(arg1 >>> 0, getObject(arg2), arg3, arg4);
    };
    imports.wbg.__wbg_setVertexBuffer_8621784e5014065b = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).setVertexBuffer(arg1 >>> 0, getObject(arg2), arg3);
    };
    imports.wbg.__wbg_setViewport_731ad30abb13f744 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).setViewport(arg1, arg2, arg3, arg4, arg5, arg6);
    };
    imports.wbg.__wbg_set_23d69db4e5c66a6e = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_set_4e647025551483bd = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setaccept_52df39868582bd90 = function(arg0, arg1, arg2) {
        getObject(arg0).accept = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setautofocus_6ebfa477d4cba31b = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).autofocus = arg1 !== 0;
    }, arguments) };
    imports.wbg.__wbg_setbox_2c55cd020a2888a8 = function(arg0, arg1) {
        getObject(arg0).box = __wbindgen_enum_ResizeObserverBoxOptions[arg1];
    };
    imports.wbg.__wbg_setbuffer_94908cf7eea525e9 = function(arg0, arg1) {
        getObject(arg0).buffer = getObject(arg1);
    };
    imports.wbg.__wbg_setchannelCount_089f29a4056f0730 = function(arg0, arg1) {
        getObject(arg0).channelCount = arg1 >>> 0;
    };
    imports.wbg.__wbg_setcursor_c96a9337962c3df1 = function(arg0, arg1, arg2) {
        getObject(arg0).cursor = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setdownload_6f17b767672b31f5 = function(arg0, arg1, arg2) {
        getObject(arg0).download = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setduration_e63f671ee21618f9 = function(arg0, arg1) {
        getObject(arg0).duration = arg1;
    };
    imports.wbg.__wbg_setheight_16d76e7fa9d506ea = function(arg0, arg1) {
        getObject(arg0).height = arg1 >>> 0;
    };
    imports.wbg.__wbg_setheight_9805cc527c3e4d65 = function(arg0, arg1) {
        getObject(arg0).height = arg1 >>> 0;
    };
    imports.wbg.__wbg_sethidden_4cafe341b727499d = function(arg0, arg1) {
        getObject(arg0).hidden = arg1 !== 0;
    };
    imports.wbg.__wbg_sethref_272aef65ff8abb6e = function(arg0, arg1, arg2) {
        getObject(arg0).href = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setid_b6aa394f9a8400f5 = function(arg0, arg1, arg2) {
        getObject(arg0).id = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setinnerHTML_2d75307ba8832258 = function(arg0, arg1, arg2) {
        getObject(arg0).innerHTML = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setinnerText_a9690b82c4cb9063 = function(arg0, arg1, arg2) {
        getObject(arg0).innerText = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setiterations_0c25c370b1dd2d0d = function(arg0, arg1) {
        getObject(arg0).iterations = arg1;
    };
    imports.wbg.__wbg_setmethod_cfc7f688ba46a6be = function(arg0, arg1, arg2) {
        getObject(arg0).method = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setmode_cd03637eb7da01e0 = function(arg0, arg1) {
        getObject(arg0).mode = __wbindgen_enum_RequestMode[arg1];
    };
    imports.wbg.__wbg_setmultiple_811c4c7d8b43520b = function(arg0, arg1) {
        getObject(arg0).multiple = arg1 !== 0;
    };
    imports.wbg.__wbg_setonclick_096859d6b1b46a82 = function(arg0, arg1) {
        getObject(arg0).onclick = getObject(arg1);
    };
    imports.wbg.__wbg_setonended_42d712d61e631fd6 = function(arg0, arg1) {
        getObject(arg0).onended = getObject(arg1);
    };
    imports.wbg.__wbg_setonload_36cf7239551d2544 = function(arg0, arg1) {
        getObject(arg0).onload = getObject(arg1);
    };
    imports.wbg.__wbg_setonmessage_4e389f08c5e3d852 = function(arg0, arg1) {
        getObject(arg0).onmessage = getObject(arg1);
    };
    imports.wbg.__wbg_setonuncapturederror_19541466822d790b = function(arg0, arg1) {
        getObject(arg0).onuncapturederror = getObject(arg1);
    };
    imports.wbg.__wbg_setpremultiplyalpha_b1b58299e943b0df = function(arg0, arg1) {
        getObject(arg0).premultiplyAlpha = __wbindgen_enum_PremultiplyAlpha[arg1];
    };
    imports.wbg.__wbg_setsamplerate_aafb90591adbc778 = function(arg0, arg1) {
        getObject(arg0).sampleRate = arg1;
    };
    imports.wbg.__wbg_setsize_47b33a778ed778d3 = function(arg0, arg1) {
        getObject(arg0).size = arg1 >>> 0;
    };
    imports.wbg.__wbg_setsrc_99e2795c356bc837 = function(arg0, arg1, arg2) {
        getObject(arg0).src = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_settype_e19ab551722d5681 = function(arg0, arg1, arg2) {
        getObject(arg0).type = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_settype_fd39465d237c2f36 = function(arg0, arg1, arg2) {
        getObject(arg0).type = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setvalue_44c59c360ad57cf0 = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setwidth_4afeb01eae9784dd = function(arg0, arg1) {
        getObject(arg0).width = arg1 >>> 0;
    };
    imports.wbg.__wbg_setwidth_c588fe07a7982aca = function(arg0, arg1) {
        getObject(arg0).width = arg1 >>> 0;
    };
    imports.wbg.__wbg_shiftKey_0d6625838238aee8 = function(arg0) {
        const ret = getObject(arg0).shiftKey;
        return ret;
    };
    imports.wbg.__wbg_shiftKey_4b30f68655b97001 = function(arg0) {
        const ret = getObject(arg0).shiftKey;
        return ret;
    };
    imports.wbg.__wbg_signal_1fdadeba2d04660e = function(arg0) {
        const ret = getObject(arg0).signal;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_size_661bddb3f9898121 = function(arg0) {
        const ret = getObject(arg0).size;
        return ret;
    };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_start_468bfa26bc34cd1c = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).start(arg1);
    }, arguments) };
    imports.wbg.__wbg_start_4c1bf0eed2ea2a6c = function(arg0) {
        getObject(arg0).start();
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_0be7472e492ad3e3 = function() {
        const ret = typeof global === 'undefined' ? null : global;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_THIS_1a6eb482d12c9bfb = function() {
        const ret = typeof globalThis === 'undefined' ? null : globalThis;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_static_accessor_SELF_1dc398a895c82351 = function() {
        const ret = typeof self === 'undefined' ? null : self;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_static_accessor_WINDOW_ae1c80c7eea8d64a = function() {
        const ret = typeof window === 'undefined' ? null : window;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_status_317f53bc4c7638df = function(arg0) {
        const ret = getObject(arg0).status;
        return ret;
    };
    imports.wbg.__wbg_stringify_f4f701bc34ceda61 = function() { return handleError(function (arg0) {
        const ret = JSON.stringify(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_style_53bb2d762dd1c030 = function(arg0) {
        const ret = getObject(arg0).style;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_subarray_46adeb9b86949d12 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_submit_f635072bb3d05faa = function(arg0, arg1) {
        getObject(arg0).submit(getObject(arg1));
    };
    imports.wbg.__wbg_text_dfc4cb7631d2eb34 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).text();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_then_0438fad860fe38e1 = function(arg0, arg1) {
        const ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_0ffafeddf0e182a4 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_toBlob_50ebe8ca763e3295 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).toBlob(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_transferFromImageBitmap_0fceaef8c8b48fde = function(arg0, arg1) {
        getObject(arg0).transferFromImageBitmap(getObject(arg1));
    };
    imports.wbg.__wbg_type_c0d5d83032e9858a = function(arg0) {
        const ret = getObject(arg0).type;
        return (__wbindgen_enum_GpuCompilationMessageType.indexOf(ret) + 1 || 4) - 1;
    };
    imports.wbg.__wbg_unmap_8c2e8131b2aaa844 = function(arg0) {
        getObject(arg0).unmap();
    };
    imports.wbg.__wbg_unobserve_811e068c4afdf7d8 = function(arg0, arg1) {
        getObject(arg0).unobserve(getObject(arg1));
    };
    imports.wbg.__wbg_usage_13caa02888040e9f = function(arg0) {
        const ret = getObject(arg0).usage;
        return ret;
    };
    imports.wbg.__wbg_userAgentData_5e600df9bb352050 = function(arg0) {
        const ret = getObject(arg0).userAgentData;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_userAgent_918b064b4cd32842 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg1).userAgent;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_valueOf_5b6f2438fbd02962 = function(arg0) {
        const ret = getObject(arg0).valueOf();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_value_47fde8ea2d9fdcd5 = function(arg0, arg1) {
        const ret = getObject(arg1).value;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_value_b7364a12af04cfd2 = function(arg0) {
        const ret = getObject(arg0).value;
        return ret;
    };
    imports.wbg.__wbg_versions_c71aa1626a93e0a1 = function(arg0) {
        const ret = getObject(arg0).versions;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_visibilityState_77ef9c00e56d5fb4 = function(arg0) {
        const ret = getObject(arg0).visibilityState;
        return (__wbindgen_enum_VisibilityState.indexOf(ret) + 1 || 3) - 1;
    };
    imports.wbg.__wbg_webkitExitFullscreen_da1f86a19cc44384 = function(arg0) {
        getObject(arg0).webkitExitFullscreen();
    };
    imports.wbg.__wbg_webkitFullscreenElement_0c4654cdfe6894e5 = function(arg0) {
        const ret = getObject(arg0).webkitFullscreenElement;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_webkitRequestFullscreen_fe95241c4f21ea63 = function(arg0) {
        getObject(arg0).webkitRequestFullscreen();
    };
    imports.wbg.__wbg_width_0d7b0b5ad3c2009f = function(arg0) {
        const ret = getObject(arg0).width;
        return ret;
    };
    imports.wbg.__wbg_writeBuffer_5ca4981365eb5ac0 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).writeBuffer(getObject(arg1), arg2, getObject(arg3), arg4, arg5);
    };
    imports.wbg.__wbg_writeText_e65e98e75a2a92b8 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).writeText(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_writeTexture_246118eb2f5a1592 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).writeTexture(getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4));
    };
    imports.wbg.__wbg_x_f1a4f46155227ad0 = function(arg0) {
        const ret = getObject(arg0).x;
        return ret;
    };
    imports.wbg.__wbg_y_fd817e2108616912 = function(arg0) {
        const ret = getObject(arg0).y;
        return ret;
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        return ret;
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper12499 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_34);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper12531 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_34);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper12533 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_34);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper12538 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_34);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper12703 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_34);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper21501 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_45);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper56129 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 6420, __wbg_adapter_45);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper56155 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_45);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper57239 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_34);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper58437 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 6420, __wbg_adapter_34);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper59045 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_56);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper59054 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_34);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper59073 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_34);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper59075 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_34);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper59115 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_34);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper59119 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_34);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper59131 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_34);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper59162 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1406, __wbg_adapter_71);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbindgen_is_null = function(arg0) {
        const ret = getObject(arg0) === null;
        return ret;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedFloat32ArrayMemory0 = null;
    cachedUint32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    cachedUint8ClampedArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('mogura-wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
