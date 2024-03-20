import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import path from "path";

export default defineConfig({
	plugins: [solid()],
	build: {
		outDir: "../server/dist",
	},
	resolve: {
		alias: {
			"styled-system": path.resolve(__dirname, "styled-system"),
		},
	},
});
