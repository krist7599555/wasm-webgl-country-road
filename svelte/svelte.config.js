import preprocess from 'svelte-preprocess';
import { wasm } from '@rollup/plugin-wasm';
import adapt from '@sveltejs/adapter-static';
import { dirname, resolve } from 'path';
import { fileURLToPath } from 'url';
import { spawn } from 'child_process';
const __dirname = dirname(fileURLToPath(import.meta.url));

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),
	kit: {
		adapter: adapt(),
		// hydrate the <div id="svelte"> element in src/app.html
		target: '#svelte',
		vite: () => {
			return {
				plugins: [
					// wasm({
					// 	sync: ["local-rust/local_rust_bg.wasm"]
					// }),
					({
						name: 'watch-my-wasm-rust-fullreload',
						apply: 'serve',
						config: () => ({ server: { watch: { disableGlobbing: false } } }),
						// handleHotUpdate(ctx) {
						// 	console.log(ctx)
						// },
						configureServer({ watcher, ws }) {
							watcher.add(['../rust/src/**/*.rs'])
							watcher.on('change', (path) => {
								if (path.includes('rust/src') && path.endsWith('.rs')) {
									const ps = spawn(`
										make --directory="../rust" dev-replace;
										yes | cp -rf ../rust/dist/* ./node_modules/local-rust
									`, { shell: true, stdio: "inherit" });
									ps.on('close', () => {
										ws.send({ type: 'full-reload' })
										// ws.send({ type: 'update', updates: [
										// 	{ type: 'js-update', timestamp: new Date().getTime(), acceptedPath }
										// ] })
										
									})
								}
							});
						}
					}),
				],
				optimizeDeps: {
					exclude: ['local-rust']
				},
				resolve: {
					alias: {
					}
				}
			}
		}
	}
};

export default config;
