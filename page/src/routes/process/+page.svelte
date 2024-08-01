<script>
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { getContext, setContext } from 'svelte';
	import init, { process } from '$lib/wasm/pkg/fry_core';
	import EditorSidebar from '$lib/components/editor_sidebar/editor_sidebar.svelte';

	let file;
	let imgElement;
	let activeUrl =
		$page.url.pathname === '/' ? '/' : $page.url.pathname.replace(/\/$/, '');

	function update_file(new_file) {
		file = new_file;
	}

	async function executeWasm() {
		const result = await process(new Uint8Array(file));
		imgElement.src = URL.createObjectURL(
			new Blob([result.buffer], { type: 'image/png' })
		);
	}

	onMount(async () => {
		await init();
	});
</script>

<div
	class="h-[calc(100vh-80px)] w-full relative bg-[url('/src/lib/images/blob_1_light.svg')] dark:bg-[url('/src/lib/images/blob_1_dark.svg')] bg-cover bg-center bg-no-repeat"
>
	<div
		class="h-full w-full flex flex-col items-center justify-center gap-[3vw]"
	>
		<div class="h-full w-full flex flex-row items-center justify-between">
			<div class="w-full flex flex-row items-center justify-center">
				<img src="" alt="loading" bind:this={imgElement} />
			</div>
			{#key activeUrl}
				<EditorSidebar {update_file} on:process={executeWasm} />
			{/key}
		</div>
	</div>
</div>
