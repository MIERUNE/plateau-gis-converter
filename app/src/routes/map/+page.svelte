<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import Icon from '@iconify/svelte';
	import * as dialog from '@tauri-apps/plugin-dialog';
	import { abbreviatePath } from '$lib/utils';
	import { MapLibre, Popup, FillLayer, GeoJSONSource, Marker } from 'svelte-maplibre-gl';
	import { meshcodeToPolygon, meshcodeToCenter } from '$lib/meshcode';

	// Structure: { meshcode: { type: [filepath1, filepath2, ...] } }
	type MeshcodeData = Record<string, Record<string, string[]>>;

	// Type labels in Japanese
	const TYPE_LABELS: Record<string, string> = {
		bldg: '建築物',
		tran: '交通',
		fld: '洪水浸水想定区域',
		luse: '土地利用',
		lsld: '土砂災害警戒区域',
		urf: '都市計画区域',
		frn: '都市設備',
		veg: '植生',
		dem: '地形',
		wtr: '水部',
		tnl: 'トンネル',
		cons: 'その他の構造物',
		gen: '汎用都市オブジェクト',
		brid: '橋梁',
		ubld: '地下建物',
		rwy: '鉄道',
		trk: '徒歩道',
		squr: '広場',
		wwy: '航路',
		tnm: '津波浸水想定',
		htd: '高潮浸水想定区域',
		ifld: '内水浸水想定区域'
	};

	let meshcodeData: MeshcodeData | null = $state(null);
	let inputPaths: string[] = $state([]);
	let loading: boolean = $state(false);
	let error: string | null = $state(null);

	// Map state
	let showMap: boolean = $state(false);
	let mapCenter: [number, number] = $state([139.7, 35.7]); // Tokyo default
	let mapZoom: number = $state(10);
	let selectedMesh: { meshcode: string; types: string[] } | null = $state(null);
	let popupLngLat: [number, number] | null = $state(null);
	let cursor: string | undefined = $state();

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

		loading = true;
		error = null;

		try {
			meshcodeData = await invoke<MeshcodeData>('get_meshcodes_with_prefix', {
				inputPaths: inputPaths
			});
			if (meshcodeData && Object.keys(meshcodeData).length > 0) {
				// Calculate map center from first meshcode
				const firstMeshcode = Object.keys(meshcodeData)[0];
				mapCenter = meshcodeToCenter(firstMeshcode);
				mapZoom = firstMeshcode.length === 6 ? 12 : 14;
				showMap = true;
			}
		} catch (e) {
			error = e as string;
			meshcodeData = null;
		} finally {
			loading = false;
		}
	}

	function clearSelected() {
		inputPaths = [];
		meshcodeData = null;
		error = null;
		showMap = false;
		selectedMesh = null;
		popupLngLat = null;
	}

	function removeFile(index: number) {
		inputPaths = inputPaths.filter((_, i) => i !== index);
		if (inputPaths.length === 0) {
			clearSelected();
		} else {
			// 残りのファイルで再読み込み
			loadFiles();
		}
	}

	async function loadFiles() {
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

	function getTypeLabel(type: string): string {
		return TYPE_LABELS[type] || type;
	}

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

	// Handle mesh polygon click
	function handleMeshClick(meshcode: string, event: any) {
		console.log('handleMeshClick called with:', meshcode, event);
		const types = meshcodeData ? Object.keys(meshcodeData[meshcode] || {}) : [];
		selectedMesh = { meshcode, types };

		// svelte-maplibre-glのイベント構造を確認
		let clickPosition = null;

		if (event.lngLat) {
			clickPosition = [event.lngLat.lng, event.lngLat.lat];
		} else if (event.detail?.lngLat) {
			clickPosition = [event.detail.lngLat.lng, event.detail.lngLat.lat];
		} else if (event.detail?.point) {
			// pointからlngLatに変換が必要な場合
			console.log('Event has point but no lngLat:', event.detail.point);
			clickPosition = meshcodeToCenter(meshcode);
		} else {
			// フォールバック：メッシュの中心座標を使用
			clickPosition = meshcodeToCenter(meshcode);
		}

		popupLngLat = clickPosition;
		console.log('Popup will be shown at:', popupLngLat);
	}

	// Generate GeoJSON for all mesh polygons
	function generateMeshGeoJSON() {
		if (!meshcodeData) return null;

		const features = Object.keys(meshcodeData).map((meshcode) => ({
			type: 'Feature' as const,
			geometry: meshcodeToPolygon(meshcode),
			properties: {
				meshcode,
				types: Object.keys(meshcodeData[meshcode]),
				fileCount: Object.values(meshcodeData[meshcode]).reduce(
					(sum, files) => sum + files.length,
					0
				)
			}
		}));
		console.log('Generated GeoJSON features:', features);

		return {
			type: 'FeatureCollection' as const,
			features
		};
	}
</script>

<div class="flex h-screen">
	<!-- Left panel for file selection and data display -->
	<div
		class="flex w-1/3 min-w-96 flex-col gap-4 overflow-y-auto border-r border-gray-200 bg-gray-50 p-4"
	>
		<div class="flex items-center gap-1.5">
			<h1 class="text-2xl font-bold">PLATEAU ZIP Viewer</h1>
			<a href="/" class="hover:text-accent1">
				<Icon class="mt-0.5 text-2xl" icon="mingcute:information-line" />
			</a>
		</div>

		<div>
			<div class="mb-2 flex items-center gap-1.5">
				<Icon class="text-xl" icon="material-symbols:input-rounded" />
				<h2 class="text-xl font-bold">ZIPファイル選択</h2>
			</div>
			<hr class="mb-4" />

			<div class="ml-3">
				<div class="mb-4 flex items-center gap-3">
					<button
						onclick={openFileDialog}
						class="rounded-sm bg-accent1 px-4 py-1 font-semibold shadow-sm hover:opacity-75 disabled:opacity-50"
						disabled={loading}
					>
						{loading ? '読み込み中...' : '選択'}
					</button>

					<div class="flex-1 text-sm">
						{#if inputPaths.length === 0}
							<p class="text-gray-500">PLATEAUのZIPファイルを選択してください（複数選択可）</p>
						{:else}
							<div class="space-y-1">
								{#each inputPaths as path, index (path)}
									<div class="flex items-center gap-1 rounded bg-gray-100 px-2 py-1">
										<Icon icon="material-symbols:archive" class="text-gray-600" />
										<p class="flex-1 truncate font-medium" title={path}>
											{abbreviatePath(path, 4)}
										</p>
										<button
											onclick={() => removeFile(index)}
											class="text-gray-600 hover:opacity-75"
										>
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
				</div>

				{#if error}
					<div class="mb-4 rounded border border-red-200 bg-red-50 p-3">
						<p class="text-sm text-red-700">エラー: {error}</p>
					</div>
				{/if}

				{#if meshcodeData}
					<div class="mt-6">
						<h3 class="mb-3 text-lg font-semibold">検出されたデータ</h3>

						{#if Object.keys(meshcodeData).length === 0}
							<p class="text-gray-500">PLATEAUデータが見つかりませんでした</p>
						{:else}
							<div class="space-y-4">
								{#each Object.entries(meshcodeData) as [meshcode, types] (meshcode)}
									<div class="rounded-lg border bg-gray-50 p-4">
										<h4 class="text-md mb-3 flex items-center gap-2 font-semibold">
											<span class="text-accent1">メッシュコード: {meshcode}</span>
											<span class="ml-auto text-sm text-gray-600">
												{Object.values(types).reduce((sum, files) => sum + files.length, 0)}
												ファイル
											</span>
										</h4>

										<div class="space-y-2">
											{#each Object.entries(types) as [type, filepaths] (meshcode + type)}
												<div class="rounded border bg-white px-3 py-2">
													<div class="mb-1 flex items-center gap-2">
														<span class="text-sm font-medium">{getTypeLabel(type)}</span>
														<span class="text-xs text-gray-500">({type})</span>
														<span class="ml-auto text-xs text-gray-600"
															>{filepaths.length}
															ファイル</span
														>
													</div>
													{#if filepaths.length === 1}
														<div class="truncate text-xs text-gray-600" title={filepaths[0]}>
															{getFilename(filepaths[0])}
														</div>
													{:else}
														<details class="text-xs">
															<summary class="cursor-pointer text-gray-600 hover:text-gray-800">
																{filepaths.length} ファイル (クリックで展開)
															</summary>
															<div class="mt-1 space-y-1 border-l-2 border-gray-200 pl-2">
																{#each filepaths as filepath (meshcode + type + filepath)}
																	<div
																		class="truncate text-gray-600 hover:text-gray-800"
																		title={filepath}
																	>
																		• {getFilename(filepath)}
																	</div>
																{/each}
															</div>
														</details>
													{/if}
												</div>
											{/each}
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/if}
			</div>
		</div>
	</div>

	<!-- Right panel for map -->
	<div class="flex-1">
		{#if showMap && meshcodeData}
			<MapLibre
				style={{
					version: 8,
					sources: {
						pale: {
							type: 'raster',
							tiles: ['https://cyberjapandata.gsi.go.jp/xyz/pale/{z}/{x}/{y}.png'],
							tileSize: 256,
							attribution: '国土地理院'
						}
					},
					layers: [
						{
							id: 'pale-layer',
							type: 'raster',
							source: 'pale'
						}
					]
				}}
				class="h-full w-full"
				center={mapCenter}
				zoom={mapZoom}
				onclick={() => {
					selectedMesh = null;
					popupLngLat = null;
				}}
				{cursor}
			>
				<GeoJSONSource data={generateMeshGeoJSON()} id="mesh-source">
					<FillLayer
						paint={{
							'fill-color': [
								'case',
								['boolean', ['feature-state', 'hover'], false],
								'#ff6b6b',
								'#4dabf7'
							],

							'fill-opacity': 0.6,
							'fill-outline-color': '#339af0'
						}}
						onmousemove={() => {
							cursor = 'pointer';
						}}
						onmouseleave={() => {
							cursor = undefined;
						}}
						onclick={(e) => {
							console.log('FillLayer click event:', e);
							console.log('Event detail:', e.detail);
							console.log('Event features:', e.features);

							// Svelte 5でのイベントプロパゲーション停止
							if (e.stopPropagation) e.stopPropagation();
							if (e.preventDefault) e.preventDefault();

							// フィーチャー情報の取得方法を複数試す
							let meshcode = null;
							if (e.features?.[0]?.properties?.meshcode) {
								meshcode = e.features[0].properties.meshcode;
							} else if (e.detail?.features?.[0]?.properties?.meshcode) {
								meshcode = e.detail.features[0].properties.meshcode;
							}

							if (meshcode) {
								console.log('Found meshcode:', meshcode);
								handleMeshClick(meshcode, e);
							} else {
								console.log('No meshcode found in click event');
							}
						}}
					/>
				</GeoJSONSource>

				<!-- Popup for selected mesh -->
				{#if selectedMesh && popupLngLat}
					<!-- Debug info -->
					{console.log('Rendering popup for mesh:', selectedMesh.meshcode, 'at:', popupLngLat)}
					<Marker lnglat={popupLngLat}>
						<Popup
							open={true}
							onclose={() => {
								console.log('Popup close event triggered');
								selectedMesh = null;
								popupLngLat = null;
							}}
						>
							<div class="min-w-48 p-2">
								<h3 class="mb-2 text-sm font-semibold">メッシュコード: {selectedMesh.meshcode}</h3>
								<div class="space-y-1">
									{#each selectedMesh.types as type}
										<div class="rounded bg-gray-100 px-2 py-1 text-xs">
											<span class="font-medium">{getTypeLabel(type)}</span>
											<span class="ml-1 text-gray-500">({type})</span>
										</div>
									{/each}
								</div>
							</div>
						</Popup>
					</Marker>
				{/if}
			</MapLibre>
		{:else}
			<div class="flex h-full w-full items-center justify-center bg-gray-100 text-gray-500">
				<div class="text-center">
					<Icon icon="material-symbols:map" class="mb-4 text-6xl opacity-50" />
					<p class="text-lg">PLATEAUファイルを選択すると地図が表示されます</p>
				</div>
			</div>
		{/if}
	</div>
</div>
