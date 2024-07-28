<script>
    import { onMount } from 'svelte';

    let Header_underline = undefined;
    onMount(() => {
        Header_underline.classList.add('scale-x-100');
        Header_underline.classList.remove('scale-x-0');
    })
</script>

<div
class="h-[2px] w-full bg-dark dark:bg-light transform scale-x-0 transition-transform duration-700 ease-[cubic-bezier(0.445, 0.05, 0.55, 0.95)] origin-left"
bind:this={Header_underline}
/>