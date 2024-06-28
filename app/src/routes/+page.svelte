<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { attachConsole } from 'tauri-plugin-log-api';
	import { message } from '@tauri-apps/api/dialog';
	import type { ParamsOption } from '$lib/parameters';

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
	let paramsOption = {} as ParamsOption;
	let isRunning = false;
	let isValidationError = false;
	let isConvertButtonDisabled = true;

	$: isConvertButtonDisabled = !inputPaths.length || !outputPath || isRunning || isValidationError;

	async function convertAndSave() {
		isRunning = true;

		try {
			await invoke('run_conversion', {
				inputPaths,
				outputPath,
				filetype,
				epsg,
				rulesPath,
				paramsOption
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
	<div class="fixed inset-0 bg-black/70 backdrop-blur-[2px] z-20 h-screen">
		<LoadingAnimation />
	</div>
{/if}

<div class="py-5 grid place-items-center h-screen">
	<div class="max-w-2xl flex flex-col gap-12 pb-4">
		<div class="flex items-center gap-1.5">
			<h1 class="font-bold text-2xl">PLATEAU GIS Converter</h1>
			<a href="/about" class="hover:text-accent1">
				<Icon class="text-2xl mt-0.5" icon="mingcute:information-line" />
			</a>
		</div>

		<InputSelector bind:inputPaths />

		<SettingSelector
			bind:filetype
			bind:epsg
			bind:rulesPath
			bind:paramsOption
			bind:isValidationError
		/>

		<OutputSelector {filetype} bind:outputPath />

		<div class="flex justify-end">
			<button
				on:click={convertAndSave}
				disabled={isConvertButtonDisabled}
				class="bg-accent1 flex items-center font-bold py-1.5 pl-3 pr-5 rounded-full gap-1 shawdow-2xl {isConvertButtonDisabled
					? 'opacity-50'
					: ''}"
			>
				<Icon class="text-lg" icon="ic:baseline-play-arrow" />
				変換
			</button>
		</div>
	</div>
</div>
