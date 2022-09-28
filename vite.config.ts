import { defineConfig } from "vite";
import svgr from "vite-plugin-svgr";
import react from "@vitejs/plugin-react";

export default defineConfig({
  root: "src",
  plugins: [svgr(), react()],
  build: {
    outDir: "../dist",
    minify: "esbuild",
    emptyOutDir: true,
  },
  server: {
    port: 3000,
    strictPort: true,
  },
});
