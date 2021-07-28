<script lang="ts">
	import { onMount } from "svelte";
	import vs from '$lib/webgl/vertex_shader.glsl?raw'
	import fs from '$lib/webgl/fragment_shader.glsl?raw'
	import * as twgl from 'twgl.js';
	import { Matrix4, Vector3, toRadians } from '@math.gl/core'
	import { tweened } from 'svelte/motion'
	import * as easing from 'svelte/easing';
	import wretch from 'wretch'

	let canvas: HTMLCanvasElement;
	let innerWidth: number;
	let innerHeight: number;

	let player_x = tweened(0, { duration: 250, easing: easing.quartOut });
	let player_y = 0;
	let player_y_velocity = 0;
	const GRAVITY = 0.006;
	const JUMP_VELOCITY = 0.2;

	onMount(async () => {
		const gl = canvas.getContext('webgl');
		const program_info = twgl.createProgramInfo(gl, [vs, fs]);
		gl.clearColor(0.8, 0.9, 0.7, 1);
		// https://twgljs.org/docs/module-twgl.html#.createTexture
		const moto_texture = twgl.createTexture(gl, { 
			src: await createImageBitmap(await wretch("/assets/moto-back-square.png").get().blob()),
		})
		const tree_texture = twgl.createTexture(gl, { 
			src: "/assets/tree-1.png",
		})
		// new ImageBitmap()
		
		const ground_texture = twgl.createTexture(gl, { 
			wrap: gl.REPEAT,
			width: 1,
			height: 1,
			// minMag: gl.NEAREST,
			src: new Uint8Array([
				160,   145,  110, 255,
				//   0, 255,   0, 255,
				//   0,   0, 255, 255,
				// 255,   0,   0,   0,
			])
		});
		const scott_texture = twgl.createTexture(gl, { 
			// src: "/assets/road.jpg",
			// src: new Uint8Array(road),
			src: await createImageBitmap(await wretch("/assets/road2.jpg").get().blob()),
			wrap: gl.REPEAT,
			// wrap: gl.CLAMP_TO_EDGE,
			width: 2,
			height: 2,
			// minMag: gl.NEAREST,
			// src: new Uint8Array([
			// 	255,   0,   0, 255,
			// 	  0, 255,   0, 255,
			// 	  0,   0, 255, 255,
			// 	255,   0,   0,   0,
			// ])
		});
		
		// gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.REPEAT);
		// gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.REPEAT);
		// gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
		// gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);

		// prettier-ignore
		function buffer_info_repeat_rect(repeat_x = 1, repeat_y = 1) {
			return twgl.createBufferInfoFromArrays(gl, {
				position: [-1, -1, 0, 1, -1, 0, -1, 1, 0, -1, 1, 0, 1, -1, 0, 1, 1, 0],
				texcoord: [
					0 * repeat_x, 0 * repeat_y, 
					1 * repeat_x, 0 * repeat_y, 
					0 * repeat_x, 1 * repeat_y, 
					0 * repeat_x, 1 * repeat_y, 
					1 * repeat_x, 0 * repeat_y, 
					1 * repeat_x, 1 * repeat_y
				]
			});
		}

		const obstructs = Array.from({ length: 50 }).map(_ => {
			const ROAD_SIZE = 3.5;
			const random_z = () => Math.random() * -1000;
			const position = new Vector3(
				(Math.random() > 0.5 ? -1 : 1) * (ROAD_SIZE + Math.random() * 2), 
				2,
			random_z());
			return {
				position,
				scale: 3,
				texture: tree_texture,
				reset() {
					position.z = random_z() - 600
				}
			}
		})

		function render(time: number) {
			if (!canvas) return;
			requestAnimationFrame(render);
			twgl.resizeCanvasToDisplaySize(canvas);
    	gl.viewport(0, 0, canvas.width, canvas.height);
			if (canvas.width != innerWidth || canvas.height != innerHeight) {
				canvas.width = innerWidth
				canvas.height = innerHeight
			}
			gl.useProgram(program_info.program);
			gl.enable(gl.BLEND); // allow alpha
			// gl.enable(gl.DEPTH_TEST); //! NOT WORK ON ALPHA
			gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
			gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

			// setting camera
			twgl.setUniforms(program_info, {
				projection: new Matrix4().perspective({ aspect: canvas.width / canvas.height, fovy: toRadians(60), near: 0.8, far: 520 }),
				view: new Matrix4().lookAt([0, 0.5 + player_y * 1.3, 5 + player_y * 0.5], [$player_x / 5, 0.7 + player_y, 0], [0, 1, 0]),
			});

			// ground
			const groun_info = buffer_info_repeat_rect(100, 100);
			twgl.setUniforms(program_info, {
				model: new Matrix4().identity()
					.rotateY(toRadians(90))
					.rotateX(toRadians(90))
					.scale(500)
					// .scale([repeat_time, 1, 1])
					// .scale([repeat_time, 1, 1])
				,
				u_texture: ground_texture
			});
			twgl.drawBufferInfo(gl, groun_info);

			// road
			const repeat_time = 1000;
			const road_info = buffer_info_repeat_rect(repeat_time, 1);
			twgl.setBuffersAndAttributes(gl, program_info, road_info);
			twgl.setUniforms(program_info, {
				model: new Matrix4().identity()
					.translate([0, 0, time / 100 % (repeat_time / 2)])
					.scale([1, 1, repeat_time])
					.rotateY(toRadians(90))
					.rotateX(toRadians(90))
					// .scale([repeat_time, 1, 1])
					// .scale([repeat_time, 1, 1])
				,
				u_texture: scott_texture
			});
			twgl.drawBufferInfo(gl, road_info);


			// obstructs
			const basic_rect_info = buffer_info_repeat_rect(1, 1);
			twgl.setBuffersAndAttributes(gl, program_info, basic_rect_info);
			obstructs.sort((lhs, rhs) => lhs.position.z - rhs.position.z)
			for (const o of obstructs) {
				o.position.z += 0.8
				if (o.position.z > 0) {
					o.reset()
				}
				twgl.setUniforms(program_info, {
					model: new Matrix4().identity().translate(o.position).scale(o.scale),
					u_texture: o.texture
				});
				twgl.drawBufferInfo(gl, basic_rect_info);
			}


			// bike
			const moto_info = buffer_info_repeat_rect(1, 1);
			player_y += player_y_velocity;
			if (player_y > 0) {
				player_y_velocity -= GRAVITY;
			}
			if (player_y < 0) {
				player_y = 0;
				player_y_velocity = 0;
			}
			console.log(player_y, player_y_velocity)
			twgl.setBuffersAndAttributes(gl, program_info, moto_info);
			twgl.setUniforms(program_info, {
				model: new Matrix4().identity().translate([$player_x, player_y, 1.5]),
				u_texture: moto_texture
			});
			twgl.drawBufferInfo(gl, moto_info);

			
		}
		requestAnimationFrame(render);
	})

	function handle_key(e: KeyboardEvent) {
		console.log(e.code, e.key)
		switch (e.code) {
			case 'ArrowLeft':
			case 'KeyA':
				player_x.set(-1);
				break;
				case 'ArrowRight':
					case 'KeyD':
						player_x.set(1);
						break;
				case 'ArrowUp':
				case 'KeyW':
				case 'Space':
					if (player_y == 0) {
						player_y_velocity = JUMP_VELOCITY;
						break;
					}

		}
	}
</script>
<svelte:window bind:innerWidth bind:innerHeight on:keydown={handle_key} />
<canvas style="position: fixed" bind:this={canvas}></canvas>
<div style="position: fixed; top: 1rem; left: 1rem; display: flex">
	<div class="keyboard-key">A</div>
	<div class="keyboard-key">D</div>
	<div class="keyboard-key">Space</div>
</div>
<div style="position: fixed; top: 1rem; right: 1rem; display: flex; padding: 5px; font-size: 1.2rem; font-weight: bold; color: white;">
	JOURNEY FUN DRIVE
</div>
<style>
	:global(body) {
		overflow: hidden;
	}
	.keyboard-key {
		border: white solid 2px;
		color: white;
		padding: 5px 14px;
		margin-right: 10px;
	}
</style>
