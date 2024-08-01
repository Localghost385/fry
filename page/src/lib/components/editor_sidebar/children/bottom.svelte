<script>
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	function process() {
		dispatch('process');
	}
</script>

<div
	class="flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700"
>
	<button on:click={process}> Process </button>
</div>
