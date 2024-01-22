<script lang="ts">
	import { dialog } from '@tauri-apps/api';
	import { fileTypeOptions } from '$lib/settings';

	export let fileType: string;
	export let outputPath: string;

	async function openOutputDialog() {
		const res = await dialog.save();
		outputPath = Array.isArray(res) ? res[0] : res;
	}
</script>

<div class="">
	<label for="filetype-select" class="mr-11">出力形式</label>
	<select bind:value={fileType} name="filetype" id="filetype-select" class="text-gray-700">
		{#each fileTypeOptions as fileType}
			<option value={fileType}>{fileType}</option>
		{/each}
	</select>
</div>

<div class="">
	<h2>出力ファイル</h2>
	<div class="bg-white px-4 py-2 rounded-xl flex gap-4">
		<button
			on:click={openOutputDialog}
			class="bg-blue-500 hover:bg-blue-700 text-white font-bold rounded-sm px-4 shadow">選択</button
		>
		<div class={outputPath ? 'text-gray-800' : 'text-gray-500'}>
			{(outputPath.length < 36 ? outputPath : `... ${outputPath.slice(-36)}`) ||
				'ファイルが選択されていません'}
		</div>
	</div>
</div>
