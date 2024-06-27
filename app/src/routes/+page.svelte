<script lang="ts">
	import { message } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import { attachConsole } from 'tauri-plugin-log-api';

	import Icon from '@iconify/svelte';
	import InputSelector from './InputSelector.svelte';
	import LoadingAnimation from './LoadingAnimation.svelte';
	import OutputSelector from './OutputSelector.svelte';
	import SettingSelector from './SettingSelector.svelte';

	attachConsole(); // For Tauri log in the webview console

	let inputPaths: string[] = [];
	let filetype: string;
	let epsg: number;
	let rulesPath = '';
	let outputPath = '';
	let isRunning = false;
	let transformerSwitchOption: { key: string; label: string; enabled: boolean }[];

	async function convertAndSave() {
		if (!inputPaths) {
			await message('入力フォルダ/ファイルを選択してください', { type: 'warning' });
			return;
		}

		if (!outputPath) {
			await message('出力先を選択してください', { type: 'warning' });
			return;
		}

		isRunning = true;

		const option = transformerSwitchOption.map((transformerDefinition) => {
			return {
				key: transformerDefinition.key,
				enabled: transformerDefinition.enabled
			};
		});

		try {
			await invoke('run_conversion', {
				inputPaths,
				outputPath,
				filetype,
				epsg,
				rulesPath,
				transformerOption: option
			});
			isRunning = false;
			await message(`変換が完了しました。\n'${outputPath}' に出力しました。`, { type: 'info' });
		} catch (error: any) {
			if (error.type != 'Canceled') {
				await message(`エラーが発生しました。\n\n${error.type}: ${error.message}`, {
					title: '変換エラー',
					type: 'error'
				});
			}
			isRunning = false;
		}
	}
</script>

{#if isRunning}
	<div class="absolute inset-0 bg-black/70 backdrop-blur-[2px] z-20">
		<LoadingAnimation />
	</div>
{/if}

<div class="py-5 grid place-items-center h-screen">
	<div class="max-w-2xl flex flex-col gap-12 pb-8">
		<div class="flex items-center gap-1.5">
			<h1 class="font-bold text-2xl">PLATEAU GIS Converter</h1>
			<a href="/about" class="hover:text-accent1">
				<Icon class="text-2xl mt-0.5" icon="mingcute:information-line" />
			</a>
		</div>

		<InputSelector bind:inputPaths />

		<SettingSelector bind:filetype bind:epsg bind:rulesPath bind:transformerSwitchOption={transformerSwitchOption} />

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
