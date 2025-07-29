<script lang="ts">
	import Icon from '@iconify/svelte';
	import SettingSelector, { type SettingSelectorProps } from '../SettingSelector.svelte';
	import OutputSelector, { type OutputSelectorProps } from '../OutputSelector.svelte';
	import SecondaryButton from './SecondaryButton.svelte';
	import PrimaryButton from './PrimaryButton.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { message } from '@tauri-apps/plugin-dialog';
	import LoadingAnimation from '../LoadingAnimation.svelte';
	import type { MeshcodeData } from './utils';

	type Props = {
		meshcodeData: MeshcodeData;
		selectedFeatureTypes: string[];
		selectedMeshes: string[];
		inputPaths: string[];
		onclickBack: () => void;
	} & SettingSelectorProps &
		OutputSelectorProps;

	let {
		meshcodeData,
		selectedFeatureTypes,
		selectedMeshes,
		inputPaths,
		filetype = $bindable(),
		epsg = $bindable(4979),
		rulesPath = $bindable(),
		sinkParameters = $bindable(),
		transformerSettings = $bindable(),
		outputPath = $bindable(),
		onclickBack
	}: Props = $props();

	let isRunning = $state(false);

	let selectedFiles = $derived.by(() => {
		const files: {
			filepath: string;
			meshcode: string;
			type: string;
		}[] = [];
		selectedMeshes.forEach((meshcode) => {
			if (meshcodeData && meshcodeData[meshcode]) {
				selectedFeatureTypes.forEach((type) => {
					if (meshcodeData && meshcodeData[meshcode][type]) {
						meshcodeData[meshcode][type].forEach((filepath) => {
							files.push({ filepath, meshcode, type });
						});
					}
				});
			}
		});
		return files;
	});

	// Extract filename from path, removing common directory structure
	function getFilename(filepath: string): string {
		// Handle zip[index]/path/to/file.gml format
		const zipMatch = filepath.match(/^zip\[(\d+)\]\/(.*)/);
		if (zipMatch) {
			const zipIndex = parseInt(zipMatch[1]);
			const innerPath = zipMatch[2];
			const parts = innerPath.split('/');
			const filename = parts[parts.length - 1] || innerPath;

			// Add ZIP file name if multiple files are selected
			if (inputPaths.length > 1 && zipIndex < inputPaths.length) {
				const zipName = inputPaths[zipIndex].split('/').pop() || `ZIP${zipIndex + 1}`;
				return `[${zipName}] ${filename}`;
			}
			return filename;
		}

		const parts = filepath.split('/');
		return parts[parts.length - 1] || filepath;
	}

	async function convertFiles() {
		if (selectedFiles.length === 0 || !outputPath) return;

		isRunning = true;
		const inputPaths = selectedFiles.map((f) => f.filepath);

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

<div class="flex min-h-0 flex-1 flex-col gap-4">
	<div class="mb-2 flex items-center gap-1.5 border-b pb-1">
		<Icon class="text-xl" icon="material-symbols:convert" />
		<h2 class="text-xl font-bold">変換</h2>
	</div>
	<div class="flex flex-1 flex-col gap-4">
		<div>
			<h3 class="mb-2 text-sm font-semibold">
				選択されたファイル ({selectedFiles.length}個)
			</h3>
			<div class="max-h-32 overflow-y-auto rounded border bg-gray-50 p-2">
				<div class="space-y-1 text-xs">
					{#each selectedFiles as file (file.filepath)}
						<div class="flex justify-between">
							<span class="truncate">{getFilename(file.filepath)}</span>
							<span class="text-gray-500">{file.meshcode}</span>
						</div>
					{/each}
				</div>
			</div>
		</div>

		<SettingSelector
			bind:filetype
			bind:epsg
			bind:rulesPath
			bind:sinkParameters
			bind:transformerSettings
		/>

		<OutputSelector {filetype} bind:outputPath />
	</div>
	<div class="flex justify-between gap-2">
		<SecondaryButton onclick={onclickBack}>戻る</SecondaryButton>
		<PrimaryButton
			onclick={convertFiles}
			disabled={!outputPath || selectedFiles.length === 0 || isRunning}
		>
			<Icon class="inline text-lg" icon="ic:baseline-play-arrow" />
			変換
		</PrimaryButton>
	</div>
</div>
