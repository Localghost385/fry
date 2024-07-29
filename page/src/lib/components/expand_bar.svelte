<script>
	import { onMount, afterUpdate } from 'svelte';
    import { page } from '$app/stores';

	export let isExpanded = false;

	let Header_underline = undefined;

	let url = undefined;

	onMount(() => {
		updateClasses();
	});

	afterUpdate(() => {
		if (url !== $page.url.pathname) {
			url = $page.url.pathname;
			updateClasses();
		}
	});

	function updateClasses() {
		if (!isExpanded) {
			console.log("1");
			Header_underline.classList.remove('scale-x-0', 'opacity-0');
			Header_underline.classList.add('scale-x-100', 'opacity-100');
		} else {
			console.log("2");
			Header_underline.classList.remove('scale-x-100', 'opacity-100');
			Header_underline.classList.add('scale-x-0', 'opacity-0');
		}
	}
</script>

<div
	class="h-[2px] w-full bg-dark dark:bg-light transform scale-x-0 transition-transform duration-700 ease-[cubic-bezier(0.445, 0.05, 0.55, 0.95)] origin-left opacity-0"
	bind:this={Header_underline}
/>
