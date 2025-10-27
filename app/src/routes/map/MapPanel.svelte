<script lang="ts">
	import {
		fetchCityGMLMetadataByMeshcodes,
		type FetchCityGmlMetadataResult
	} from '$lib/fetchCityGMLMetadata';
	import {
		makeMeshPatches,
		type MeshLevelEntry,
		type MeshLevelKey,
		type MeshPatchFeatureCollection
	} from '$lib/japanMeshMaker';
	import Icon from '@iconify/svelte';
	import type { Map, MapLayerMouseEvent } from 'maplibre-gl';
	import { Map as MaplibreMap, GeoJSONSource, FillLayer, Marker, Popup } from 'svelte-maplibre-gl';
	import PrimaryButton from '../PrimaryButton.svelte';

	const MESH_LEVEL_ENTRIES: Record<MeshLevelKey, MeshLevelEntry> = Object.freeze({
		'1': { name: '1次メッシュ', minZoom: 1, maxZoom: 7, parentLevel: null, standardZoom: 7.5 },
		'2': { name: '2次メッシュ', minZoom: 8, maxZoom: 10, parentLevel: '1', standardZoom: 10.5 },
		'3': { name: '3次メッシュ', minZoom: 11, maxZoom: 13, parentLevel: '2', standardZoom: 13.5 }
	});

	interface Props {
		selectedMeshes: string[];
		revalidateSelectedMeshcodesData: () => Promise<void>;
	}

	let { selectedMeshes = $bindable(), revalidateSelectedMeshcodesData }: Props = $props();

	let meshLevel: MeshLevelKey = $state('1');
	let meshPolygons: MeshPatchFeatureCollection | null = $state(null);
	let map: Map | undefined = $state(undefined);
	let mapCenter: [number, number] = $state([139.7, 35.7]); // Tokyo default
	let mapZoom: number = $state(10);
	let cursor: string | undefined = $state();
	let popupMeshcode: string | null = $state(null);
	let popupData: FetchCityGmlMetadataResult | null = $state(null);
	let isPopupDataLoading: boolean = $state(false);
	let popupLngLat: [number, number] | null = $state(null);

	function remakeMeshData() {
		if (map) {
			const bounds = map.getBounds();
			const zoom = map.getZoom();
			Object.entries(MESH_LEVEL_ENTRIES).forEach(([key, value]) => {
				if (zoom >= value.minZoom) {
					meshLevel = key as MeshLevelKey;
				}
			});
			const sw = bounds.getSouthWest();
			const ne = bounds.getNorthEast();
			const padding = Math.max(ne.lng - sw.lng, ne.lat - sw.lat) * 0.1;
			meshPolygons = makeMeshPatches(
				meshLevel,
				[
					sw.lng - (ne.lng - sw.lng) * padding,
					sw.lat - (ne.lat - sw.lat) * padding,
					ne.lng + (ne.lng - sw.lng) * padding,
					ne.lat + (ne.lat - sw.lat) * padding
				],
				selectedMeshes
			);
		}
	}

	async function handleMeshClick(e: MapLayerMouseEvent) {
		if (popupData || popupMeshcode || popupLngLat) {
			return;
		}
		isPopupDataLoading = true;
		const features = e.features;
		if (features && features.length > 0) {
			const meshcode = features[0].properties?.code;
			if (meshcode) {
				popupMeshcode = meshcode;
				popupLngLat = [e.lngLat.lng, e.lngLat.lat];
				popupData = await fetchCityGMLMetadataByMeshcodes([meshcode]);
			}
		}
		isPopupDataLoading = false;
	}
</script>

<MaplibreMap
	class="h-full w-full"
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
	bind:map
	center={mapCenter}
	bind:zoom={mapZoom}
	boxZoom={false}
	onstyledata={remakeMeshData}
	onmove={remakeMeshData}
	onmoveend={remakeMeshData}
	{cursor}
>
	{#if meshPolygons}
		<GeoJSONSource data={meshPolygons}>
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
					'fill-opacity': 0.6,
					'fill-outline-color': '#339af0'
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
	{/if}
	{#if popupMeshcode && popupLngLat}
		<Marker lnglat={popupLngLat}>
			{#snippet content()}
				<div></div>
			{/snippet}
			<Popup
				open={true}
				closeButton={false}
				onclose={() => {
					popupData = null;
					popupMeshcode = null;
					popupLngLat = null;
				}}
			>
				<div class="relative min-w-48 px-1 py-3">
					<button
						class="absolute top-0.5 right-0.5 size-4 cursor-pointer rounded hover:bg-black/20"
						onclick={() => {
							popupData = null;
							popupMeshcode = null;
							popupLngLat = null;
						}}
					>
						<Icon class="size-full" icon="material-symbols:close" />
					</button>
					{#if isPopupDataLoading}
						<div class="flex h-24 w-48 items-center justify-center">
							<Icon class="animate-spin text-2xl text-gray-500" icon="mdi:loading" />
						</div>
					{:else if popupData && !popupData.success}
						<div class="p-4 text-sm text-red-500">
							エラー: {popupData.message} ({popupData.type})
						</div>
					{:else if popupData && popupData.success}
						<div class="max-h-80 space-y-2 overflow-y-auto px-4">
							<h3 class="mb-2 text-sm font-semibold">
								メッシュコード: {popupMeshcode}
							</h3>

							<ul class="mb-3 list-inside list-disc space-y-1">
								{#each Object.entries(popupData.featureTypes) as [type, info] (type)}
									<li class="rounded pl-2 text-xs">
										<span class="font-medium">{info.label}</span>
										<span class="ml-1 text-gray-500">({type})</span>
									</li>
								{/each}
							</ul>
							<div class="flex gap-2">
								{#if selectedMeshes.includes(popupMeshcode)}
									<button
										onclick={() => {
											selectedMeshes = selectedMeshes.filter(
												(m) => !!popupData?.success && !(m in popupData.meshes)
											);
											revalidateSelectedMeshcodesData();
											remakeMeshData();
										}}
										class="flex-1 rounded bg-red-500 px-4 py-1 text-sm text-white hover:opacity-75"
									>
										選択解除
									</button>
								{:else}
									<PrimaryButton
										class="w-full"
										onclick={() => {
											selectedMeshes = new Array(
												...new Set([
													...selectedMeshes,
													...(popupData?.success ? Object.keys(popupData.meshes) : [])
												])
											);
											revalidateSelectedMeshcodesData();
											remakeMeshData();
										}}
									>
										選択
									</PrimaryButton>
								{/if}
							</div>
						</div>
					{/if}
				</div>
			</Popup>
		</Marker>
	{/if}
</MaplibreMap>

<style>
	:global {
		.maplibregl-popup-content {
			padding: 0 !important;
		}
	}
</style>
