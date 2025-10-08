<script lang="ts">
	import type { Map, MapLayerMouseEvent } from 'maplibre-gl';
	import { getTypeLabel, type MeshcodeData } from './utils';
	import { meshcodeToCenter, meshcodeToPolygon } from '$lib/meshcode';
	import {
		FillLayer,
		GeoJSONSource,
		MapLibre,
		Marker,
		NavigationControl,
		Popup
	} from 'svelte-maplibre-gl';
	import PrimaryButton from './PrimaryButton.svelte';
	import Icon from '@iconify/svelte';

	type Props = {
		meshcodeData: MeshcodeData;
		selectedMeshes: string[];
	};

	let { meshcodeData, selectedMeshes = $bindable() }: Props = $props();

	let mapInstance: Map | undefined = $state();
	// Map state
	let mapCenter: [number, number] = $state([139.7, 35.7]); // Tokyo default
	let mapZoom: number = $state(10);
	let prevMapZoom: number = $state(10);
	let selectedMesh: { meshcode: string; types: string[] }[] | null = $state(null);
	let popupLngLat: [number, number] | null = $state(null);
	let cursor: string | undefined = $state();
	// Box selection state
	let isBoxSelecting: boolean = $state(false);
	let boxSelectionStart: [number, number] | null = $state(null);
	let boxSelectionEnd: [number, number] | null = $state(null);
	let meshLevel: 'second' | 'third' = $state('second');

	$effect(() => {
		// 12以上になったとき
		if (mapZoom > 11 && prevMapZoom <= 11) {
			meshLevel = 'third';
		}
		// 9未満になったとき
		if (mapZoom < 11 && prevMapZoom >= 11) {
			meshLevel = 'second';
		}
		prevMapZoom = mapZoom;
	});

	function handleMeshClick(event: MapLayerMouseEvent) {
		const features = event.features || [];
		if (features.length === 0) return;

		// SelectMode
		if (event.originalEvent.shiftKey) {
			if (isBoxSelecting) {
				return;
			}
			// 最初のメッシュのみを選択/選択解除
			const meshcode = features[0]?.properties?.meshcode;
			if (meshcode) {
				event.originalEvent.preventDefault();
				toggleMeshSelection(meshcode);
			}
			return;
		}

		const meshes: { meshcode: string; types: string[] }[] = [];

		for (const feature of features) {
			const meshcode = feature.properties?.meshcode;

			if (meshcode) {
				const types = [
					...new Set(
						Object.entries(meshcodeData)
							.filter(([code]) => code.startsWith(meshcode))
							.flatMap(([, data]) => {
								return Object.keys(data);
							})
					)
				];

				meshes.push({ meshcode, types });
			}
		}

		if (meshes.length > 0) {
			selectedMesh = meshes;

			if (event.lngLat) {
				popupLngLat = [event.lngLat.lng, event.lngLat.lat];
			} else {
				popupLngLat = meshcodeToCenter(meshes[0].meshcode);
			}
		}
	}

	function toggleMeshSelection(meshcode: string) {
		if (meshcode.length === 6 && selectedMeshes.some((m) => m.startsWith(meshcode))) {
			selectedMeshes = selectedMeshes.filter((m) => !m.startsWith(meshcode));
		} else if (meshcode.length === 8 && selectedMeshes.includes(meshcode)) {
			selectedMeshes = selectedMeshes.filter((m) => m != meshcode);
		} else if (meshcode.length === 6) {
			const availableMeshes = Object.keys(meshcodeData).filter((m) => {
				return m.startsWith(meshcode);
			});

			selectedMeshes = [...selectedMeshes, ...availableMeshes];
		} else {
			const secondMeshcode = meshcode.slice(0, 6);
			if (
				!selectedMeshes.includes(secondMeshcode) &&
				Object.keys(meshcodeData).includes(secondMeshcode)
			) {
				selectedMeshes.push(secondMeshcode);
			}
			selectedMeshes.push(meshcode);
		}
	}

	function handleMapMouseDown(event: MapLayerMouseEvent) {
		if (!event.originalEvent.shiftKey) return;

		isBoxSelecting = true;
		boxSelectionStart = [event.lngLat.lng, event.lngLat.lat];
		boxSelectionEnd = null;
		popupLngLat = null;

		// Disable Map Move
		if (mapInstance) {
			mapInstance.dragPan.disable();
		}
	}

	function handleMapMouseMove(event: MapLayerMouseEvent) {
		if (!boxSelectionStart || !isBoxSelecting) return;
		boxSelectionEnd = [event.lngLat.lng, event.lngLat.lat];
	}

	function handleMapMouseUp() {
		if (!boxSelectionStart || !isBoxSelecting) return;

		isBoxSelecting = false;

		if (boxSelectionStart && boxSelectionEnd) {
			selectMeshesInBounds();
		}

		if (mapInstance) {
			mapInstance.dragPan.enable();
		}

		boxSelectionStart = null;
		boxSelectionEnd = null;
	}

	function selectMeshesInBounds() {
		if (!meshcodeData || !boxSelectionStart || !boxSelectionEnd) return;

		const minLng = Math.min(boxSelectionStart[0], boxSelectionEnd[0]);
		const maxLng = Math.max(boxSelectionStart[0], boxSelectionEnd[0]);
		const minLat = Math.min(boxSelectionStart[1], boxSelectionEnd[1]);
		const maxLat = Math.max(boxSelectionStart[1], boxSelectionEnd[1]);

		const meshesInBounds = Object.keys(meshcodeData).filter((meshcode) => {
			const center = meshcodeToCenter(meshcode);
			return (
				center[0] >= minLng && center[0] <= maxLng && center[1] >= minLat && center[1] <= maxLat
			);
		});

		// 範囲内のメッシュを選択リストに追加（既に選択されているものは除外）
		const newSelections = meshesInBounds.filter((meshcode) => !selectedMeshes.includes(meshcode));
		selectedMeshes = [...selectedMeshes, ...newSelections];
	}

	function generateSecondMeshGeoJSON() {
		if (!meshcodeData) return null;

		const secondMeshcodes = [
			...new Set(Object.keys(meshcodeData).map((meshcode) => meshcode.slice(0, 6)))
		];

		const features = secondMeshcodes.map((meshcode) => {
			return {
				type: 'Feature' as const,
				geometry: meshcodeToPolygon(meshcode),
				properties: {
					meshcode,
					selected: selectedMeshes.some((m) => m.startsWith(meshcode))
				}
			};
		});

		return {
			type: 'FeatureCollection' as const,
			features
		};
	}

	function generateThirdMeshGeoJSON() {
		if (!meshcodeData) return null;

		const thirdMeshcodes = Object.keys(meshcodeData).filter((meshcode) => meshcode.length === 8);

		const features = thirdMeshcodes.map((meshcode) => ({
			type: 'Feature' as const,
			geometry: meshcodeToPolygon(meshcode),
			properties: {
				meshcode,
				selected: selectedMeshes.includes(meshcode)
			}
		}));

		return {
			type: 'FeatureCollection' as const,
			features
		};
	}

	function generateSelectionBoxGeoJSON() {
		if (!isBoxSelecting || !boxSelectionStart || !boxSelectionEnd) return null;

		const minLng = Math.min(boxSelectionStart[0], boxSelectionEnd[0]);
		const maxLng = Math.max(boxSelectionStart[0], boxSelectionEnd[0]);
		const minLat = Math.min(boxSelectionStart[1], boxSelectionEnd[1]);
		const maxLat = Math.max(boxSelectionStart[1], boxSelectionEnd[1]);

		return {
			type: 'FeatureCollection' as const,
			features: [
				{
					type: 'Feature' as const,
					geometry: {
						type: 'Polygon' as const,
						coordinates: [
							[
								[minLng, minLat],
								[maxLng, minLat],
								[maxLng, maxLat],
								[minLng, maxLat],
								[minLng, minLat]
							]
						]
					},
					properties: {}
				}
			]
		};
	}

	$effect(() => {
		fitMapToMeshes();
	});

	function calculateMeshBounds() {
		if (!meshcodeData) return null;

		const meshcodes = Object.keys(meshcodeData);
		if (meshcodes.length === 0) return null;

		let minLng = Infinity,
			maxLng = -Infinity;
		let minLat = Infinity,
			maxLat = -Infinity;

		meshcodes.forEach((meshcode) => {
			const polygon = meshcodeToPolygon(meshcode);
			const coords = polygon.coordinates[0];

			coords.forEach(([lng, lat]) => {
				minLng = Math.min(minLng, lng);
				maxLng = Math.max(maxLng, lng);
				minLat = Math.min(minLat, lat);
				maxLat = Math.max(maxLat, lat);
			});
		});

		return {
			sw: [minLng, minLat] as [number, number],
			ne: [maxLng, maxLat] as [number, number]
		};
	}

	function fitMapToMeshes() {
		if (!mapInstance || !meshcodeData) return;

		const bounds = calculateMeshBounds();
		if (!bounds) return;

		mapInstance.fitBounds([bounds.sw, bounds.ne], {
			padding: 50,
			duration: 1000
		});
	}
</script>

<MapLibre
	bind:map={mapInstance}
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
	bind:zoom={mapZoom}
	boxZoom={false}
	onclick={() => {
		selectedMesh = null;
		popupLngLat = null;
	}}
	onmousedown={handleMapMouseDown}
	onmousemove={handleMapMouseMove}
	onmouseup={handleMapMouseUp}
	onload={() => {
		setTimeout(fitMapToMeshes, 100);
	}}
	{cursor}
>
	<GeoJSONSource
		data={generateSecondMeshGeoJSON() || { type: 'FeatureCollection', features: [] }}
		id="second-mesh-source"
	>
		<FillLayer
			paint={{
				'fill-color': [
					'case',
					['get', 'selected'],
					'#ff9800',
					['boolean', ['feature-state', 'hover'], false],
					'#ff6b6b',
					'#4dabf7'
				],
				'fill-opacity': ['case', ['get', 'selected'], 0.8, 0.6],
				'fill-outline-color': ['case', ['get', 'selected'], '#f57c00', '#339af0']
			}}
			layout={{
				visibility: meshLevel === 'second' ? 'visible' : 'none'
			}}
			onmousemove={() => {
				cursor = 'pointer';
			}}
			onmouseleave={() => {
				cursor = undefined;
			}}
			onclick={handleMeshClick}
		/>
	</GeoJSONSource>

	<GeoJSONSource
		data={generateThirdMeshGeoJSON() || { type: 'FeatureCollection', features: [] }}
		id="third-mesh-source"
	>
		<FillLayer
			paint={{
				'fill-color': [
					'case',
					['get', 'selected'],
					'#ff9800',
					['boolean', ['feature-state', 'hover'], false],
					'#ff6b6b',
					'#4dabf7'
				],
				'fill-opacity': ['case', ['get', 'selected'], 0.8, 0.6],
				'fill-outline-color': ['case', ['get', 'selected'], '#f57c00', '#339af0']
			}}
			layout={{
				visibility: meshLevel === 'third' ? 'visible' : 'none'
			}}
			onmousemove={() => {
				cursor = 'pointer';
			}}
			onmouseleave={() => {
				cursor = undefined;
			}}
			onclick={handleMeshClick}
		/>
	</GeoJSONSource>

	<!-- Selection box overlay -->
	{#if isBoxSelecting}
		<GeoJSONSource
			data={generateSelectionBoxGeoJSON() || { type: 'FeatureCollection', features: [] }}
			id="selection-box-source"
		>
			<FillLayer
				paint={{
					'fill-color': '#2196f3',
					'fill-opacity': 0.2,
					'fill-outline-color': '#2196f3'
				}}
			/>
		</GeoJSONSource>
	{/if}

	<!-- Popup for selected mesh -->
	{#if selectedMesh && popupLngLat}
		<Marker lnglat={popupLngLat}>
			{#snippet content()}
				<div></div>
			{/snippet}
			<Popup
				open={true}
				closeButton={false}
				onclose={() => {
					selectedMesh = null;
					popupLngLat = null;
				}}
			>
				<div class="relative min-w-48 px-1 py-3">
					<button
						class="absolute top-0.5 right-0.5 size-4 cursor-pointer rounded hover:bg-black/20"
						onclick={() => {
							selectedMesh = null;
							popupLngLat = null;
						}}
					>
						<Icon class="size-full" icon="material-symbols:close" />
					</button>
					<div class="max-h-80 space-y-2 overflow-y-auto px-4">
						{#each selectedMesh as mesh (mesh.meshcode)}
							<h3 class="mb-2 text-sm font-semibold">
								メッシュコード: {mesh.meshcode}
							</h3>
							<ul class="mb-3 list-inside list-disc space-y-1">
								{#each mesh.types as type (type)}
									<li class="rounded pl-2 text-xs">
										<span class="font-medium">{getTypeLabel(type)}</span>
										<span class="ml-1 text-gray-500">({type})</span>
									</li>
								{/each}
							</ul>
							<div class="flex gap-2">
								{#if selectedMeshes.includes(mesh.meshcode)}
									<button
										onclick={() => {
											if (mesh) {
												toggleMeshSelection(mesh.meshcode);
											}
										}}
										class="flex-1 rounded bg-red-500 px-4 py-1 text-sm text-white hover:opacity-75"
									>
										選択解除
									</button>
								{:else}
									<PrimaryButton
										class="w-full"
										onclick={() => {
											if (mesh) {
												toggleMeshSelection(mesh.meshcode);
											}
										}}
									>
										選択
									</PrimaryButton>
								{/if}
							</div>
						{/each}
					</div>
				</div>
			</Popup>
		</Marker>
	{/if}
	<div class="absolute top-2 left-2 z-10 flex">
		<button
			data-selected={meshLevel === 'second' ? '' : undefined}
			onclick={() => {
				meshLevel = 'second';
				selectedMesh = null;
				popupLngLat = null;
			}}
			type="button"
			class="relative inline-flex cursor-pointer items-center gap-2 rounded-l-md bg-white px-4 py-2 text-sm font-semibold text-gray-900 ring-1 ring-gray-300 ring-inset hover:bg-gray-50 focus:z-10 data-selected:pointer-events-none data-selected:bg-accent1"
		>
			2次メッシュ
		</button>
		<button
			data-selected={meshLevel === 'third' ? '' : undefined}
			onclick={() => {
				meshLevel = 'third';
				selectedMesh = null;
				popupLngLat = null;
			}}
			type="button"
			class="relative -ml-px inline-flex cursor-pointer items-center gap-2 rounded-r-md bg-white px-4 py-2 text-sm font-semibold text-gray-900 ring-1 ring-gray-300 ring-inset hover:bg-gray-50 focus:z-10 data-selected:pointer-events-none data-selected:bg-accent1"
		>
			3次メッシュ
		</button>
	</div>
	<NavigationControl />
</MapLibre>

<style>
	:global {
		.maplibregl-popup-content {
			padding: 0 !important;
		}
	}
</style>
