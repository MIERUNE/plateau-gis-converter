<script lang="ts">
	import Icon from '@iconify/svelte';
	import TabNavigation from '../TabNavigation.svelte';
	import type { SinkParameters } from '$lib/sinkparams';
	import type { TransformerSettings } from '$lib/transformer';
	import type { MeshcodeData } from './utils';
	import FeatureTypeSelectStepSidePanel from './FeatureTypeSelectStepSidePanel.svelte';
	import ConvertStepSidePanel from './ConvertStepSidePanel.svelte';
	import MapPanel from './MapPanel.svelte';

	let meshcodeData: MeshcodeData | null = $state(null);
	let inputPaths: string[] = $state([]);

	let selectedMeshes: string[] = $state([]);

	// New UI flow state
	let currentStep: 'featureTypeSelect' | 'convert' = $state('featureTypeSelect');
	let selectedFeatureTypes: string[] = $state([]);

	// Conversion settings
	let filetype: string = $state('gpkg');
	let epsg: number = $state(4979);
	let rulesPath = $state('');
	let outputPath = $state('');
	let sinkParameters = $state({} as SinkParameters);
	let transformerSettings: TransformerSettings | undefined = $state(undefined);

	function clearAll() {
		inputPaths = [];
		meshcodeData = null;
		selectedMeshes = [];
		currentStep = 'featureTypeSelect';
		selectedFeatureTypes = [];
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

		{#if currentStep === "featureTypeSelect" || !meshcodeData}
			<FeatureTypeSelectStepSidePanel
				bind:selectedFeatureTypes
				bind:selectedMeshes
				bind:inputPaths
				bind:meshcodeData
				{clearAll}
				onclickNext={() => {
					if (selectedFeatureTypes.length === 0) return;
					currentStep = 'convert';
				}}
			/>
		{:else if currentStep === 'convert'}
			<ConvertStepSidePanel
				{meshcodeData}
				{selectedFeatureTypes}
				{selectedMeshes}
				{inputPaths}
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
		{#if meshcodeData}
			<MapPanel bind:selectedMeshes {meshcodeData} />
		{:else}
			<div class="flex h-full w-full items-center justify-center bg-gray-100 text-gray-500">
				<div class="flex flex-col items-center gap-4 text-center">
					<Icon icon="material-symbols:map" class="text-6xl opacity-50" />
					<p class="text-lg">PLATEAU ZIP ファイルを選択すると地図が表示されます</p>
				</div>
			</div>
		{/if}
	</div>
</div>
