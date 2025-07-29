<script lang="ts">
	import * as dialog from '@tauri-apps/plugin-dialog';
	import { invoke } from '@tauri-apps/api/core';
	import Icon from '@iconify/svelte';

	import PrimaryButton from './PrimaryButton.svelte';
	import { abbreviatePath } from '$lib/utils';
	import { getTypeLabel, type MeshcodeData } from './utils';
	import SecondaryButton from './SecondaryButton.svelte';

	type Props = {
		meshcodeData: MeshcodeData | null;
		selectedMeshes: string[];
		inputPaths: string[];
		clearAll: () => void;
		onClickNext: () => void;
	};

	let {
		meshcodeData = $bindable(),
		selectedMeshes = $bindable(),
		inputPaths = $bindable(),

		clearAll,
		onClickNext
	}: Props = $props();

	let loading: boolean = $state(false);
	let error: string | null = $state(null);

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
</script>

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
	<div class="flex min-h-0 flex-1 flex-col">
		{#if selectedMeshes.length > 0}
			<h3 class="mb-3 text-lg font-semibold">
				変換対象メッシュ ({selectedMeshes.length}個)
			</h3>
			<div class="h-auto min-h-0 flex-1 space-y-2 overflow-y-auto">
				{#each selectedMeshes as meshcode (meshcode)}
					<div class="rounded border border-orange-200 bg-orange-50 px-3 py-2">
						<span class="text-sm font-medium text-orange-800">{meshcode}</span>
						{#if meshcodeData && meshcodeData[meshcode]}
							<div class="mt-1 text-xs text-orange-600">
								{Object.keys(meshcodeData[meshcode])
									.map((type) => getTypeLabel(type))
									.join(', ')}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<div class="mt-3 flex justify-between gap-2">
		<SecondaryButton
			onclick={() => {
				selectedMeshes = [];
			}}
			disabled={selectedMeshes.length === 0}
		>
			選択解除
		</SecondaryButton>
		<PrimaryButton onclick={onClickNext} disabled={selectedMeshes.length === 0}>
			地物型選択へ <Icon icon="material-symbols:arrow-forward" class="inline" />
		</PrimaryButton>
	</div>
</div>
