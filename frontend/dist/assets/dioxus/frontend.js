import { Interpreter } from './snippets/dioxus-interpreter-js-459fb15b86d869f7/src/interpreter.js';

let wasm;

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error('expected a boolean argument');
    }
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
    if (builtInMatches.length > 1) {
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

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder('utf-8');

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

    if (typeof(arg) !== 'string') throw new Error('expected a string argument');

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedInt32Memory0 = new Int32Array();

function getInt32Memory0() {
    if (cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

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
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_18(arg0, arg1, arg2) {
    try {
        _assertNum(arg0);
        _assertNum(arg1);
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h92458c9eedaa9450(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_21(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc48b526495cd2bf6(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_24(arg0, arg1, arg2) {
    try {
        _assertNum(arg0);
        _assertNum(arg1);
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1230d4be0cfe8280(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function __wbg_adapter_27(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8e9681bdb38972c7(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_30(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he9f8b310f7941f56(arg0, arg1);
}

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getObject(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }

const u32CvtShim = new Uint32Array(2);

const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);

function isLikeNone(x) {
    return x === undefined || x === null;
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

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

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_error_09919627ac0992f5 = function() { return logError(function (arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
    if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1); }
    console.error(v0);
}, arguments) };
imports.wbg.__wbg_new_693216e109162396 = function() { return logError(function () {
    const ret = new Error();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_stack_0ddaca5d1abfb52f = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbindgen_is_undefined = function(arg0) {
    const ret = getObject(arg0) === undefined;
    _assertBoolean(ret);
    return ret;
};
imports.wbg.__wbg_setTimeout_131fc254e1bd5624 = function() { return handleError(function (arg0, arg1) {
    const ret = setTimeout(getObject(arg0), arg1);
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_clearTimeout_65417660fe82f08d = typeof clearTimeout == 'function' ? clearTimeout : notDefined('clearTimeout');
imports.wbg.__wbg_new_b28a2eedeb0ae791 = function() { return logError(function (arg0) {
    const ret = new Interpreter(takeObject(arg0));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_SetNode_3cacb3b69eaf984a = function() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).SetNode(arg1 >>> 0, takeObject(arg2));
}, arguments) };
imports.wbg.__wbg_PushRoot_2fc45e345a75c0f4 = function() { return logError(function (arg0, arg1, arg2) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    getObject(arg0).PushRoot(n0);
}, arguments) };
imports.wbg.__wbg_PopRoot_37f502792df5013a = function() { return logError(function (arg0) {
    getObject(arg0).PopRoot();
}, arguments) };
imports.wbg.__wbg_AppendChildren_cb58331e674a9890 = function() { return logError(function (arg0, arg1) {
    getObject(arg0).AppendChildren(arg1 >>> 0);
}, arguments) };
imports.wbg.__wbg_ReplaceWith_047004f1504dd9fd = function() { return logError(function (arg0, arg1, arg2, arg3) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    getObject(arg0).ReplaceWith(n0, arg3 >>> 0);
}, arguments) };
imports.wbg.__wbg_InsertAfter_2b51bcb76b875653 = function() { return logError(function (arg0, arg1, arg2, arg3) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    getObject(arg0).InsertAfter(n0, arg3 >>> 0);
}, arguments) };
imports.wbg.__wbg_InsertBefore_35f9eb4e97336da6 = function() { return logError(function (arg0, arg1, arg2, arg3) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    getObject(arg0).InsertBefore(n0, arg3 >>> 0);
}, arguments) };
imports.wbg.__wbg_Remove_4e775fa1976ca6ee = function() { return logError(function (arg0, arg1, arg2) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    getObject(arg0).Remove(n0);
}, arguments) };
imports.wbg.__wbg_CreateTextNode_274d8b18710171d1 = function() { return logError(function (arg0, arg1, arg2, arg3) {
    u32CvtShim[0] = arg2;
    u32CvtShim[1] = arg3;
    const n0 = uint64CvtShim[0];
    getObject(arg0).CreateTextNode(takeObject(arg1), n0);
}, arguments) };
imports.wbg.__wbg_CreateElement_e83d7c8e84e44d57 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    u32CvtShim[0] = arg3;
    u32CvtShim[1] = arg4;
    const n1 = uint64CvtShim[0];
    getObject(arg0).CreateElement(v0, n1);
}, arguments) };
imports.wbg.__wbg_CreateElementNs_14ee497c33e3c2eb = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    u32CvtShim[0] = arg3;
    u32CvtShim[1] = arg4;
    const n1 = uint64CvtShim[0];
    var v2 = getCachedStringFromWasm0(arg5, arg6);
    getObject(arg0).CreateElementNs(v0, n1, v2);
}, arguments) };
imports.wbg.__wbg_CreatePlaceholder_124b20053ff032fa = function() { return logError(function (arg0, arg1, arg2) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    getObject(arg0).CreatePlaceholder(n0);
}, arguments) };
imports.wbg.__wbg_NewEventListener_bbe4a49a18dde684 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    u32CvtShim[0] = arg3;
    u32CvtShim[1] = arg4;
    const n1 = uint64CvtShim[0];
    getObject(arg0).NewEventListener(v0, n1, getObject(arg5));
}, arguments) };
imports.wbg.__wbg_RemoveEventListener_92b2d0d66f2a2018 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).RemoveEventListener(n0, v1);
}, arguments) };
imports.wbg.__wbg_SetText_f42fd3eb23f65f34 = function() { return logError(function (arg0, arg1, arg2, arg3) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    getObject(arg0).SetText(n0, takeObject(arg3));
}, arguments) };
imports.wbg.__wbg_SetAttribute_862ba64fad05bd68 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    var v2 = getCachedStringFromWasm0(arg6, arg7);
    getObject(arg0).SetAttribute(n0, v1, takeObject(arg5), v2);
}, arguments) };
imports.wbg.__wbg_RemoveAttribute_6d85cea304fd611f = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    var v2 = getCachedStringFromWasm0(arg5, arg6);
    getObject(arg0).RemoveAttribute(n0, v1, v2);
}, arguments) };
imports.wbg.__wbg_instanceof_Window_42f092928baaee84 = function() { return logError(function (arg0) {
    const ret = getObject(arg0) instanceof Window;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_document_15b2e504fb1556d6 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_location_312161fbd0cf64f0 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).location;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_history_cb2cdfbe20fef7ad = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).history;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_requestAnimationFrame_9e5ccef32fec2b99 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_requestIdleCallback_eb8356640f18b686 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).requestIdleCallback(getObject(arg1));
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_title_bd6187635987b962 = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).title;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_createElement_28fc3740fb11defb = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createElement(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_getElementById_927eae2597d26692 = function() { return logError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).getElementById(v0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_pointerId_ba430da80d384ca3 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).pointerId;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_width_5538d517078eb23c = function() { return logError(function (arg0) {
    const ret = getObject(arg0).width;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_height_75744ff243f79011 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).height;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_pressure_aed4071137e434ac = function() { return logError(function (arg0) {
    const ret = getObject(arg0).pressure;
    return ret;
}, arguments) };
imports.wbg.__wbg_tangentialPressure_27bd1573c80bd82f = function() { return logError(function (arg0) {
    const ret = getObject(arg0).tangentialPressure;
    return ret;
}, arguments) };
imports.wbg.__wbg_tiltX_86d46f1caa79bd6f = function() { return logError(function (arg0) {
    const ret = getObject(arg0).tiltX;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_tiltY_8abb41f13bbab526 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).tiltY;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_twist_f05392f1874568ae = function() { return logError(function (arg0) {
    const ret = getObject(arg0).twist;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_pointerType_b0dc396d27477579 = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).pointerType;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_isPrimary_fc806fc5fa82996f = function() { return logError(function (arg0) {
    const ret = getObject(arg0).isPrimary;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_instanceof_HtmlInputElement_3fad42774bc62388 = function() { return logError(function (arg0) {
    const ret = getObject(arg0) instanceof HTMLInputElement;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_checked_5b4bb12a26c07690 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).checked;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_type_4e50db5683405dfc = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).type;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_value_30770021ca38e0db = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).value;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_type_eb72b0796a80d18e = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).type;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_target_68a5c10e2732a79e = function() { return logError(function (arg0) {
    const ret = getObject(arg0).target;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_preventDefault_b4d36c8196fbe491 = function() { return logError(function (arg0) {
    getObject(arg0).preventDefault();
}, arguments) };
imports.wbg.__wbg_addEventListener_ec92ea1297eefdfc = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).addEventListener(v0, getObject(arg3), getObject(arg4));
}, arguments) };
imports.wbg.__wbg_removeEventListener_bfe676215a590711 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).removeEventListener(v0, getObject(arg3), arg4 !== 0);
}, arguments) };
imports.wbg.__wbg_get_51805aeacaac6e69 = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_altKey_2a9cfd28ef911eab = function() { return logError(function (arg0) {
    const ret = getObject(arg0).altKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_metaKey_ccfaf60bbd6dad26 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).metaKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_ctrlKey_70fdfeaa6a8c0f93 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).ctrlKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_shiftKey_9d0f3836fa2d483a = function() { return logError(function (arg0) {
    const ret = getObject(arg0).shiftKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_instanceof_HtmlSelectElement_1ac0fbdb5feae38a = function() { return logError(function (arg0) {
    const ret = getObject(arg0) instanceof HTMLSelectElement;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_value_35a0dee68831cc3a = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).value;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_charCode_53a2a4a0e201e2a8 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).charCode;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_keyCode_3075e448a1211124 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).keyCode;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_altKey_f68144e23e506e43 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).altKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_ctrlKey_c5358f866c4f1ea6 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).ctrlKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_shiftKey_f0821ad63ff4a752 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).shiftKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_metaKey_6130fc4c73b4548c = function() { return logError(function (arg0) {
    const ret = getObject(arg0).metaKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_location_e166796096737eaf = function() { return logError(function (arg0) {
    const ret = getObject(arg0).location;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_repeat_900481cbdbdc38d2 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).repeat;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_key_4c91be0431f26101 = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).key;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_instanceof_Comment_146723c9747192a9 = function() { return logError(function (arg0) {
    const ret = getObject(arg0) instanceof Comment;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_instanceof_Node_9ba6b05d6dcc42dd = function() { return logError(function (arg0) {
    const ret = getObject(arg0) instanceof Node;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_parentElement_14138ef2ff0b9c88 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).parentElement;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_childNodes_2a84f5b530c45789 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).childNodes;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_nodeValue_26a09c0dff4ce37c = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).nodeValue;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_textContent_a414705450c24b11 = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).textContent;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_settextContent_8db6b500abb6e3ae = function() { return logError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).textContent = v0;
}, arguments) };
imports.wbg.__wbg_instanceof_Text_57fb9948979f4b65 = function() { return logError(function (arg0) {
    const ret = getObject(arg0) instanceof Text;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_pageX_1893d1c3a0cdd854 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).pageX;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_pageY_de42924c1322d8f0 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).pageY;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_which_e58aa55255878454 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).which;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_pushState_65b5fb0f30ca9d61 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    var v1 = getCachedStringFromWasm0(arg4, arg5);
    getObject(arg0).pushState(getObject(arg1), v0, v1);
}, arguments) };
imports.wbg.__wbg_replaceState_be211595e488c786 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    var v1 = getCachedStringFromWasm0(arg4, arg5);
    getObject(arg0).replaceState(getObject(arg1), v0, v1);
}, arguments) };
imports.wbg.__wbg_length_7399238b14adcf66 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).length;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_item_4175e9d985466d3c = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg0).item(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_instanceof_IdleDeadline_45c36398e0e7d924 = function() { return logError(function (arg0) {
    const ret = getObject(arg0) instanceof IdleDeadline;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_timeRemaining_2fd7b5d6b3f1c2e0 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).timeRemaining();
    return ret;
}, arguments) };
imports.wbg.__wbg_href_87729771ef419286 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg1).href;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_instanceof_Element_1714e50f9bda1d15 = function() { return logError(function (arg0) {
    const ret = getObject(arg0) instanceof Element;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_tagName_0b399645f1a6af61 = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).tagName;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_getAttribute_4c623ae097fca4b4 = function() { return logError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    const ret = getObject(arg1).getAttribute(v0);
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
}, arguments) };
imports.wbg.__wbg_setAttribute_8cfc462c0dedd03b = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).setAttribute(v0, v1);
}, arguments) };
imports.wbg.__wbg_instanceof_HtmlElement_057bfd3477e9b9b6 = function() { return logError(function (arg0) {
    const ret = getObject(arg0) instanceof HTMLElement;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_debug_1dccd22b8a8988e1 = function() { return logError(function (arg0, arg1, arg2, arg3) {
    console.debug(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
}, arguments) };
imports.wbg.__wbg_error_800b8d466653f7ea = function() { return logError(function (arg0) {
    console.error(getObject(arg0));
}, arguments) };
imports.wbg.__wbg_error_d539c0f5eafe6a31 = function() { return logError(function (arg0, arg1, arg2, arg3) {
    console.error(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
}, arguments) };
imports.wbg.__wbg_info_17d18b9f8eaab7d9 = function() { return logError(function (arg0, arg1, arg2, arg3) {
    console.info(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
}, arguments) };
imports.wbg.__wbg_log_f286f3fe4aad906d = function() { return logError(function (arg0, arg1, arg2, arg3) {
    console.log(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
}, arguments) };
imports.wbg.__wbg_warn_3d6689f77cb29c86 = function() { return logError(function (arg0, arg1, arg2, arg3) {
    console.warn(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
}, arguments) };
imports.wbg.__wbg_data_b5d6ae35216847ce = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).data;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_animationName_37936e97dad1780b = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).animationName;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_elapsedTime_5abaa066cc958db1 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).elapsedTime;
    return ret;
}, arguments) };
imports.wbg.__wbg_pseudoElement_16a5b434756e975f = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).pseudoElement;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_instanceof_CompositionEvent_8fede9fc0ba0e7bd = function() { return logError(function (arg0) {
    const ret = getObject(arg0) instanceof CompositionEvent;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_data_603bf0b6091a0dd6 = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).data;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_screenX_34e0510327eeef36 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).screenX;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_screenY_5a94a404b17d031b = function() { return logError(function (arg0) {
    const ret = getObject(arg0).screenY;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_clientX_e24ae62c30359a79 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).clientX;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_clientY_623ec4c13b3d821e = function() { return logError(function (arg0) {
    const ret = getObject(arg0).clientY;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_ctrlKey_9e2e6f4b1ec9595c = function() { return logError(function (arg0) {
    const ret = getObject(arg0).ctrlKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_shiftKey_ca83eebd8c6225ef = function() { return logError(function (arg0) {
    const ret = getObject(arg0).shiftKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_altKey_b6c328da58c2491b = function() { return logError(function (arg0) {
    const ret = getObject(arg0).altKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_metaKey_4f66dd3f3b7a2673 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).metaKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_button_80d1dce690815d29 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).button;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_buttons_38f26b6db9950f64 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).buttons;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_propertyName_63c0cc8930c47796 = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).propertyName;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_elapsedTime_bd9c855587ec5de3 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).elapsedTime;
    return ret;
}, arguments) };
imports.wbg.__wbg_pseudoElement_f61449ef780a66bd = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).pseudoElement;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_deltaX_c6424f322b7fc622 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).deltaX;
    return ret;
}, arguments) };
imports.wbg.__wbg_deltaY_439cc75be784d15e = function() { return logError(function (arg0) {
    const ret = getObject(arg0).deltaY;
    return ret;
}, arguments) };
imports.wbg.__wbg_deltaZ_adbb0a220642a35a = function() { return logError(function (arg0) {
    const ret = getObject(arg0).deltaZ;
    return ret;
}, arguments) };
imports.wbg.__wbg_deltaMode_9a9992d84b151599 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).deltaMode;
    _assertNum(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_instanceof_HtmlFormElement_cf35f564c31c7e40 = function() { return logError(function (arg0) {
    const ret = getObject(arg0) instanceof HTMLFormElement;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_elements_58da4627ac7ab794 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).elements;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_instanceof_HtmlTextAreaElement_f8666dc47678e5c0 = function() { return logError(function (arg0) {
    const ret = getObject(arg0) instanceof HTMLTextAreaElement;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_value_eb32f706ae6bfab2 = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg1).value;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_newnoargs_971e9a5abe185139 = function() { return logError(function (arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Function(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_call_33d7bcddbbfa394a = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_instanceof_Object_9657a9e91b05959b = function() { return logError(function (arg0) {
    const ret = getObject(arg0) instanceof Object;
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_hasOwnProperty_037be09b3a6b983b = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg0).hasOwnProperty(getObject(arg1));
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbg_new_e6a9fecc2bf26696 = function() { return logError(function () {
    const ret = new Object();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_resolve_0107b3a501450ba0 = function() { return logError(function (arg0) {
    const ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_then_18da6e5453572fc8 = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_globalThis_3348936ac49df00a = function() { return handleError(function () {
    const ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_self_fd00a1ef86d1b2ed = function() { return handleError(function () {
    const ret = self.self;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_window_6f6e346d8bbd61d7 = function() { return handleError(function () {
    const ret = window.window;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_global_67175caf56f55ca9 = function() { return handleError(function () {
    const ret = global.global;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_set_2762e698c2f5b7e0 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
    _assertBoolean(ret);
    return ret;
}, arguments) };
imports.wbg.__wbindgen_is_function = function(arg0) {
    const ret = typeof(getObject(arg0)) === 'function';
    _assertBoolean(ret);
    return ret;
};
imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};
imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};
imports.wbg.__wbindgen_closure_wrapper820 = function() { return logError(function (arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 131, __wbg_adapter_18);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbindgen_closure_wrapper4240 = function() { return logError(function (arg0, arg1, arg2) {
    const ret = makeClosure(arg0, arg1, 180, __wbg_adapter_21);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbindgen_closure_wrapper4242 = function() { return logError(function (arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 182, __wbg_adapter_24);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbindgen_closure_wrapper4797 = function() { return logError(function (arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 292, __wbg_adapter_27);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbindgen_closure_wrapper5648 = function() { return logError(function (arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 324, __wbg_adapter_30);
    return addHeapObject(ret);
}, arguments) };

return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = new Int32Array();
    cachedUint8Memory0 = new Uint8Array();

    wasm.__wbindgen_start();
    return wasm;
}

function initSync(bytes) {
    const imports = getImports();

    initMemory(imports);

    const module = new WebAssembly.Module(bytes);
    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {

    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
