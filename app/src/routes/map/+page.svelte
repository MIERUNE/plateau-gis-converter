<script lang="ts">
	import TabNavigation from '../TabNavigation.svelte';
	import Icon from '@iconify/svelte';
	import {
		fetchCityGMLMetadataByMeshcodes,
		type FetchCityGmlMetadataResult
	} from '$lib/fetchCityGMLMetadata';
	import FeatureTypeSelectStepSidePanel from './FeatureTypeSelectStepSidePanel.svelte';
	import MapPanel from './MapPanel.svelte';
	import ConvertStepSidePanel from './ConvertStepSidePanel.svelte';
	import type { SinkParameters } from '$lib/sinkparams';
	import type { TransformerSettings } from '$lib/transformer';

	let currentStep: 'featureTypeSelect' | 'convert' = $state('featureTypeSelect');
	let selectedMeshes: string[] = $state([]);
	let selectedFeatureTypes: string[] = $state([]);
	let selectedMeshcodesData: FetchCityGmlMetadataResult | null = $state(null);
	let isSelectedMeshcodesDataLoading = $state(false);

	// Conversion settings
	let filetype: string = $state('gpkg');
	let epsg: number = $state(4979);
	let rulesPath = $state('');
	let outputPath = $state('');
	let sinkParameters = $state({} as SinkParameters);
	let transformerSettings: TransformerSettings | undefined = $state(undefined);

	async function revalidateSelectedMeshcodesData() {
		isSelectedMeshcodesDataLoading = true;
		if (selectedMeshes.length === 0) {
			isSelectedMeshcodesDataLoading = false;
			selectedMeshcodesData = null;
			return;
		}
		selectedMeshcodesData = await fetchCityGMLMetadataByMeshcodes(selectedMeshes, false);
		isSelectedMeshcodesDataLoading = false;
	}

	function clearAll() {
		currentStep = 'featureTypeSelect';
		selectedMeshes = [];
		selectedFeatureTypes = [];
		selectedMeshcodesData = null;
	}
</script>

<div class="flex h-screen">
	<div
		class="flex w-1/3 min-w-96 flex-col gap-8 overflow-y-auto border-r border-gray-200 bg-gray-50 p-4"
	>
		<div class="flex items-center gap-1.5">
			<h1 class="text-2xl font-bold">PLATEAU GIS Converter</h1>
			<a href="/about" class="hover:text-accent1">
				<Icon class="mt-0.5 text-2xl" icon="mingcute:information-line" />
			</a>
		</div>

		<TabNavigation />
		{#if currentStep === 'featureTypeSelect'}
			<FeatureTypeSelectStepSidePanel
				bind:selectedFeatureTypes
				{selectedMeshes}
				{selectedMeshcodesData}
				{clearAll}
				{isSelectedMeshcodesDataLoading}
				onclickNext={() => {
					if (selectedFeatureTypes.length === 0) return;
					currentStep = 'convert';
				}}
			/>
		{:else}
			<ConvertStepSidePanel
				{selectedMeshcodesData}
				{selectedFeatureTypes}
				bind:filetype
				bind:epsg
				bind:rulesPath
				bind:sinkParameters
				bind:transformerSettings
				bind:outputPath
				onclickBack={() => {
					currentStep = 'featureTypeSelect';
				}}
			/>
		{/if}
	</div>

	<div class="flex-1">
		<MapPanel bind:selectedMeshes {revalidateSelectedMeshcodesData} />
	</div>
</div>
