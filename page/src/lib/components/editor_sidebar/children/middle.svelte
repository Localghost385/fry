<script>
	import {
		Sidebar,
		SidebarGroup,
		SidebarItem,
		SidebarWrapper,
		Fileupload,
		Range
	} from 'flowbite-svelte';
	import {
		ChartPieSolid,
		GridSolid,
		MailBoxSolid,
		UserSolid,
		ShoppingBagSolid,
		ArrowRightToBracketOutline,
		EditOutline,
		FireSolid,
		BookSolid,
		RestoreWindowOutline,
		LifeSaverSolid
	} from 'flowbite-svelte-icons';
	import { slide } from 'svelte/transition';

	let spanClass = 'flex-1 ms-3 whitespace-nowrap';
</script>

<div
	class="self-center flex items-center gap-2 rounded-md p-2 text-base font-normal bg-light dark:bg-dark hover:bg-light dark:hover:bg-dark text-dark dark:text-light border-[1px] dark:border-dark border-light hover:border-dark dark:hover:border-light hover:shadow-md dark:hover:shadow-xl transition-all"
>
	<div>meat</div>
	<input
		type="range"
		min="0"
		max="100"
		step="1"
		value="50"
		class="w-full bg-light dark:bg-dark"
	/>
</div>
<div
	class="self-center flex items-center gap-2 rounded-md p-2 text-base font-normal bg-light dark:bg-dark hover:bg-light dark:hover:bg-dark text-dark dark:text-light border-[1px] dark:border-dark border-light hover:border-dark dark:hover:border-light hover:shadow-md dark:hover:shadow-xl transition-all"
>
	<div>meat</div>
</div>
<div
	class="self-center flex items-center gap-2 rounded-md p-2 text-base font-normal bg-light dark:bg-dark hover:bg-light dark:hover:bg-dark text-dark dark:text-light border-[1px] dark:border-dark border-light hover:border-dark dark:hover:border-light hover:shadow-md dark:hover:shadow-xl transition-all"
>
	<div>meat</div>
</div>

<style >
	input[type='range'] {
		color: #ef233c;
		--thumb-height: 1.125em;
		--track-height: 0.125em;
		--track-color: rgba(0, 0, 0, 0.2);
		--brightness-hover: 180%;
		--brightness-down: 80%;
		--clip-edges: 0.125em;
	}
</style>
