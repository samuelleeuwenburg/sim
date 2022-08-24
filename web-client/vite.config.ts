import { defineConfig } from "vite";
import checker from "vite-plugin-checker";
import wasmPack from "vite-plugin-wasm-pack";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [checker({ typescript: true }), wasmPack("./sim-web-client")],
});
