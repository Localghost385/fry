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
	class="flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700"
>
	<input type="file" name="preview" class="hidden" on:change={onFileChange} />
	<div class=" ">{previewValue}</div>
</label>
