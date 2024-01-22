<script lang="ts">
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import InputSelector from './InputSelector.svelte';
	import SettingSelector from './SettingSelector.svelte';
	import OutputSelector from './OutputSelector.svelte';

	let inputPaths: string[] = [];
	let fileType: string;
	let outputPath = '';

	async function convertAndSave() {
		if (!inputPaths) {
			alert('入力フォルダ/ファイルを選択してください');
			return;
		}
		if (!outputPath) {
			alert('出力先を選択してください');
			return;
		}

		await invoke('run', {
			inputPaths,
			outputPath,
			fileType
		});
		alert(`${fileType}形式で '${outputPath}' に出力しました。`);
	}
</script>

<div class="grid place-items-center h-screen">
	<div class="max-w-2xl flex flex-col gap-12">
		<div class="flex items-center gap-1.5">
			<h1 class="font-bold text-2xl">BRIDGE 都市デジタルツイン・GISコンバータ</h1>
			<a href="/about" class="hover:text-accent1">
				<Icon class="text-2xl mt-0.5" icon="mingcute:information-line" />
			</a>
		</div>

		<InputSelector bind:inputPaths />

		<SettingSelector bind:fileType />

		<OutputSelector bind:outputPath />

		<div class="flex justify-end">
			<button
				on:click={convertAndSave}
				class="bg-accent1 flex items-center font-bold py-1.5 pl-3 pr-5 rounded-full gap-1 shawdow-2xl hover:opacity-75"
			>
				<Icon class="text-lg" icon="ic:baseline-play-arrow" />
				変換
			</button>
		</div>
	</div>
</div>
