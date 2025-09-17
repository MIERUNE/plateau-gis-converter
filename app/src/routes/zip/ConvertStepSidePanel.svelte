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

	function getAvailableFiles() {
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
	}

	let availableFiles = $derived.by(() => {
		return getAvailableFiles();
	});

	let selectedFiles: string[] = $state(getAvailableFiles().map((f) => f.filepath));

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

		try {
			await invoke('run_conversion', {
				inputPaths: selectedFiles,
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
			<div class="jus flex flex-row items-center"></div>
			<h3 class="mb-2 text-sm font-semibold">
				選択されたファイル ({selectedFiles.length}個)
			</h3>
			<div class=" rounded border bg-gray-50">
				<div class="flex justify-between px-2 py-1 text-xs">
					<input
						type="checkbox"
						checked={availableFiles.length === selectedFiles.length}
						indeterminate={selectedFiles.length !== 0 &&
							availableFiles.length !== selectedFiles.length}
						onchange={() => {
							if (selectedFiles.length === availableFiles.length) {
								selectedFiles = [];
							} else {
								selectedFiles = availableFiles.map((f) => f.filepath);
							}
						}}
					/>
					<span class="truncate">ファイル名</span>
					<span class="text-gray-500">メッシュコード</span>
				</div>
				<div class="max-h-32 space-y-1 overflow-y-auto px-2 pb-2 text-xs">
					{#each availableFiles as file (file.filepath)}
						<label class="flex justify-between">
							<input type="checkbox" bind:group={selectedFiles} value={file.filepath} />
							<span class="truncate">{getFilename(file.filepath)}</span>
							<span class="text-gray-500">{file.meshcode}</span>
						</label>
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
