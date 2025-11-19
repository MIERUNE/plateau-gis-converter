<script lang="ts">
	import Icon from '@iconify/svelte';
	import SettingSelector, { type SettingSelectorProps } from '../SettingSelector.svelte';
	import OutputSelector, { type OutputSelectorProps } from '../OutputSelector.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { message } from '@tauri-apps/plugin-dialog';
	import LoadingAnimation from '../LoadingAnimation.svelte';
	import SecondaryButton from '../SecondaryButton.svelte';
	import type { FetchCityGmlMetadataResult } from '$lib/fetchCityGMLMetadata';
	import PrimaryButton from '../PrimaryButton.svelte';
	import ZipOutputSelector from '../ZipOutputSelector.svelte';

	type Props = {
		selectedMeshcodesData: FetchCityGmlMetadataResult | null;
		selectedFeatureTypes: string[];
		zipOutputPath: string;
		onclickBack: () => void;
	} & SettingSelectorProps &
		OutputSelectorProps;

	let {
		selectedMeshcodesData,
		selectedFeatureTypes,
		filetype = $bindable(),
		epsg = $bindable(4979),
		rulesPath = $bindable(),
		sinkParameters = $bindable(),
		transformerSettings = $bindable(),
		outputPath = $bindable(),
		zipOutputPath = $bindable(),
		onclickBack
	}: Props = $props();

	let isRunning = $state(false);

	function getAvailableFiles() {
		const files: {
			filepath: string;
			meshcode: string;
			type: string;
		}[] = [];

		if (!selectedMeshcodesData?.success) {
			return [];
		}
		for (const type of selectedFeatureTypes) {
			const info = selectedMeshcodesData.featureTypes[type];
			if (!info || !info.files.length) {
				continue;
			}
			for (const file of info.files) {
				files.push({
					filepath: file.url,
					meshcode: file.meshcode,
					type: type
				});
			}
		}
		return files;
	}

	let availableFiles = $derived.by(() => {
		return getAvailableFiles();
	});

	let selectedFiles: string[] = $state(getAvailableFiles().map((f) => f.filepath));

	function getFilename(fileUrl: string): string {
		const parts = fileUrl.split('/').filter(Boolean);
		return parts.at(-1) || fileUrl;
	}

	async function convertFiles() {
		if (selectedFiles.length === 0 || !outputPath) return;

		isRunning = true;
		try {
			await invoke('pack_and_run_conversion', {
				urls: selectedFiles,
				outputPath,
				zipOutputPath,
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
			console.log(error);
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
		<ZipOutputSelector bind:zipOutputPath />
	</div>
	<div class="flex justify-between gap-2 pb-4">
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
