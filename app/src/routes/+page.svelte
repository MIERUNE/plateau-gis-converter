<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { dialog } from '@tauri-apps/api';
	import { fileTypeOptions } from '$lib/settings';

	import InputSelector from './InputSelector.svelte';

	let inputPath = '';
	let filetype = fileTypeOptions[0];
	let outputPath = '';

	async function openOutputDialog() {
		const res = await dialog.save();
		outputPath = Array.isArray(res) ? res[0] : res;
	}

	async function convertAndSave() {
		if (!inputPath) {
			alert('入力ファイルを選択してください');
			return;
		}
		if (!outputPath) {
			alert('出力先を選択してください');
			return;
		}

		await invoke('run', {
			inputPath,
			outputPath,
			filetype
		});
		alert(`${filetype}形式で、 ${outputPath} に出力しました 🚀`);
	}
</script>

<div class="grid place-items-center h-screen">
	<InputSelector bind:inputPath />

	<div class="">
		<label for="filetype-select" class="mr-11">出力形式</label>
		<select bind:value={filetype} name="filetype" id="filetype-select" class="text-gray-700">
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
				class="bg-blue-500 hover:bg-blue-700 text-white font-bold rounded-sm px-4 shadow"
				>選択</button
			>
			<div class={outputPath ? 'text-gray-800' : 'text-gray-500'}>
				{(outputPath.length < 36 ? outputPath : `... ${outputPath.slice(-36)}`) ||
					'ファイルが選択されていません'}
			</div>
		</div>
	</div>

	<div class="flex justify-center">
		<button
			on:click={convertAndSave}
			class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-6 rounded-2xl shadow"
		>
			変換
		</button>
	</div>
</div>
