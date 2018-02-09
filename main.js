document.addEventListener('DOMContentLoaded', function () {
    // partially taken from https://www.hellorust.com
    // (https://github.com/badboy/hellorust)
    function copyCStr(module, ptr) {
        let orig_ptr = ptr;
        const collectCString = function* () {
            let memory = new Uint8Array(module.memory.buffer);
            while (memory[ptr] !== 0) {
                if (memory[ptr] === undefined) { throw new Error("Tried to read undef mem") }
                yield memory[ptr]
                ptr += 1
            }
        }

        const buffer_as_u8 = new Uint8Array(collectCString())
        const utf8Decoder = new TextDecoder("UTF-8");
        const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
        module.dealloc_str(orig_ptr);
        return buffer_as_utf8
    }

    function getStr(module, ptr, len) {
        const getData = function* (ptr, len) {
            let memory = new Uint8Array(module.memory.buffer);
            for (let index = 0; index < len; index++) {
                if (memory[ptr] === undefined) { throw new Error(`Tried to read undef mem at ${ptr}`) }
                yield memory[ptr + index]
            }
        }

        const buffer_as_u8 = new Uint8Array(getData(ptr / 8, len / 8));
        const utf8Decoder = new TextDecoder("UTF-8");
        const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
        return buffer_as_utf8;
    }

    function newString(module, str) {
        const utf8Encoder = new TextEncoder("UTF-8");
        let string_buffer = utf8Encoder.encode(str)
        let len = string_buffer.length
        let ptr = module.alloc(len + 1)

        let memory = new Uint8Array(module.memory.buffer);
        for (i = 0; i < len; i++) {
            memory[ptr + i] = string_buffer[i]
        }

        memory[ptr + len] = 0;

        return ptr;
    }

    var context = {
        module: {},

        calc_fact: function () {
            var value = document.getElementById("input").value;
            var output = document.getElementById("output");
            var res = this.module.sum_bytes_js(value, this.module);
            output.innerText = res;
        },

        run: async function () {
            "use strict";

            const res = await fetch("./synacor_vm.wasm");
            const buffer = await res.arrayBuffer();
            const module = await WebAssembly.compile(buffer);
            const instance = await WebAssembly.instantiate(module);
            const exports = instance.exports;

            this.module.sum_bytes_js = function (str, module) {
                let buf = newString(module, str);
                let outptr = module.sum_bytes(buf);
                let result = copyCStr(module, outptr);
                return result;
            }
            this.module.sum_bytes = exports.sum_bytes;
            this.module.alloc = exports.alloc;
            this.module.dealloc_str = exports.dealloc_str;
            this.module.memory = exports.memory;

            var input = document.getElementById("input");
            input.addEventListener("change", this.calc_fact.bind(this));
            input.addEventListener("keyup", this.calc_fact.bind(this));
        }
    };

    context.run();

    window.ctx = context;
});

/*
      var input = document.getElementById("input");
      var output = document.getElementById("output");
      var number_out = document.getElementById("number-out");
      function calcFact() {
        value = input.value|0;
        number_out.innerText = "fact("+value+") = ";
        if (value < 0) {
            output.innerText = "[Value too small.]"
            return;
        }
        output.innerText = Module.fact_str(value);
      }
      calcFact()

      input.addEventListener("keyup", calcFact);*/
