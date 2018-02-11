document.addEventListener('DOMContentLoaded', function () {
    "use strict";

    function read_file(file) {
        "use strict";
        return new Promise((resolve, reject) => {
            let fr = new FileReader();
            fr.onload = () => {
                resolve(fr.result)
            };
            fr.readAsArrayBuffer(file);
        });
    }

    let context = {
        exports: {},
        loaded: false,

        fetch_string: function (ptr, len) {
            "use strict";

            let buffer_u8 = new Uint8Array(this.exports.memory.buffer, ptr, len);
            const utf8_decoder = new TextDecoder("utf-8");
            const buffer_utf8 = utf8_decoder.decode(buffer_u8);
            return buffer_utf8;
        },

        run: async function () {
            "use strict";

            const imports = this.init_imports();

            const res = await fetch("./wasm/synacor_vm.wasm");
            const buffer = await res.arrayBuffer();
            const module = await WebAssembly.instantiate(buffer, imports);
            const instance = module.instance;
            this.exports = instance.exports;

            var input = document.getElementById("run");
            input.addEventListener("click", this.on_click.bind(this));

            document.getElementById("input-form").addEventListener("submit", this.load_form_submit.bind(this));
        },

        init_imports: function () {
            "use strict";

            let env = {
                log: function (ptr, len) {
                    "use strict";

                    var str = context.fetch_string(ptr, len);
                    console.log("WASM: ", str);
                    context.show_log_message(str, "info");
                },
                output: function (val) {
                    "use strict";

                    console.log("output: ", val);
                }
            };
            return {env: env};
        },

        fetch_paste: async function (id) {
            "use strict";

            let url = "https://pastebin.com/raw/" + id;

            try {
                let res = await fetch(url);
                return await res.arrayBuffer();
            } catch (e) {
                console.log(e);
                return null;
            }
        },

        decode_base64_text: function (str) {
            "use strict";

            let enc = new TextEncoder();
            try {
                const data = atob(str);
                return enc.encode(data);
            } catch (e) {
                console.log(e);
                return null;
            }
        },

        decode_base64: function (arr) {
            "use strict";

            let dec = new TextDecoder();
            try {
                const s = dec.decode(arr);
                return this.decode_base64_text(s);
            } catch (e) {
                console.log(e);
                return arr;
            }
        },

        on_click: async function () {
            "use strict";

            if (!this.loaded) {
                this.show_log_message("No image loaded", "warning");
                return;
            }

            window.requestAnimationFrame(this.run_loop.bind(this));

            /*
            this.exports.test_imports();
            var a = await this.fetch_paste("bcFeqDzm");
            console.log(a);

            var res = this.decode_base64(a);
            console.log(res);
            */
        },

        run_loop: function () {
            console.time("loop");

            let cont_loop = true;
            const bulk_size = 10000;

            for (let i = 0; i < bulk_size && cont_loop; ++i) {
                cont_loop = this.exports.execute_step();
            }

            if (cont_loop) {
                window.requestAnimationFrame(this.run_loop.bind(this));

            } else {
                console.log("Ending");
            }
            console.timeEnd("loop");
        },

        load_image: function (buffer) {
            "use strict";

            if (!buffer) {
                this.show_log_message("Invalid data!", "danger");
                return;
            }

            const data = new Uint8Array(buffer);

            this.loaded = false;
            console.log("Loading...", data);

            // upload data
            let len = data.length;
            const ptr = this.exports.alloc(len);

            let memory = new Uint8Array(this.exports.memory.buffer);
            for (let i = 0; i < len; ++i) {
                memory[ptr+i] = data[i]
            }

            if (this.exports.load_image(ptr, len)) {
                this.show_log_message("Loaded image successful", "success");
                this.loaded = true;
            } else {
                this.set_log_level("danger");
            }

            this.exports.dealloc(ptr, len);
        },

        load_form_submit: async function (event) {
            "use strict";

            event.preventDefault();

            if (document.getElementById("input-file").classList.contains("active")) {
                const files = document.getElementById('upload-image').files;
                if (1 != files.length) {
                    this.show_log_message("You need to select a single file!", "danger");
                    return;
                }
                const file = files[0];
                const data = await read_file(file);

                this.load_image(data);
            } else if (document.getElementById("input-paste").classList.contains("active")) {
                // read textarea
                let text = await document.getElementById("paste-image").value;
                let data = this.decode_base64_text(text);

                this.load_image(data);
            } else if (document.getElementById("input-load").classList.contains("active")) {
                this.show_log_message("Not implemented", "danger");
                return;
                // remote
            } else {
                this.show_log_message("Unknown error", "danger");
                return;
            }
        },

        show_log_message: function (message, level) {
            "use strict";

            let el = document.getElementById("alert");
            el.innerText = message;

            this.set_log_level(level);
            el.classList.remove("invisible");
        },

        set_log_level: function (level) {
            "use strict";

            let el = document.getElementById("alert");
            el.classList.remove("alert-success");
            el.classList.remove("alert-info");
            el.classList.remove("alert-warning");
            el.classList.remove("alert-danger");
            el.classList.add("alert-" + level);
        },

        hide_log_message: function () {
            "use strict";

            document.getElementById("alert").classList.add("invisible");
        },

        //newString(module, str)
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
