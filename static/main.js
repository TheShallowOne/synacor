document.addEventListener('DOMContentLoaded', function () {
    "use strict";

    var context = {
        exports: {},

        // calc_fact: function () {
        //     var value = document.getElementById("input").value;
        //     var output = document.getElementById("output");
        //     var res = this.module.sum_bytes_js(value, this.module);
        //     output.innerText = res;

        //     var a = 234|0;
        //     var b = 432|0;

        //     var c = this.module.add(a, b);
        //     console.log(c);
        // },

        fetch_string: function (ptr, len) {
            "use strict";

            var buffer_as_u8 = new Uint8Array(this.exports.memory.buffer, ptr, len);
            const utf8Decoder = new TextDecoder("UTF-8");
            const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
            return buffer_as_utf8
        },

        run: async function () {
            "use strict";

            const imports = this.init_imports();

            const res = await fetch("./synacor_vm.wasm");
            const buffer = await res.arrayBuffer();
            const module = await WebAssembly.instantiate(buffer, imports);
            const instance = module.instance;
            const exports = instance.exports;

            this.exports = instance.exports;

            var input = document.getElementById("input");
            input.addEventListener("click", this.on_click.bind(this));
        },

        init_imports: function () {
            "use strict";

            var env = {
                log: function (ptr, len) {
                    "use strict";

                    var str = context.fetch_string(ptr, len);
                    console.log("log: ", str);
                },
                output: function (val) {
                    "use strict";

                    console.log("output: ", val);
                }
            };
            return { env: env };
        },

        on_click: function () {
            "use strict";

            this.exports.test_imports();
        }
    };

    context.run();

    window.ctx = context;





    // // partially taken from https://www.hellorust.com
    // // (https://github.com/badboy/hellorust)
    // function copyCStr(module, ptr) {
    //     let orig_ptr = ptr;
    //     const collectCString = function* () {
    //         let memory = new Uint8Array(module.memory.buffer);
    //         while (memory[ptr] !== 0) {
    //             if (memory[ptr] === undefined) { throw new Error("Tried to read undef mem") }
    //             yield memory[ptr]
    //             ptr += 1
    //         }
    //     }

    //     const buffer_as_u8 = new Uint8Array(collectCString())
    //     const utf8Decoder = new TextDecoder("UTF-8");
    //     const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
    //     module.dealloc_str(orig_ptr);
    //     return buffer_as_utf8
    // }

    // function getStr(module, ptr, len) {
    //     const getData = function* (ptr, len) {
    //         let memory = new Uint8Array(module.memory.buffer);
    //         for (let index = 0; index < len; index++) {
    //             if (memory[ptr] === undefined) { throw new Error(`Tried to read undef mem at ${ptr}`) }
    //             yield memory[ptr + index]
    //         }
    //     }

    //     const buffer_as_u8 = new Uint8Array(getData(ptr / 8, len / 8));
    //     const utf8Decoder = new TextDecoder("UTF-8");
    //     const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
    //     return buffer_as_utf8;
    // }

    // function newString(module, str) {
    //     const utf8Encoder = new TextEncoder("UTF-8");
    //     let string_buffer = utf8Encoder.encode(str)
    //     let len = string_buffer.length
    //     let ptr = module.alloc(len + 1)

    //     let memory = new Uint8Array(module.memory.buffer);
    //     for (i = 0; i < len; i++) {
    //         memory[ptr + i] = string_buffer[i]
    //     }

    //     memory[ptr + len] = 0;

    //     return ptr;
    // }
});
