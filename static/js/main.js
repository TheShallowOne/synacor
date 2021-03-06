document.addEventListener('DOMContentLoaded', function () {
    "use strict";

    // 0: deactivated
    // 1: normal
    // 2: verbose
    // 3: every WASM instruction
    const DEBUG_LEVEL = 2;

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

    const WASM_EXECUTE = Object.freeze({
        Error: -1,
        Ok: 0,
        Halt: 1,
        NeedInput: 2,
        NoImage: 3,
    });

    let context = {
        exports: {},
        loaded: false,
        last_timestamp: null,
        bulk_size: 10000,
        running: false,
        waiting_for_input: false,

        fetch_string: function (ptr, len) {
            "use strict";

            let buffer_u8 = new Uint8Array(this.exports.memory.buffer, ptr, len);
            const utf8_decoder = new TextDecoder("utf-8");
            return utf8_decoder.decode(buffer_u8);
        },

        run: async function () {
            "use strict";

            const imports = this.init_imports();

            const res = await fetch("./wasm/synacor_vm.wasm");
            const buffer = await res.arrayBuffer();
            const module = await WebAssembly.instantiate(buffer, imports);
            const instance = module.instance;
            this.exports = instance.exports;

            document.getElementById("vm-input").addEventListener("keypress", this.input_key_down.bind(this));
            document.getElementById("vm-input").addEventListener("paste", this.input_paste.bind(this));
            document.getElementById("vm-run").addEventListener("click", this.on_run_click.bind(this));
            document.getElementById("input-form").addEventListener("submit", this.load_form_submit.bind(this));
        },

        init_imports: function () {
            "use strict";

            let env = {
                log: function (ptr, len) {
                    "use strict";

                    let str = context.fetch_string(ptr, len);
                    console.log("WASM: ", str);
                    context.show_log_message(str, "info");
                },
                output: function (val) {
                    "use strict";

                    const chr = String.fromCharCode(val);

                    if (DEBUG_LEVEL == 2) {
                        console.log("output", val, chr);
                    }
                    document.getElementById("vm-output").textContent += chr;
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
            try {
                return Uint8Array.from(atob(str), c => c.charCodeAt(0));
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

        on_run_click: async function () {
            "use strict";

            if (!this.loaded) {
                this.show_log_message("No image loaded", "warning");
                return;
            }

            this.running = true;
            this.waiting_for_input = false;
            document.getElementById("vm-run").disabled = true;
            document.getElementById("vm-status").textContent = "Running";

            window.requestAnimationFrame(this.run_loop.bind(this));

            /*
            this.exports.test_imports();
            var a = await this.fetch_paste("bcFeqDzm");
            console.log(a);

            var res = this.decode_base64(a);
            console.log(res);
            */
        },

        run_loop: function (stamp) {
            "use strict";

            if (this.last_timestamp) {
                let diff = stamp - this.last_timestamp;
                // goal: 60 Hz => 1000 / 60 = 16.666

                if (diff > 17) {
                    let used_percentage = diff / 16.666;
                    this.bulk_size = (this.bulk_size / used_percentage) | 0;

                    if (DEBUG_LEVEL >= 1) {
                        console.log("Used time: ", diff);
                        console.log("Reducing:  ", this.bulk_size);
                    }
                }
                else {
                    this.bulk_size = (this.bulk_size * 1.2) | 0;

                    if (DEBUG_LEVEL >= 1) {
                        console.log("Increase:  ", this.bulk_size);
                    }
                }
            }
            if (this.bulk_size < 1000) {
                this.bulk_size = 1000;
            }

            this.last_timestamp = stamp;

            console.time("loop");

            let continue_execute = true;
            let error = false;

            for (let i = 0; i < this.bulk_size && continue_execute; ++i) {
                let res = this.exports.execute_step();

                continue_execute = false;

                if (res === WASM_EXECUTE.Ok) {
                    continue_execute = true;
                } else if (res === WASM_EXECUTE.NeedInput) {
                    this.show_log_message("Waiting for input!", "warning");
                    document.getElementById("vm-status").textContent = "Waiting for input";
                    this.waiting_for_input = true;
                } else {
                    document.getElementById("vm-status").textContent = "Ended";
                    error = res === WASM_EXECUTE.Error;
                    if (!error) {
                        this.show_log_message("Finished successfully", "success");
                    } else {
                        this.set_log_level("danger");
                    }
                }
            }

            // check if need to scroll after this
            const pos = document.documentElement.clientHeight + document.documentElement.scrollTop;
            const do_scroll = (pos === document.documentElement.scrollHeight);

            this.exports.do_output();

            if (do_scroll) {
                window.scrollTo(0, document.documentElement.scrollHeight);
            }

            if (continue_execute) {
                window.requestAnimationFrame(this.run_loop.bind(this));
            }

            if (DEBUG_LEVEL >= 2) {
                console.timeEnd("loop");
            }
        },

        load_image: function (buffer) {
            "use strict";

            if (!buffer) {
                this.show_log_message("Invalid data!", "danger");
                return;
            }

            const data = new Uint8Array(buffer);

            this.loaded = false;
            if (DEBUG_LEVEL >= 1) {
                console.log("Loading...", data);
            }

            // upload data
            let len = data.length;
            const ptr = this.exports.alloc(len);

            let memory = new Uint8Array(this.exports.memory.buffer);
            for (let i = 0; i < len; ++i) {
                memory[ptr + i] = data[i]
            }

            if (this.exports.load_image(ptr, len, DEBUG_LEVEL >= 3)) {
                this.show_log_message("Loaded image successful", "success");
                this.loaded = true;
                this.running = false;
            } else {
                this.set_log_level("danger");
            }

            document.getElementById("btn-show-execute").disabled = !this.loaded;
            if (this.loaded) {
                document.getElementById("vm-input").value = "";
                document.getElementById("vm-run").disabled = false;
                document.getElementById("vm-status").textContent = "Waiting";
                document.getElementById("vm-output").textContent = "";

                $('#collapseTwo').collapse('show');
            } else {
                $('#collapseTwo').collapse('hide');
            }

            this.exports.dealloc(ptr, len);
        },

        load_form_submit: async function (event) {
            "use strict";

            event.preventDefault();

            if (document.getElementById("input-file").classList.contains("active")) {
                const files = document.getElementById('upload-image').files;
                if (1 !== files.length) {
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
                // remote
            } else {
                this.show_log_message("Unknown error", "danger");
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

        add_input: function (code) {
            if (DEBUG_LEVEL >= 2) {
                console.log("Char: ", code, " (", String.fromCharCode(code), ")");
            }
            this.exports.add_input(code);
            let textfield = document.getElementById("vm-input");
            textfield.value += String.fromCharCode(code);
            textfield.scrollTo(0, textfield.scrollHeight);

            // continue if waiting
            if (this.waiting_for_input) {
                this.waiting_for_input = false;
                window.requestAnimationFrame(this.run_loop.bind(this));
            }
        },

        input_paste: function (e) {
            e.stopPropagation();
            e.preventDefault();

            let data = e.clipboardData.getData('Text');
            if (DEBUG_LEVEL >= 1) {
                console.log("Paste: ", data);
            }
            if (!data) {
                return;
            }

            for (let i = 0; i < data.length; i++) {
                const code = data.charCodeAt(i);

                if (13 === code) {
                    continue;
                }

                this.add_input(code);
            }
        },

        input_key_down: function (e) {
            if (DEBUG_LEVEL >= 2) {
                console.log("Input: ", e);
            }

            if (e.keyCode === 13) {
                this.add_input(10);
            } else {
                this.add_input(e.charCode);
            }
        }
    };

    context.run();

    window.ctx = context;
});
