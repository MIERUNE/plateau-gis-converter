<script lang="ts">
	import Icon from '@iconify/svelte';
	import PrimaryButton from './PrimaryButton.svelte';
	import SecondaryButton from './SecondaryButton.svelte';
	import {
		getTypeLabel,
		mergeRemoteMeshcodeData,
		type FeatureTypeLookup,
		type MeshcodeData,
		type RemoteFileInfo,
		type RemoteMeshcodeData
	} from './utils';
	import { invoke } from '@tauri-apps/api/core';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';

	type PackProgressEvent = {
		stage: string;
		status: string;
		progress: number;
	};

	type CitySummary = {
		cityCode?: string | null;
		cityName?: string | null;
		year?: number | null;
		registrationYear?: number | null;
	};

	type FetchCityGmlMetadataResult = {
		cityCode?: string | null;
		cityName?: string | null;
		year?: number | null;
		registrationYear?: number | null;
		featureTypes: FeatureTypeLookup;
		meshes: RemoteMeshcodeData;
		cities?: CitySummary[];
	};

	type DownloadCityGmlPackResult = {
		packId: string;
		zipPath: string;
	};

	type Props = {
		selectedFeatureTypes: string[];
		selectedMeshes: string[];
		inputPaths: string[];
		remoteMeshcodeData: RemoteMeshcodeData;
		featureTypeLookup: FeatureTypeLookup;
		meshcodeData: MeshcodeData | null;
		clearAll: () => void;
		onclickNext: () => void;
	};

	let {
		selectedFeatureTypes = $bindable(),
		selectedMeshes = $bindable(),
		inputPaths = $bindable(),
		remoteMeshcodeData = $bindable(),
		featureTypeLookup = $bindable(),
		meshcodeData = $bindable(),
		clearAll,
		onclickNext
	}: Props = $props();

	let loadingMeshcodes: string[] = $state([]);
	let loadedMeshcodes: string[] = $state([]);
	let fetchError: string | null = $state(null);
	let lastFetchSummary: CitySummary[] = $state([]);

	let isDownloading = $state(false);
	let downloadError: string | null = $state(null);
	let packProgress: PackProgressEvent | null = $state(null);

	let progressUnlisten: UnlistenFn | null = null;
	let citySummaryMap: Map<string, CitySummary> = new Map();

	const isFetching = $derived.by(() => loadingMeshcodes.length > 0);

	onMount(async () => {
		progressUnlisten = await listen<PackProgressEvent>('plateau://pack-progress', (event) => {
			packProgress = event.payload;
		});
	});

	onDestroy(() => {
		if (progressUnlisten) {
			progressUnlisten();
			progressUnlisten = null;
		}
	});

	function resetAll() {
		fetchError = null;
		downloadError = null;
		lastFetchSummary = [];
		packProgress = null;
		loadingMeshcodes = [];
		loadedMeshcodes = [];
		citySummaryMap = new Map();
		clearAll();
	}

	function getAllMeshesCount(): number {
		return Object.keys(remoteMeshcodeData).length;
	}

	function getEntriesForMesh(meshcode: string) {
		if (remoteMeshcodeData[meshcode]) {
			return [[meshcode, remoteMeshcodeData[meshcode]] as const];
		}
		if (meshcode.length === 6) {
			return Object.entries(remoteMeshcodeData).filter(([code]) => code.startsWith(meshcode));
		}
		return [];
	}

	function getAvailableFeatureTypes(): string[] {
		if (selectedMeshes.length === 0) return [];

		const typeSet = new Set<string>();

		selectedMeshes.forEach((meshcode) => {
			const entries = getEntriesForMesh(meshcode);
			entries.forEach(([, types]) => {
				Object.keys(types).forEach((type) => typeSet.add(type));
			});
		});

		return Array.from(typeSet).toSorted();
	}

	function getFileCountForType(type: string): number {
		if (selectedMeshes.length === 0) return 0;

		let total = 0;
		selectedMeshes.forEach((meshcode) => {
			const entries = getEntriesForMesh(meshcode);
			entries.forEach(([, types]) => {
				if (types[type]) {
					total += types[type]!.length;
				}
			});
		});

		return total;
	}

	function getSelectedRemoteFiles(): RemoteFileInfo[] {
		const files: RemoteFileInfo[] = [];
		selectedMeshes.forEach((meshcode) => {
			const entries = getEntriesForMesh(meshcode);
			entries.forEach(([, types]) => {
				selectedFeatureTypes.forEach((type) => {
					if (types[type]) {
						files.push(...types[type]!);
					}
				});
			});
		});
		return files;
	}

	function selectAllFeatureTypes() {
		selectedFeatureTypes = getAvailableFeatureTypes();
	}

	function normalizeSecondMesh(meshcode: string): string {
		return meshcode.length >= 6 ? meshcode.slice(0, 6) : meshcode;
	}

	function isMeshLoaded(meshcode: string): boolean {
		return loadedMeshcodes.includes(meshcode);
	}

	function markMeshLoaded(meshcode: string) {
		if (!loadedMeshcodes.includes(meshcode)) {
			loadedMeshcodes = [...loadedMeshcodes, meshcode];
		}
	}

	function unmarkMeshLoaded(meshcode: string) {
		loadedMeshcodes = loadedMeshcodes.filter((code) => code !== meshcode);
	}

	function isMeshLoading(meshcode: string): boolean {
		return loadingMeshcodes.includes(meshcode);
	}

	function markMeshLoading(meshcode: string) {
		if (!loadingMeshcodes.includes(meshcode)) {
			loadingMeshcodes = [...loadingMeshcodes, meshcode];
		}
	}

	function unmarkMeshLoading(meshcode: string) {
		loadingMeshcodes = loadingMeshcodes.filter((code) => code !== meshcode);
	}

	async function fetchMetadataForMesh(meshcode: string, { force = false } = {}) {
		const secondMesh = normalizeSecondMesh(meshcode);

		if (!force && isMeshLoaded(secondMesh)) {
			return;
		}
		if (isMeshLoading(secondMesh)) {
			return;
		}

		if (force) {
			unmarkMeshLoaded(secondMesh);
		}

		markMeshLoading(secondMesh);
		fetchError = null;

		try {
			const result = await invoke<FetchCityGmlMetadataResult>('fetch_citygml_metadata', {
				conditions: `m:${secondMesh}`
			});

			if (!result.meshes || Object.keys(result.meshes).length === 0) {
				fetchError = `メッシュ ${secondMesh} の地物情報が見つかりませんでした。`;
				return;
			}

			remoteMeshcodeData = mergeRemoteMeshcodeData(remoteMeshcodeData, result.meshes);

			featureTypeLookup = {
				...featureTypeLookup,
				...result.featureTypes
			};

			const summaries =
				result.cities ??
				([
					{
						cityName: result.cityName,
						cityCode: result.cityCode,
						year: result.year,
						registrationYear: result.registrationYear
					}
				] as CitySummary[]);

			for (const summary of summaries) {
				const key = summary.cityCode ?? `${summary.cityName ?? 'unknown'}-${secondMesh}`;
				citySummaryMap.set(key, summary);
			}
			lastFetchSummary = Array.from(citySummaryMap.values());

			markMeshLoaded(secondMesh);
		} catch (error) {
			if (error && typeof error === 'object' && 'message' in error) {
				fetchError = (error as { message: string }).message;
			} else {
				fetchError = String(error);
			}
		} finally {
			unmarkMeshLoading(secondMesh);
		}
	}

	function refreshSelectedMeshes() {
		const unique = Array.from(new Set(selectedMeshes.map(normalizeSecondMesh)));
		unique.forEach((meshcode) => {
			void fetchMetadataForMesh(meshcode, { force: true });
		});
	}

	$effect(() => {
		const unique = Array.from(new Set(selectedMeshes.map(normalizeSecondMesh)));
		unique.forEach((meshcode) => {
			if (!isMeshLoaded(meshcode) && !isMeshLoading(meshcode)) {
				void fetchMetadataForMesh(meshcode);
			}
		});
	});

	function filterMeshcodeDataBySelection(data: MeshcodeData): MeshcodeData {
		const filtered: MeshcodeData = {};
		selectedMeshes.forEach((meshcode) => {
			const entries = data[meshcode]
				? [[meshcode, data[meshcode]] as const]
				: Object.entries(data).filter(([code]) => meshcode.length === 6 && code.startsWith(meshcode));

			entries.forEach(([code, typeMap]) => {
				selectedFeatureTypes.forEach((type) => {
					if (typeMap[type]) {
						if (!filtered[code]) {
							filtered[code] = {};
						}
						filtered[code][type] = typeMap[type];
					}
				});
			});
		});
		return filtered;
	}

	async function downloadSelectedData() {
		if (selectedMeshes.length === 0) {
			downloadError = 'メッシュが選択されていません。';
			return;
		}
		if (selectedFeatureTypes.length === 0) {
			downloadError = '地物型が選択されていません。';
			return;
		}

		const files = getSelectedRemoteFiles();
		if (files.length === 0) {
			downloadError = '選択した条件に該当するファイルがありません。';
			return;
		}

		const uniqueUrls = Array.from(new Set(files.map((file) => file.url)));

		isDownloading = true;
		downloadError = null;

		try {
			const result = await invoke<DownloadCityGmlPackResult>('download_citygml_pack', {
				urls: uniqueUrls
			});

			if (!result.zipPath) {
				throw new Error('CityGMLパックの保存に失敗しました。');
			}

			inputPaths = [result.zipPath];

			const data = await invoke<MeshcodeData>('get_meshcodes_with_prefix', {
				inputPaths
			});

			const filtered = filterMeshcodeDataBySelection(data);
			meshcodeData = Object.keys(filtered).length > 0 ? filtered : data;
			const availableMeshcodes = Object.keys(meshcodeData ?? {});
			if (availableMeshcodes.length > 0) {
				const expanded = new Set<string>();
				selectedMeshes.forEach((meshcode) => {
					if (meshcodeData && meshcodeData[meshcode]) {
						expanded.add(meshcode);
						return;
					}
					availableMeshcodes
						.filter((code) => code.startsWith(meshcode))
						.forEach((code) => expanded.add(code));
				});
				selectedMeshes = Array.from(expanded);
			}
			packProgress = null;
		} catch (error) {
			if (error && typeof error === 'object' && 'message' in error) {
				downloadError = (error as { message: string }).message;
			} else {
				downloadError = String(error);
			}
		} finally {
			isDownloading = false;
		}
	}
</script>

{#if isDownloading}
	<div class="fixed inset-0 z-30 flex h-screen items-center justify-center bg-black/70 backdrop-blur-[2px]">
		<div class="rounded-lg border border-gray-200 bg-white px-6 py-4 text-center shadow-lg">
			<Icon icon="eos-icons:bubble-loading" class="mx-auto mb-2 text-4xl text-accent1" />
			<p class="text-sm font-semibold">CityGMLパックを作成中...</p>
			{#if packProgress}
				<p class="mt-1 text-xs text-gray-600">
					{packProgress.stage} - {packProgress.status} ({Math.round(packProgress.progress * 100)}%)
				</p>
			{/if}
		</div>
	</div>
{/if}

<div class="flex min-h-0 flex-1 flex-col gap-4">
	<div class="flex flex-col gap-4">
		<div class="flex items-center gap-1.5 border-b-1 pb-1">
			<Icon class="text-xl" icon="material-symbols:map-search" />
			<h2 class="text-xl font-bold">PLATEAUデータ取得</h2>
		</div>

		<p class="text-sm text-gray-600">
			地図上のメッシュを選択すると自動的に利用可能な地物型を取得します。必要に応じて再取得ボタンで更新してください。
		</p>
		<div class="rounded border border-gray-200 bg-white p-3 text-xs text-gray-600">
			<p>
				選択中のメッシュ: <span class="font-semibold">{selectedMeshes.length}</span>
			</p>
			<p class="mt-0.5">
				地物情報を取得済み: <span class="font-semibold">{loadedMeshcodes.length}</span>
			</p>
			{#if loadingMeshcodes.length > 0}
				<p class="mt-1 text-blue-600">
					取得中: {loadingMeshcodes.join(', ')}
				</p>
			{/if}
		</div>
		<div class="flex gap-2">
			<PrimaryButton
				onclick={refreshSelectedMeshes}
				disabled={selectedMeshes.length === 0 || isFetching}
			>
				{isFetching ? '取得中...' : '選択メッシュを再取得'}
			</PrimaryButton>
			<SecondaryButton onclick={resetAll}>リセット</SecondaryButton>
		</div>
		{#if fetchError}
			<div class="rounded border border-red-200 bg-red-50 p-3 text-sm text-red-700">
				{fetchError}
			</div>
		{/if}
		{#if lastFetchSummary.length > 0}
			<div class="rounded border border-gray-200 bg-white p-3 text-xs text-gray-600">
				<p class="font-semibold">取得済み自治体: {lastFetchSummary.length} 件</p>
				<ul class="mt-1 space-y-1 max-h-32 overflow-y-auto">
					{#each lastFetchSummary as city, index (city.cityCode ?? `${city.cityName}-${index}`)}
						<li class="rounded bg-gray-50 px-2 py-1">
							{#if city.cityName}
								<span class="font-medium">{city.cityName}</span>
							{/if}
							{#if city.cityCode}
								<span class="ml-1 text-gray-500">({city.cityCode})</span>
							{/if}
							<div class="mt-0.5">
								<span>年度: {city.year ?? '不明'}</span>
								{#if city.registrationYear}
									<span class="ml-2">登録年度: {city.registrationYear}</span>
								{/if}
							</div>
						</li>
					{/each}
				</ul>
				<p class="mt-2">取得済みメッシュ数: {getAllMeshesCount()} 件</p>
			</div>
		{/if}
	</div>

	<div class="border-t border-gray-200 pt-4">
		<div class="mb-2 flex items-center gap-1.5">
			<Icon class="text-xl" icon="mdi:grid" />
			<h3 class="text-lg font-semibold">メッシュ選択</h3>
		</div>
		<p class="text-xs text-gray-600">
			Shift+ドラッグで範囲選択、Shift+クリックで個別メッシュを選択できます
		</p>
	</div>

	<div class="mt-4 mb-2 flex items-center gap-1.5 border-b pb-1">
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
				<PrimaryButton onclick={selectAllFeatureTypes} disabled={getAvailableFeatureTypes().length === 0}>
					全選択
				</PrimaryButton>
			{/if}
		</div>

		{#if getAvailableFeatureTypes().length === 0}
			<p class="rounded border border-dashed border-gray-300 p-3 text-sm text-gray-600">
				地物型がまだ取得されていません。メッシュを選択すると自動で取得が始まります。
			</p>
		{:else}
			<div class="flex flex-1 flex-col gap-0.5">
				{#each getAvailableFeatureTypes() as type (type)}
					<label
						class="flex cursor-pointer items-center gap-2 rounded bg-gray-100 px-2 py-1 hover:bg-gray-200"
					>
						<input type="checkbox" bind:group={selectedFeatureTypes} value={type} class="rounded" />
						<Icon icon="material-symbols:category" class="text-gray-600" />
						<span class="flex-1">
							<span class="font-medium">{featureTypeLookup[type] ?? getTypeLabel(type)}</span>
							<span class="ml-2 text-xs text-gray-500">({type})</span>
						</span>
						<span class="text-xs text-gray-500">
							{getFileCountForType(type)}ファイル
						</span>
					</label>
				{/each}
			</div>
		{/if}

		<div class="mt-4 space-y-2">
			<PrimaryButton
				onclick={downloadSelectedData}
				disabled={
					isDownloading ||
					selectedFeatureTypes.length === 0 ||
					selectedMeshes.length === 0 ||
					getSelectedRemoteFiles().length === 0
				}
			>
				<Icon icon="material-symbols:download" class="inline text-lg" />
				選択した地物型のCityGMLをダウンロード
			</PrimaryButton>
			{#if downloadError}
				<div class="rounded border border-red-200 bg-red-50 p-2 text-xs text-red-700">
					{downloadError}
				</div>
			{/if}
		</div>

		<div class="flex justify-between gap-2 py-6">
			<SecondaryButton onclick={resetAll}>はじめから</SecondaryButton>
			<PrimaryButton
				onclick={onclickNext}
				disabled={selectedFeatureTypes.length === 0 || !meshcodeData || isDownloading}
			>
				変換設定へ
				<Icon icon="material-symbols:arrow-forward" class="inline" />
			</PrimaryButton>
		</div>
	</div>
</div>
