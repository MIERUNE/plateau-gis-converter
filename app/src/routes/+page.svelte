<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { attachConsole } from 'tauri-plugin-log-api';

	import Icon from '@iconify/svelte';
	import InputSelector from './InputSelector.svelte';
	import SettingSelector from './SettingSelector.svelte';
	import OutputSelector from './OutputSelector.svelte';
	import LoadingAnimation from './LoadingAnimation.svelte';

	attachConsole(); // For Tauri log in the webview console

	let inputPaths: string[] = [];
	let filetype: string;
	let epsg: number;
	let rulesPath = '';
	let outputPath = '';
	let isRunning = false;

	async function convertAndSave() {
		if (!inputPaths) {
			alert('入力フォルダ/ファイルを選択してください');
			return;
		}
		if (!outputPath) {
			alert('出力先を選択してください');
			return;
		}

		isRunning = true;

		try {
			await invoke('run', {
				inputPaths,
				outputPath,
				filetype,
				epsg,
				rulesPath
			});
			alert(`変換が完了しました。\n'${outputPath}' に出力しました。`);
		} catch (error) {
			alert(`エラーが発生しました。\n\n${error}`);
		}

		isRunning = false;
	}
</script>

{#if isRunning}
	<div class="grid place-items-center absolute w-screen h-screen z-20 bg-black/60">
		<LoadingAnimation />
	</div>
{/if}

<div class="grid place-items-center h-screen">
	<div class="max-w-2xl flex flex-col gap-12">
		<div class="flex items-center gap-1.5">
			<h1 class="font-bold text-2xl">BRIDGE 都市デジタルツイン・GISコンバータ</h1>
			<a href="/about" class="hover:text-accent1">
				<Icon class="text-2xl mt-0.5" icon="mingcute:information-line" />
			</a>
		</div>

		<InputSelector bind:inputPaths />

		<SettingSelector bind:filetype bind:epsg bind:rulesPath />

		<OutputSelector {filetype} bind:outputPath />

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
