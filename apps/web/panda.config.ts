import { defineConfig } from "@pandacss/dev";

export default defineConfig({
	presets: ["@pandacss/dev/presets"],
	// Whether to use css reset
	preflight: true,

	// Where to look for your css declarations
	include: ["./src/**/*.{js,jsx,ts,tsx}"],

	// Files to exclude
	exclude: [],

	// Useful for theme customization
	theme: {
		extend: {},
	},

	jsxFramework: "solid",

	syntax: "object-literal",

	strictTokens: true,

	strictPropertyValues: true,

	// The output directory for your css system
	outdir: "../../packages/styled-system",

	importMap: {
		css: "@gd/styled-system/css",
		recipes: "@gd/styled-system/recipes",
		patterns: "@gd/styled-system/patterns",
		jsx: "@gd/styled-system/jsx",
	},
});
