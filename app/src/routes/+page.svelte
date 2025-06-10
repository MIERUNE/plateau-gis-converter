<script lang="ts">
	import { message } from '@tauri-apps/plugin-dialog';
	import { invoke } from '@tauri-apps/api/core';
	import { attachConsole } from '@tauri-apps/plugin-log';
	import type { SinkParameters } from '$lib/sinkparams';
	import type { TransformerSettings } from '$lib/transformer';

	import Icon from '@iconify/svelte';
	import InputSelector from './InputSelector.svelte';
	import LoadingAnimation from './LoadingAnimation.svelte';
	import OutputSelector from './OutputSelector.svelte';
	import SettingSelector from './SettingSelector.svelte';

	attachConsole(); // For Tauri log in the webview console

	let inputPaths: string[] = $state([]);
	let filetype: string = $state('gpkg');
	let epsg: number = $state(4979);
	let rulesPath = $state('');
	let outputPath = $state('');
	let sinkParameters = $state({} as SinkParameters);
	let isRunning = $state(false);
	let isConvertButtonDisabled = $derived(!inputPaths.length || !outputPath || isRunning);

	let transformerSettings: TransformerSettings | undefined = $state(undefined);

	async function convertAndSave() {
		isRunning = true;

		try {
			await invoke('run_conversion', {
				inputPaths,
				outputPath,
				filetype,
				epsg,
				rulesPath,
				transformerSettings,
				sinkParameters
			});

			isRunning = false;
			await message(`変換が完了しました。\n'${outputPath}' に出力しました。`, { kind: 'info' });
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
		} catch (error: any) {
			if (error.type != 'Canceled') {
				await message(`エラーが発生しました。\n\n${error.type}: ${error.message}`, {
					title: '変換エラー',
					kind: 'error'
				});
			}
			isRunning = false;
		}
	}
</script>

{#if isRunning}
	<div class="fixed inset-0 z-20 h-screen bg-black/70 backdrop-blur-[2px]">
		<LoadingAnimation />
	</div>
{/if}

<div class="grid h-screen place-items-center py-5">
	<div class="flex max-w-2xl flex-col gap-8 pb-4">
		<div class="flex items-center gap-1.5">
			<h1 class="text-2xl font-bold">PLATEAU GIS Converter</h1>
			<a href="/about" class="hover:text-accent1">
				<Icon class="mt-0.5 text-2xl" icon="mingcute:information-line" />
			</a>
		</div>

		<InputSelector bind:inputPaths />

		<SettingSelector
			bind:filetype
			bind:epsg
			bind:rulesPath
			bind:sinkParameters
			bind:transformerSettings
		/>

		<OutputSelector {filetype} bind:outputPath />

		<div class="flex justify-end">
			<button
				onclick={convertAndSave}
				disabled={isConvertButtonDisabled}
				class="flex items-center gap-1 rounded-full bg-accent1 py-1.5 pr-5 pl-3 font-bold shadow-2xl {isConvertButtonDisabled
					? 'opacity-50'
					: ''}"
			>
				<Icon class="text-lg" icon="ic:baseline-play-arrow" />
				変換
			</button>
		</div>
	</div>
</div>
