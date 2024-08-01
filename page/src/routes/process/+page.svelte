<script>
	import init, { process } from '$lib/wasm/pkg/fry_core';
	import Editor_sidebar from '$lib/components/editor_sidebar/editor_sidebar.svelte';
	import { page } from '$app/stores';

	import { getContext, onMount } from 'svelte';

	let spanClass = 'flex-1 ms-3 whitespace-nowrap';

	$: activeUrl = $page.url.pathname === '/' ? '/' : $page.url.pathname.replace(/\/$/, '');

	let img_element = undefined;

	async function execute_wasm() {
		await init();
		console.log('process in parent');
		let file = getContext('file');
		console.log("start file")
		console.log(file);
		console.log("end file")
		// if (file.length == 0) {
			// console.log('no file');
			// return;
		// }
		let result = await process(new Uint8Array(file));
		console.log(result);

		img_element.src = URL.createObjectURL(new Blob([result.buffer], { type: 'image/png' }));
	}
</script>

<div
	class="h-[calc(100vh-80px)] w-full relative bg-[url('/src/lib/images/blob_1_light.svg')] dark:bg-[url('/src/lib/images/blob_1_dark.svg')] bg-cover bg-center bg-no-repeat"
>
	<div class=" h-full w-full flex flex-col items-center justify-center gap-[3vw]">
		<div class="h-full w-full flex flex-row items-center justify-between">
			<div class="w-full flex flex-row items-center justify-center">
				<img src="" alt="loading" bind:this={img_element} />
			</div>
			{#key activeUrl}
				<Editor_sidebar on:process={execute_wasm} />
			{/key}
		</div>
	</div>
</div>

