<script lang="ts">
	import Icon from '@iconify/svelte';
	import SecondaryButton from '../SecondaryButton.svelte';
	import PrimaryButton from '../PrimaryButton.svelte';
	import type { FetchCityGmlMetadataResult } from '$lib/fetchCityGMLMetadata';

	interface Props {
		selectedFeatureTypes: string[];
		selectedMeshes: string[];
		selectedMeshcodesData: FetchCityGmlMetadataResult | null;
		isSelectedMeshcodesDataLoading: boolean;
		clearAll: () => void;
		onclickNext: () => void;
	}

	let {
		selectedFeatureTypes = $bindable(),
		selectedMeshes,
		selectedMeshcodesData,
		isSelectedMeshcodesDataLoading,
		clearAll,
		onclickNext
	}: Props = $props();

	function selectAllFeatureTypes() {
		if (selectedMeshcodesData?.success) {
			selectedFeatureTypes = Object.keys(selectedMeshcodesData.featureTypes);
		}
	}
</script>

<div class="flex min-h-0 flex-1 flex-col gap-4">
	{#if selectedMeshes.length === 0}
		<div class="flex min-h-0 flex-1 flex-col items-center justify-center gap-0.5">
			<Icon icon="material-symbols:map" class="mb-2 text-6xl opacity-50" />
			<p class="text-sm text-gray-600">メッシュが選択されていません</p>
			<p class="text-sm text-gray-600">
				地図上でメッシュを選択するとメッシュに含まれる地物型が表示されます
			</p>
		</div>
	{:else}
		<div class="mb-2 flex items-center gap-1.5 border-b pb-1">
			<Icon class="text-xl" icon="material-symbols:category" />
			<h2 class="text-xl font-bold">地物型選択</h2>
		</div>
		<div class="flex min-h-0 flex-1 flex-col">
			{#if !isSelectedMeshcodesDataLoading && selectedMeshcodesData && selectedMeshcodesData.success}
				{#if Object.keys(selectedMeshcodesData.featureTypes).length === 0}
					<div class="flex flex-1 flex-col items-center justify-center gap-0.5">
						<p class="text-sm text-gray-600">
							選択されたメッシュに対応する地物型が見つかりません。
						</p>
					</div>
				{:else}
					<div class="mb-4 flex items-center justify-between">
						<p class="text-sm text-gray-600">
							選択されたメッシュ({selectedMeshes.length}個)から変換したい地物型を選択してください
						</p>
						{#if selectedFeatureTypes.length === Object.keys(selectedMeshcodesData.featureTypes).length}
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
						{#each Object.entries(selectedMeshcodesData.featureTypes).toSorted( ([a], [b]) => (a > b ? 1 : -1) ) as [type, info] (type)}
							<label
								class="flex cursor-pointer items-center gap-2 rounded bg-gray-100 px-2 py-1 hover:bg-gray-200"
							>
								<input
									type="checkbox"
									bind:group={selectedFeatureTypes}
									value={type}
									class="rounded"
								/>
								<Icon icon="material-symbols:category" class="text-gray-600" />
								<span class="flex-1">
									<span class="font-medium">{info.label}</span>
									<span class="ml-2 text-xs text-gray-500">({type})</span>
								</span>

								<span class="text-xs text-gray-500">
									{info.fileCount.toLocaleString()}ファイル
								</span>
							</label>
						{/each}
					</div>
				{/if}
			{:else}
				<div class="flex flex-1 flex-col items-center justify-center gap-4">
					<Icon class="animate-spin text-4xl text-gray-500" icon="mdi:loading" />
					<p class="text-sm text-gray-600">選択されたメッシュの地物型情報を取得中...</p>
				</div>
			{/if}

			<div class="flex justify-between gap-2 py-6">
				<SecondaryButton onclick={clearAll}>はじめから</SecondaryButton>
				<PrimaryButton onclick={onclickNext} disabled={selectedFeatureTypes.length === 0}>
					変換設定へ
					<Icon icon="material-symbols:arrow-forward" class="inline" />
				</PrimaryButton>
			</div>
		</div>
	{/if}
</div>
