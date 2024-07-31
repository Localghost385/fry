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

	import { createEventDispatcher, onMount } from 'svelte';
	const dispatch = createEventDispatcher();

	let spanClass = 'flex-1 ms-3 whitespace-nowrap';

	let fileuploadprops = {
		id: 'user_avatar'
	};

	let Sidebar_element = undefined;

	onMount(() => {
		resetClasses();
		setTimeout(updateClasses, 50);
	});

	function resetClasses() {
		Sidebar_element.classList.remove('translate-x-0', 'opacity-100');
		Sidebar_element.classList.add('translate-x-full', 'opacity-0', 'transition-duration-0');
	}

	function updateClasses() {
		Sidebar_element.classList.remove('translate-x-full', 'opacity-0', 'transition-duration-0');
		Sidebar_element.classList.add('translate-x-0', 'opacity-100');
	}

	function process() {
		console.log('process in child');

		dispatch('process');
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
				<SidebarItem label="Dashboard">
					<svelte:fragment slot="icon">
						<ChartPieSolid
							class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
						/>
					</svelte:fragment>
				</SidebarItem>
				<SidebarItem label="Kanban" {spanClass}>
					<svelte:fragment slot="icon">
						<GridSolid
							class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
						/>
					</svelte:fragment>
					<svelte:fragment slot="subtext">
						<span
							class="inline-flex justify-center items-center px-2 ms-3 text-sm font-medium text-gray-800 bg-gray-200 rounded-full dark:bg-gray-700 dark:text-gray-300"
						>
							Pro
						</span>
					</svelte:fragment>
				</SidebarItem>
				<SidebarItem label="Inbox" {spanClass}>
					<svelte:fragment slot="icon">
						<MailBoxSolid
							class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
						/>
					</svelte:fragment>
					<svelte:fragment slot="subtext">
						<span
							class="inline-flex justify-center items-center p-3 ms-3 w-3 h-3 text-sm font-medium text-primary-600 bg-primary-200 rounded-full dark:bg-primary-900 dark:text-primary-200"
						>
							3
						</span>
					</svelte:fragment>
				</SidebarItem>
				<SidebarItem label="Users">
					<svelte:fragment slot="icon">
						<UserSolid
							class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
						/>
					</svelte:fragment>
				</SidebarItem>
				<SidebarItem label="Products">
					<svelte:fragment slot="icon">
						<ShoppingBagSolid
							class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
						/>
					</svelte:fragment>
				</SidebarItem>
				<SidebarItem label="Sign In">
					<svelte:fragment slot="icon">
						<ArrowRightToBracketOutline
							class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
						/>
					</svelte:fragment>
				</SidebarItem>
				<SidebarItem label="Sign Up">
					<svelte:fragment slot="icon">
						<EditOutline
							class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
						/>
					</svelte:fragment>
				</SidebarItem>
			</SidebarGroup>
			<SidebarGroup
				border
				borderClass="pt-4 mt-4 border-t  border-dark dark:border-light"
				class="w-full"
			>
				<SidebarItem label="Upgrade to Pro">
					<svelte:fragment slot="icon">
						<FireSolid
							class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
						/>
					</svelte:fragment>
				</SidebarItem>
				<SidebarItem label="Documentation">
					<svelte:fragment slot="icon">
						<BookSolid
							class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
						/>
					</svelte:fragment>
				</SidebarItem>
				<SidebarItem label="Components">
					<svelte:fragment slot="icon">
						<RestoreWindowOutline
							class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
						/>
					</svelte:fragment>
				</SidebarItem>
				<div
					class="flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700"
				>
					<Fileupload {...fileuploadprops} />
				</div>
				<button
					class="flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700"
					on:click={process}
				>
					Process
				</button>
			</SidebarGroup>
		</SidebarWrapper>
	</Sidebar>
</div>
