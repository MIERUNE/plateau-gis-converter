<script lang="ts">
	// NOTE debug
	import { info, warn, trace, error, debug, attachConsole } from 'tauri-plugin-log-api';
	const dettach = attachConsole();

	import { filetypeOptions } from '$lib/settings';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { SinkParameters } from '$lib/sinkparams';
	import type { TransformerRegistry } from '$lib/transformer';
	import { isBooleanConfig, isSelectionConfig } from '$lib/transformer';
	import {
		isIntegerParameter,
		isStringParameter,
		isBooleanParameter,
		createRangeArray
	} from '$lib/sinkparams';
	import Icon from '@iconify/svelte';
	import { dialog } from '@tauri-apps/api';

	export let filetype: string;
	export let epsg: number = 4979;
	export let rulesPath: string;
	export let sinkParameters: SinkParameters;
	export let transformerRegistry: TransformerRegistry;

	$: epsgOptions = filetypeOptions[filetype]?.epsg || [];
	$: disableEpsgOptions = epsgOptions.length < 2;

	$: {
		// Reset the target CRS if the selected filetype does not support the current CRS
		if (!filetypeOptions[filetype]?.epsg.some((item) => item.value === epsg)) {
			epsg = filetypeOptions[filetype]?.epsg[0].value || 4979;
		}
	}

	function clearRulesPath() {
		rulesPath = '';
	}

	async function openRulesPathDialog() {
		const res = await dialog.open({
			filters: [
				{
					name: 'Mapping rule format',
					extensions: ['json']
				}
			]
		});
		if (!res) return;
		rulesPath = Array.isArray(res) ? res[0] : res;
	}

	let DEBUG: any;

	async function getTransformerRegistry(filetype: string) {
		const registry = (await invoke('get_transform', { filetype })) as any;

		transformerRegistry = registry;
	}

	$: getTransformerRegistry(filetype);

	let sinkOptionKeys: string[] = [];

	// Get the parameter options for the selected filetype
	async function setSinkParameter(filetype: string) {
		const parameters = (await invoke('get_parameter', { filetype })) as SinkParameters;

		// Exclude '@output' and 'transform' from the parameter options list
		sinkOptionKeys = Object.keys(parameters.items).filter(
			(item) => item !== '@output' && item !== 'transform'
		);
		sinkParameters = parameters;
	}

	$: setSinkParameter(filetype);

	// Select the file format specified for testing, if any
	const testSink = import.meta.env.VITE_TEST_SINK;
	if (testSink && filetypeOptions.hasOwnProperty(testSink)) filetype = testSink;
</script>

<div>
	<div class="flex items-center gap-1.5">
		<Icon class="text-xl" icon="material-symbols:settings" />
		<h2 class="font-bold text-xl">設定</h2>
	</div>
	<hr class="mt-0.5" />

	<div class="flex flex-col gap-5 mt-3 ml-2">
		<div class="flex flex-col gap-1.5">
			<label for="filetype-select" class="font-bold">ファイル形式</label>
			<select bind:value={filetype} name="filetype" id="filetype-select" class="w-80">
				{#each Object.entries(filetypeOptions) as [value, item]}
					<option {value}>{item.label}</option>
				{/each}
			</select>
		</div>

		<div class="flex flex-col gap-1.5">
			<label for="epsg-select" class="font-bold">座標参照系</label>
			<select
				bind:value={epsg}
				name="epsg"
				id="epsg-select"
				class="w-80"
				disabled={disableEpsgOptions}
				class:opacity-50={disableEpsgOptions}
			>
				{#each epsgOptions as option}
					<option value={option.value}>{option.label}</option>
				{/each}
			</select>
		</div>

		{#if (transformerRegistry && transformerRegistry.configs.length > 0) || sinkOptionKeys.length > 0}
			<div class="flex flex-col gap-1.5">
				<label for="transform-select" class="font-bold">出力の詳細設定</label>

				<!-- Transformer options -->
				{#if transformerRegistry && transformerRegistry.configs.length > 0}
					{#each transformerRegistry.configs as config}
						{#if isBooleanConfig(config.parameter)}
							<div class="flex gap-2 w-80">
								<label for={config.key} class="w-3/4">
									{config.label}
								</label>
								<div class="relative inline-block w-10 h-6 rounded-full cursor-pointer">
									<input
										bind:checked={config.parameter.Boolean}
										id={config.key}
										type="checkbox"
										class="absolute w-10 h-6 transition-colors duration-300 rounded-full appearance-none cursor-pointer peer bg-gray-200 checked:bg-accent1 peer-checked:before:bg-accent1"
									/>
									<label
										for={config.key}
										class="before:content[''] absolute top-2/4 -left-1 h-6 w-6 -translate-y-2/4 cursor-pointer rounded-full border border-blue-gray-100 bg-white shadow-md transition-all duration-300 peer-checked:translate-x-full"
									>
										<div
											class="inline-block p-5 rounded-full top-2/4 left-2/4 -translate-x-2/4 -translate-y-2/4"
										></div>
									</label>
								</div>
							</div>
						{:else if isSelectionConfig(config.parameter)}
							<div class="flex gap-2 w-80">
								<label for={config.key} class="w-2/4">{config.label}</label>
								<select
									id={config.key}
									class="w-2/4 border-2 border-gray-300 px-2 rounded-md"
									bind:value={config.parameter.Selection.selected_value}
								>
									{#each config.parameter.Selection.options as option, index (index)}
										<option value={option.value}>{option.label}</option>
									{/each}
								</select>
							</div>
						{/if}
					{/each}
				{/if}

				<!-- Sink options -->
				{#if sinkOptionKeys.length > 0}
					<div class="flex flex-col gap-2">
						{#each sinkOptionKeys as key}
							{#if isIntegerParameter(sinkParameters.items[key].parameter)}
								<div class="flex gap-2 w-80">
									<label for={key} class="w-3/4">{sinkParameters.items[key].label}</label>
									<select
										bind:value={sinkParameters.items[key].parameter.Integer.value}
										id={key}
										class="w-1/4 border-2 border-gray-300 px-2 rounded-md"
									>
										{#each createRangeArray(sinkParameters.items[key].parameter.Integer.min, sinkParameters.items[key].parameter.Integer.max) as value}
											<option {value} class="text-center">{value}</option>
										{/each}
									</select>
								</div>
							{:else if isStringParameter(sinkParameters.items[key].parameter)}
								<!-- TODO String input -->
								<!-- <div class="flex gap-2 w-80">
								<label for={key} class="w-3/4">{sinkParameters.items[key].label}</label>
								<input
									type="text"
									id={key}
									bind:value={sinkParameters.items[key].parameter.String.value}
									class="w-1/4"
								/>
							</div> -->
							{:else if isBooleanParameter(sinkParameters.items[key].parameter)}
								<div class="flex gap-2 w-80">
									<label for={key} class="w-3/4">{sinkParameters.items[key].label}</label>

									<div class="relative inline-block w-10 h-6 rounded-full cursor-pointer">
										<input
											bind:checked={sinkParameters.items[key].parameter.Boolean.value}
											id={key}
											type="checkbox"
											class="absolute w-10 h-6 transition-colors duration-300 rounded-full appearance-none cursor-pointer peer bg-gray-200 checked:bg-accent1 peer-checked:before:bg-accent1"
										/>
										<label
											for={key}
											class="before:content[''] absolute top-2/4 -left-1 h-6 w-6 -translate-y-2/4 cursor-pointer rounded-full border border-blue-gray-100 bg-white shadow-md transition-all duration-300 peer-checked:translate-x-full"
										>
											<div
												class="inline-block p-5 rounded-full top-2/4 left-2/4 -translate-x-2/4 -translate-y-2/4"
											></div>
										</label>
									</div>
								</div>
							{/if}
						{/each}
					</div>
				{/if}
			</div>
		{/if}

		<div class=" flex flex-col gap-1.5">
			<label for="mapping-rule-select" class="font-bold">属性マッピングルール</label>
			<div class="flex items-center gap-3">
				<button
					on:click={openRulesPathDialog}
					class="bg-accent1 font-semibold rounded px-4 py-0.5 shadow hover:opacity-75">選択</button
				>
				<div class="text-sm" class:opacity-50={!rulesPath}>
					{#if rulesPath}
						<div class="flex justify-center items-center gap-1.5">
							<p><code>{rulesPath}</code></p>
							<button on:click={clearRulesPath} class="hover:opacity-75">
								<Icon icon="material-symbols:cancel" />
							</button>
						</div>
					{:else}
						<p>設定ファイルを選択してください（任意）</p>
					{/if}
				</div>
			</div>
		</div>
	</div>
</div>

<style>
</style>
