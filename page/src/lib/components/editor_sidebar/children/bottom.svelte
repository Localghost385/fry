<script>
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	function process() {
		dispatch('process');
	}
</script>

<div
	class="self-center flex items-center rounded-md p-2 text-base font-normal bg-light dark:bg-dark hover:bg-light dark:hover:bg-dark text-dark dark:text-light border-[1px] dark:border-dark border-light hover:border-dark dark:hover:border-light hover:shadow-md dark:hover:shadow-xl transition-all cursor-pointer"
>
	<button on:click={process}> Process </button>
</div>
