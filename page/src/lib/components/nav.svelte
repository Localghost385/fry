<script>
	import {
		Navbar,
		NavBrand,
		NavLi,
		NavUl,
		NavHamburger,
		Dropdown,
		DropdownItem,
		DropdownDivider,
		DarkMode
	} from 'flowbite-svelte';
	import { ChevronDownOutline } from 'flowbite-svelte-icons';
	import { page } from '$app/stores';

	import ExpandBar from './expand_bar.svelte';

	$: activeUrl = $page.url.pathname;

	$: hide_menu = true;

	let links = [
		{
			name: 'Home',
			url: '/'
		},
		{
			name: 'Process An Image',
			url: '/process'
		}
	];
</script>

<Navbar
	class="sticky top-0 z-30 start-0 bg-light dark:bg-dark border-b-2 border-dark dark:border-light transition-colors duration-300"
>
	<div class=" flex flex-row items-center justify-between gap-[3vw]">
		<DarkMode class="self-center" />
		<NavBrand href="/">
			<span class="self-center whitespace-nowrap text-xl font-semibold text-dark dark:text-light"
				>Fry</span
			>
		</NavBrand>
	</div>
	<button
		on:click={() => {
			hide_menu = !hide_menu;
		}}
	>
		<NavHamburger onClick="" />
	</button>
	<NavUl
		hidden={hide_menu}
		{activeUrl}
		activeClass="text-dark dark:text-light"
		nonActiveClass="text-dark dark:text-light [&>.expand-bar]:scale-x-0"
	>
		{#each links as link}
			<NavLi
				on:click={() => {
					hide_menu = true;
				}}
				href={link.url}
			>
				<div>
					{link.name}
				</div>
				<div class="expand-bar">
					<ExpandBar />
				</div>
			</NavLi>
		{/each}
	</NavUl>
</Navbar>

<style lang="postcss">
	@import url('https://fonts.googleapis.com/css2?family=Urbanist:ital,wght@0,100..900;1,100..900&display=swap');
	* {
		font-family: 'Urbanist', sans-serif;
		font-optical-sizing: auto;
		font-weight: 300;
		font-style: normal;
	}
</style>
