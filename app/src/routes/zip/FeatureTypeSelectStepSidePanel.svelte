<script lang="ts">
 	import * as dialog from '@tauri-apps/plugin-dialog';
	import Icon from '@iconify/svelte';
	import PrimaryButton from './PrimaryButton.svelte';
	import { getTypeLabel, type MeshcodeData } from './utils';
	import SecondaryButton from './SecondaryButton.svelte';
	import { onMount } from 'svelte';
	import { getCurrentWebview } from '@tauri-apps/api/webview';
	import { invoke } from '@tauri-apps/api/core';
	import { abbreviatePath } from '$lib/utils';

	type Props = {
		selectedFeatureTypes: string[];
		selectedMeshes: string[];
		inputPaths: string[];
		meshcodeData: MeshcodeData | null;
		clearAll: () => void;
		onclickNext: () => void;
	};

	let {
		selectedFeatureTypes = $bindable(),
		selectedMeshes = $bindable(),
		inputPaths = $bindable(),
		meshcodeData =$bindable(),
		clearAll,
		onclickNext
	}: Props = $props();


	let loading: boolean = $state(false);
	let error: string | null = $state(null);

	let isDnDEnter = $state(false);
	let timerId = $state<number | null>(null);

	onMount(() => {
		const unlisten = getCurrentWebview().onDragDropEvent(async (event) => {
			if (event.payload.type === 'enter') {
				isDnDEnter = true;
			}
			if (event.payload.type === 'leave') {
				isDnDEnter = false;
			}
			if (event.payload.type === 'over') {
				isDnDEnter = true;
				if (timerId) {
					clearTimeout(timerId);
				}
				timerId = setTimeout(() => (isDnDEnter = false), 100);
			}
			if (event.payload.type === 'drop') {
				isDnDEnter = false;
				inputPaths = event.payload.paths.filter((path) => path.endsWith('.zip'));
				if (inputPaths.length === 0) {
					console.warn('No valid ZIP files dropped');
					return;
				}
				await getMeshcodeWithPrefix(inputPaths);
			}
		});

		return () => {
			unlisten.then((f) => f());
		};
	});

	async function openFileDialog() {
		const res = await dialog.open({
			multiple: true,
			directory: false,
			filters: [
				{
					name: 'PLATEAU ZIP Files',
					extensions: ['zip']
				}
			]
		});
		if (!res) return;

		// resは複数選択の場合は配列、単一選択の場合は文字列
		inputPaths = Array.isArray(res) ? res : [res];

		await getMeshcodeWithPrefix(inputPaths);
	}

	async function getMeshcodeWithPrefix(inputPaths: string[]) {
		if (inputPaths.length === 0) return;
		loading = true;
		error = null;

		try {
			meshcodeData = await invoke<MeshcodeData>('get_meshcodes_with_prefix', {
				inputPaths: inputPaths
			});
		} catch (e) {
			error = e as string;
			meshcodeData = null;
		} finally {
			loading = false;
		}
	}

	function removeFile(path: string) {
		inputPaths = inputPaths.filter((p) => p !== path);
		if (inputPaths.length === 0) {
			clearAll();
		} else {
			getMeshcodeWithPrefix(inputPaths);
		}
	}

	function getAvailableFeatureTypes() {
		if (selectedMeshes.length === 0 || !meshcodeData) return [];

		return [
			...new Set(
				selectedMeshes.flatMap((meshcode) => {
					if (meshcode in meshcodeData!) {
						return Object.keys(meshcodeData![meshcode]);
					}
					return [];
				})
			)
		].toSorted();
	}

	function selectAllFeatureTypes() {
		const availableTypes = getAvailableFeatureTypes();
		selectedFeatureTypes = availableTypes;
	}

	function getFileCountForType(type: string) {
		if (!meshcodeData || selectedMeshes.length === 0) return 0;

		let totalFiles = 0;
		selectedMeshes.forEach((meshcode) => {
			if (meshcodeData && meshcodeData[meshcode] && meshcodeData[meshcode][type]) {
				totalFiles += meshcodeData[meshcode][type].length;
			}
		});

		return totalFiles;
	}
</script>

{#if isDnDEnter}
	<div
		class="fixed inset-0 z-30 flex h-screen items-center justify-center bg-black/70 backdrop-blur-[2px]"
	>
		<div class="rounded-lg border-2 border-accent1 bg-gray-50 p-8 text-center shadow-lg">
			<Icon icon="material-symbols:upload-file" class="mx-auto mb-4 text-6xl text-accent1" />
			<p class="text-xl font-semibold text-base">PLATEAU の ZIP ファイルをドロップしてください</p>
			<p class="mt-2 text-sm text-gray-600">複数ファイルにも対応しています</p>
		</div>
	</div>
{/if}


<div class="flex min-h-0 flex-1 flex-col gap-4">
 	<div class="flex flex-col gap-4">
		<div class="flex items-center gap-1.5 border-b-1 pb-1">
			<Icon class="text-xl" icon="material-symbols:input-rounded" />
			<h2 class="text-xl font-bold">ZIPファイル選択とメッシュ選択</h2>
		</div>

		<div class="flex items-center gap-3">
			<PrimaryButton onclick={openFileDialog} disabled={loading}>
				{loading ? '読み込み中...' : 'ファイル選択'}
			</PrimaryButton>
		</div>

		<div class="text-sm">
			{#if inputPaths.length === 0}
				<p class="text-gray-500">PLATEAUのZIPファイルを選択してください（複数選択可）</p>
			{:else}
				<div class="space-y-1">
					{#each inputPaths as path (path)}
						<div class="flex items-center gap-1 rounded bg-gray-100 px-2 py-1">
							<Icon icon="material-symbols:archive" class="text-gray-600" />
							<p class="flex-1 truncate font-medium" title={path}>
								{abbreviatePath(path, 60)}
							</p>
							<button onclick={() => removeFile(path)} class="text-gray-600 hover:opacity-75">
								<Icon icon="material-symbols:cancel" />
							</button>
						</div>
					{/each}
					{#if inputPaths.length > 1}
						<p class="mt-1 text-xs text-gray-600">{inputPaths.length} ファイル選択中</p>
					{/if}
				</div>
			{/if}
		</div>

		{#if error}
			<div class="rounded border border-red-200 bg-red-50 p-3">
				<p class="text-sm text-red-700">エラー: {error}</p>
			</div>
		{/if}
		{#if meshcodeData}
			<p class="text-sm text-gray-600">
				Shift+ドラッグで複数メッシュを選択、Shift+クリックで個別選択
			</p>
		{/if}
	</div>
	{#if meshcodeData}
	<div class="mb-2 flex items-center gap-1.5 border-b pb-1">
		<Icon class="text-xl" icon="material-symbols:category" />
		<h2 class="text-xl font-bold">地物型選択</h2>
	</div>

	<div class="flex min-h-0 flex-1 flex-col">
		<div class="mb-4 flex items-center justify-between">
			<p class="text-sm text-gray-600">
				選択されたメッシュ({selectedMeshes.length}個)から変換したい地物型を選択してください
			</p>
			{#if selectedFeatureTypes.length === getAvailableFeatureTypes().length}
				<SecondaryButton
					onclick={() => {
						selectedFeatureTypes = [];
					}}
				>
					全解除
				</SecondaryButton>
			{:else}
				<PrimaryButton onclick={selectAllFeatureTypes}>全選択</PrimaryButton>
			{/if}
		</div>

		<div class="flex flex-1 flex-col gap-0.5">
			{#each getAvailableFeatureTypes() as type (type)}
				<label
					class="flex cursor-pointer items-center gap-2 rounded bg-gray-100 px-2 py-1 hover:bg-gray-200"
				>
					<input type="checkbox" bind:group={selectedFeatureTypes} value={type} class="rounded" />
					<Icon icon="material-symbols:category" class="text-gray-600" />
					<span class="flex-1">
						<span class="font-medium">{getTypeLabel(type)}</span>
						<span class="ml-2 text-xs text-gray-500">({type})</span>
					</span>
					<span class="text-xs text-gray-500">
						{getFileCountForType(type)}ファイル
					</span>
				</label>
			{/each}
		</div>

		<div class="py-6 flex justify-between gap-2">
			<SecondaryButton onclick={clearAll}>はじめから</SecondaryButton>
			<PrimaryButton onclick={onclickNext} disabled={selectedFeatureTypes.length === 0}>
				変換設定へ
				<Icon icon="material-symbols:arrow-forward" class="inline" />
			</PrimaryButton>
		</div>
	</div>
	{/if}
</div>
