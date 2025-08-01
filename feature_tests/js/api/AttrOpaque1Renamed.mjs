// generated by diplomat-tool
import { RenamedAttrEnum } from "./RenamedAttrEnum.mjs"
import { Unnamespaced } from "./Unnamespaced.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const AttrOpaque1Renamed_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.namespace_AttrOpaque1_destroy(ptr);
});

export class AttrOpaque1Renamed {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];

    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("AttrOpaque1Renamed is an Opaque type. You cannot call its constructor.");
            return;
        }
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;

        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            AttrOpaque1Renamed_box_destroy_registry.register(this, this.#ptr);
        }

        return this;
    }
    /** @internal */
    get ffiValue() {
        return this.#ptr;
    }


    #defaultConstructor() {

        const result = wasm.namespace_AttrOpaque1_new();

        try {
            return new AttrOpaque1Renamed(diplomatRuntime.internalConstructor, result, []);
        }

        finally {
        }
    }

    static macTest() {

        const result = wasm.namespace_AttrOpaque1_mac_test();

        try {
            return result;
        }

        finally {
        }
    }

    static hello() {

        const result = wasm.namespace_AttrOpaque1_hello();

        try {
            return result;
        }

        finally {
        }
    }

    get methodRenamed() {

        const result = wasm.namespace_AttrOpaque1_method(this.ffiValue);

        try {
            return result;
        }

        finally {
        }
    }

    get abirenamed() {

        const result = wasm.renamed_on_abi_only(this.ffiValue);

        try {
            return result;
        }

        finally {
        }
    }

    useUnnamespaced(un) {
    wasm.namespace_AttrOpaque1_use_unnamespaced(this.ffiValue, un.ffiValue);

        try {}

        finally {
        }
    }

    useNamespaced(n) {
    wasm.namespace_AttrOpaque1_use_namespaced(this.ffiValue, n.ffiValue);

        try {}

        finally {
        }
    }

    constructor() {
        if (arguments[0] === diplomatRuntime.exposeConstructor) {
            return this.#internalConstructor(...Array.prototype.slice.call(arguments, 1));
        } else if (arguments[0] === diplomatRuntime.internalConstructor) {
            return this.#internalConstructor(...arguments);
        } else {
            return this.#defaultConstructor(...arguments);
        }
    }
}