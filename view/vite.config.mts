import { defineConfig } from "vite";
import path from "path";

import angular from "@analogjs/vite-plugin-angular";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import tsconfigPaths from "vite-tsconfig-paths";

export default defineConfig(({ mode }) => ({
    resolve: {
        alias: [
            {
                find: "wasm-client",
                replacement: path.resolve(__dirname, "../wasm_client/pkg/wasm_client.js"),
            },
        ],
    },
    plugins: [angular(), wasm(), topLevelAwait(), tsconfigPaths()],
    test: {
        globals: true,
        setupFiles: ["src/test-setup.ts"],
        environment: "jsdom",
        include: ["src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}"],
        reporters: ["default"],
        coverage: {
            provider: "v8", // or 'v8'
            enabled: true,
        },
        pool: "threads",
    },
    define: {
        "import.meta.vitest": mode !== "production",
    },
}));
