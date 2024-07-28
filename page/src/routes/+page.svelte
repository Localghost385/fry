<script async>
	import { HeroHeader } from 'flowbite-svelte-blocks';
	import { ArrowRightOutline } from 'flowbite-svelte-icons';
	import { onMount } from 'svelte';
	import ExpandBar from '$lib/components/expand_bar.svelte';
</script>

<div
	class="h-[calc(100vh-80px)] w-full relative bg-[url('/src/lib/images/blob_1_light.svg')] dark:bg-[url('/src/lib/images/blob_1_dark.svg')] bg-cover bg-center bg-no-repeat p-10"
>
	<div class=" h-full w-full flex flex-col items-center justify-center gap-[3vw]">
		<div class=" place-self-center lg:col-span-7">
			<div class="flex flex-col">
				<div
					class="max-w-2xl text-4xl font-extrabold tracking-tight leading-none md:text-5xl xl:text-6xl dark:text-light"
				>
					Fry
				</div>
				<div class="max-w-2xl mt-4 mb-4 font-light text-dark md:text-lg lg:text-xl dark:text-light">
					An image processing tool from hell.
				</div>
				<a
					class="h-10 w-36 flex flex-row items-center justify-center rounded-[2.5rem] text-center leading-[2.4rem] border-[1px] border-dark dark:border-light bg-light dark:bg-dark text-dark dark:text-light [&:hover>.arrow]:w-6 [&:hover>.arrow]:opacity-100 [&:hover>.arrow]:mr-0 [&:hover>.button-content]:ml-2"
					href="/process"
				>
					<div class="button-content ml-4 transition-all duration-300">Get Started</div>
					<div class="arrow w-0 opacity-0 mr-5 transition-all duration-300 flex flex-row items-center justify-center">
						<ArrowRightOutline />
					</div>
				</a>
			</div>
		</div>
	</div>
</div>
