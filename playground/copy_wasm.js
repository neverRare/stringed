const fs = require("fs");
fs.copyFile(
    "../wasm_core/pkg/stringed_wasm_core_bg.wasm",
    "./dist/interpretter.wasm",
    (error) => {
        if (error) throw error;
        console.log("copied wasm file");
    },
);
