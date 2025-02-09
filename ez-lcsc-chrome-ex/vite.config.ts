import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	define: {
		// This prevents Vite from trying to process 'chrome'
		'process.env.NODE_ENV': '"production"'
	}
});
