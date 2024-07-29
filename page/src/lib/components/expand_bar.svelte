<script>
	import { onMount } from 'svelte';

	let Header_underline = undefined;

	onMount(() => {
		resetClasses();
		setTimeout(updateClasses, 50);
	});

	function resetClasses() {
		Header_underline.classList.remove('scale-x-100', 'opacity-100');
		Header_underline.classList.add('scale-x-0', 'opacity-0', 'transition-duration-0');
	}

	function updateClasses() {
		Header_underline.classList.remove('scale-x-0', 'opacity-0', 'transition-duration-0');
		Header_underline.classList.add('scale-x-100', 'opacity-100');
	}
</script>

<div
	class="h-[2px] w-full bg-dark dark:bg-light transform transition-transform origin-left"
	bind:this={Header_underline}
/>

