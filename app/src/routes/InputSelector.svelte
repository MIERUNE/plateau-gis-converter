<script lang="ts">
	import { dialog } from '@tauri-apps/api';

	export let inputPath = '';

	async function openInputDialog() {
		const res = await dialog.open({
			multiple: false,
			directory: false,
			filters: [
				{
					name: 'CityGML',
					extensions: ['gml']
				}
			]
		});
		inputPath = Array.isArray(res) ? res[0] : res ?? '';
	}
</script>

<div class="">
	<h2>入力</h2>
	<div class="px-4 py-2 rounded-xl flex gap-4">
		<button
			on:click={openInputDialog}
			class="bg-blue-500 hover:bg-blue-700 text-white font-bold rounded-sm px-4 shadow">選択</button
		>
		<div class={inputPath ? 'text-gray-800' : 'text-gray-500'}>
			{(inputPath.length < 36 ? inputPath : `... ${inputPath.slice(-36)}`) ||
				'ファイルが選択されていません'}
		</div>
	</div>
</div>
