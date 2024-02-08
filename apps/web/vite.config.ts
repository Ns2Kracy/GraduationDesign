import { defineConfig } from "@solidjs/start/config";

export default defineConfig({
	start: {
		ssr: false,
		server: {
			prerender: {
				crawlLinks: true,
			},
		},
	},
});
