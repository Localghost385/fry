<script>
	import { Label, Fileupload } from 'flowbite-svelte';

	import init, { main } from '$lib/wasm/pkg/fry_core';

	import { onMount } from 'svelte';

	let fileuploadprops = {
		id: 'user_avatar'
	};

	let fileuploadprops2 = {
		id: 'user_avatar2'
	};

	let img_element = undefined;
	onMount(async () => {
		await init();
		// let result = await main();

		// img_element.src = URL.createObjectURL(new Blob([result.buffer], { type: 'image/png' }));
	});
</script>

<div
	class="h-[calc(100vh-80px)] w-full relative bg-[url('/src/lib/images/blob_1_light.svg')] dark:bg-[url('/src/lib/images/blob_1_dark.svg')] bg-cover bg-center bg-no-repeat p-10"
>
	<div class=" h-full w-full flex flex-col items-center justify-center gap-[3vw]">
		<div class="flex flex-row items-center justify-evenly">
			<img src="" alt="loading" bind:this={img_element} />
			<div class="flex flex-col">
				<Fileupload {...fileuploadprops} />
				<button
					on:click={() => {
						console.log(img_element);
					}}>Process</button
				>
			</div>
		</div>
	</div>
</div>
