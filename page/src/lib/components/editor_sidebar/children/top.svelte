<script>
	import { setContext } from 'svelte';

	let previewValue = 'Upload Image';
	let imgBuffer;

	export let update_file;

	function onFileChange(event) {
		const file = event.target.files[0];
		previewValue = file.name;

		reader(file, (err, result) => {
			if (err) {
				console.error(err);
				return;
			}
			imgBuffer = result;
			update_file(imgBuffer);
		});
	}

	function reader(file, callback) {
		const reader = new FileReader();
		reader.onload = () => callback(null, reader.result);
		reader.onerror = (err) => callback(err);
		reader.readAsArrayBuffer(file);
	}
</script>

<label
	class="self-center flex items-center rounded-md p-2 text-base font-normal bg-light dark:bg-dark hover:bg-light dark:hover:bg-dark text-dark dark:text-light border-[1px] dark:border-dark border-light hover:border-dark dark:hover:border-light hover:shadow-md dark:hover:shadow-xl transition-all cursor-pointer"
>
	<input type="file" name="preview" class="hidden" on:change={onFileChange} />
	<div class=" ">{previewValue}</div>
</label>
