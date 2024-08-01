<script>
	import {
		Footer,
		FooterCopyright,
		FooterLinkGroup,
		FooterBrand,
		FooterLink
	} from 'flowbite-svelte';
</script>

<Footer
	footerType="logo"
	class="bg-light dark:bg-dark border-t-2 border-dark dark:border-light rounded-none transition-colors duration-300"
>
	<div class="sm:flex sm:items-center sm:justify-between">
		<FooterBrand href="/" name="Fry" />
		<FooterLinkGroup
			ulClass="flex flex-wrap items-center mb-6 text-sm text-dark dark:text-light  sm:mb-0"
		>
			<FooterLink href="/license">Licensing</FooterLink>
			<FooterLink href="/">Contact</FooterLink>
		</FooterLinkGroup>
	</div>
	<hr class="my-6 border-dark dark:border-light sm:mx-auto lg:my-8" />
	<FooterCopyright
		href="/"
		by="Grace Murphy"
		year={2024}
		aClass="text-dark dark:text-light"
		spanClass="block text-sm text-dark dark:text-light sm:text-center"
	/>
</Footer>
