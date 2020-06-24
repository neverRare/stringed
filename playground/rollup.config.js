import typescript from "@rollup/plugin-typescript";
import resolve from "@rollup/plugin-node-resolve";
import { terser } from "rollup-plugin-terser";
export default {
    input: "src/main.ts",
    output: {
        file: "dist/main.js",
        format: "iife",
        name: "main",
    },
    plugins: [
        typescript(),
        resolve(),
        terser({
            output: false,
        }),
    ],
};
