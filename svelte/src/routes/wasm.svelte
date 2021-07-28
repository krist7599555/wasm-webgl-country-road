
<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import { onMount } from "svelte";
	import wasm_url from 'local-rust/main_bg.wasm?url';

	let num = -1
	onMount(async () => {
		// const buf = await fetch(wasm_url).then(r => r.arrayBuffer());
		// const memory = new WebAssembly.Memory({initial: 256, maximum: 256});
		// console.log({ buf, memory})
		// const k = await WebAssembly.instantiate(buf, {
		// 	env: {
		// 		'abortStackOverflow': _ => { throw new Error('overflow'); },
		// 		'table': new WebAssembly.Table({initial: 0, maximum: 0, element: 'anyfunc'}),
		// 		'__table_base': 0,
		// 		'memory': memory,
		// 		'__memory_base': 1024,
		// 		'STACKTOP': 0,
		// 		'STACK_MAX': memory.buffer.byteLength,
		// 	}
		// })
		// console.log(k)
		const { default: init, random_num, webgl_painting, ...rest }  = await import('local-rust');
		const inito = await init(wasm_url);
		const memory = inito.memory;
		console.log(inito)
		// memory.grow(10000)
		num = random_num();
		console.log(await webgl_painting("canvas"));
		// x.default
		// window["wasm"] = x; //{ init_wasm, fetch_repo }

		// const mod = await init_wasm();
		// console.log(mod)
		// console.log(mod.fetch_repo)
	})
</script>

hellos {num}

<canvas id="canvas"></canvas>
