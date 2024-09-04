<script lang="ts">
	import { filetypeOptions } from '$lib/settings';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { SinkParameters } from '$lib/sinkparams';
	import type { TransformerRegistry } from '$lib/transformer';
	import Icon from '@iconify/svelte';
	import { dialog } from '@tauri-apps/api';
	import SinkOptions from '$lib/components/SinkOptions.svelte';
	import TransformerOptions from '$lib/components/TransformerOptions.svelte';

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
	const targetSink = import.meta.env.VITE_TEST_SINK;
	if (targetSink) {
		if (filetypeOptions.hasOwnProperty(targetSink)) {
			filetype = targetSink;
		} else {
			console.warn(`The specified test sink "${targetSink}" is invalid. Using default value.`);
		}
	}
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
			<div class="flex flex-col">
				<label for="transform-select" class="font-bold mb-1.5">出力の詳細設定</label>
				<div class="flex flex-col gap-2">
					<TransformerOptions bind:transformerRegistry />
					<SinkOptions bind:sinkOptionKeys bind:sinkParameters />
				</div>
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
