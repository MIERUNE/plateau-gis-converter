<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	import InputSelector from './InputSelector.svelte';
	import OutputSelector from './OutputSelector.svelte';

	let inputPath = '';
	let fileType: string;
	let outputPath = '';

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
			fileType
		});
		alert(`${fileType}形式で、 ${outputPath} に出力しました 🚀`);
	}
</script>

<div class="grid place-items-center h-screen">
	<InputSelector bind:inputPath />

	<OutputSelector bind:fileType bind:outputPath />

	<div class="flex justify-center">
		<button
			on:click={convertAndSave}
			class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-6 rounded-2xl shadow"
		>
			変換
		</button>
	</div>
</div>
