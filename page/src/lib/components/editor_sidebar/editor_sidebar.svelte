<script>
	import { Sidebar, SidebarGroup, SidebarItem, SidebarWrapper, Fileupload } from 'flowbite-svelte';
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
	import { createEventDispatcher, onMount, setContext } from 'svelte';
	import Bottom from './children/bottom.svelte';
	import Middle from './children/middle.svelte';
	import Top from './children/top.svelte';

	const dispatch = createEventDispatcher();
	function process() {
		dispatch('process');
	}

	export let update_file;

	let Sidebar_element = undefined;

	onMount(() => {
		resetClasses();
		setTimeout(updateClasses, 1);
	});

	function resetClasses() {
		Sidebar_element.classList.remove('translate-x-0', 'opacity-100');
		Sidebar_element.classList.add('translate-x-full', 'opacity-0', 'transition-duration-0');
	}

	function updateClasses() {
		Sidebar_element.classList.remove('translate-x-full', 'opacity-0', 'transition-duration-0');
		Sidebar_element.classList.add('translate-x-0', 'opacity-100');
	}
</script>

<div
	class=" h-full opacity-0 transform transition-transform translate-x-full duration-500 ease-[cubic-bezier( 0.455, 0.03, 0.515, 0.955 )]"
	bind:this={Sidebar_element}
>
	<Sidebar class="h-full">
		<SidebarWrapper
			class="h-full flex flex-col items-start justify-center border-l-[1px] bg-light dark:bg-dark rounded-none border-dark dark:border-light "
		>
			<SidebarGroup class="w-full ">
				<Top {update_file} />
			</SidebarGroup>
			<SidebarGroup
				border
				borderClass="pt-4 mt-4 border-t  border-dark dark:border-light"
				class="w-full"
			>
				<Middle />
			</SidebarGroup>
			<SidebarGroup
				border
				borderClass="pt-4 mt-4 border-t  border-dark dark:border-light"
				class="w-full"
			>
				<Bottom on:process={process} />
			</SidebarGroup>
		</SidebarWrapper>
	</Sidebar>
</div>
