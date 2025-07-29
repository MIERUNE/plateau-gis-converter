<script lang="ts">
	import Icon from '@iconify/svelte';
	import PrimaryButton from './PrimaryButton.svelte';
	import { getTypeLabel, type MeshcodeData } from './utils';
	import SecondaryButton from './SecondaryButton.svelte';

	type Props = {
		selectedFeatureTypes: string[];
		selectedMeshes: string[];
		meshcodeData: MeshcodeData;
		onclickBack: () => void;
		onclickNext: () => void;
	};

	let {
		selectedFeatureTypes = $bindable(),
		selectedMeshes,
		meshcodeData,
		onclickBack,
		onclickNext
	}: Props = $props();

	function getAvailableFeatureTypes() {
		if (selectedMeshes.length === 0) return [];

		return [
			...new Set(
				selectedMeshes.flatMap((meshcode) => {
					if (meshcode in meshcodeData) {
						return Object.keys(meshcodeData[meshcode]);
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

<div class="flex min-h-0 flex-1 flex-col gap-4">
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

		<div class="mt-6 flex justify-between gap-2">
			<SecondaryButton onclick={onclickBack}>戻る</SecondaryButton>
			<PrimaryButton onclick={onclickNext} disabled={selectedFeatureTypes.length === 0}>
				変換設定へ
				<Icon icon="material-symbols:arrow-forward" class="inline" />
			</PrimaryButton>
		</div>
	</div>
</div>
