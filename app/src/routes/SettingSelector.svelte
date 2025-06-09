<script lang="ts">
	import { filetypeOptions } from '$lib/settings';
	import { invoke } from '@tauri-apps/api/core';
	import type { SinkParameters } from '$lib/sinkparams';
	import type { TransformerSettings } from '$lib/transformer';
	import Icon from '@iconify/svelte';
	import {} from '@tauri-apps/api';
	import SinkOptions from '$lib/components/SinkOptions.svelte';
	import TransformerOptions from '$lib/components/TransformerOptions.svelte';
	import * as dialog from '@tauri-apps/plugin-dialog';

	interface Props {
		filetype: string;
		epsg?: number;
		rulesPath: string;
		sinkParameters: SinkParameters;
		transformerSettings: TransformerSettings | undefined;
	}

	let {
		filetype = $bindable(),
		epsg = $bindable(4979),
		rulesPath = $bindable(),
		sinkParameters = $bindable(),
		transformerSettings = $bindable()
	}: Props = $props();

	let epsgOptions = $derived(filetypeOptions[filetype]?.epsg || []);
	let disableEpsgOptions = $derived(epsgOptions.length < 2);

	$effect(() => {
		// Reset the target CRS if the selected filetype does not support the current CRS
		if (!filetypeOptions[filetype]?.epsg.some((item) => item.value === epsg)) {
			epsg = filetypeOptions[filetype]?.epsg[0].value || 4979;
		}
	});

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

	async function getTransformerSettings(filetype: string) {
		const registry = await invoke<TransformerSettings>('get_transform', { filetype });

		transformerSettings = registry;
	}

	$effect(() => {
		getTransformerSettings(filetype);
	});

	let sinkOptionKeys: string[] = $state([]);

	// Get the parameter options for the selected filetype
	async function setSinkParameter(filetype: string) {
		const parameters = await invoke<SinkParameters>('get_parameter', { filetype });

		// Exclude '@output' and 'transform' from the parameter options list
		sinkOptionKeys = Object.keys(parameters.items).filter(
			(item) => item !== '@output' && item !== 'transform'
		);
		sinkParameters = parameters;
	}

	$effect(() => {
		setSinkParameter(filetype);
	});

	// Select the file format specified for testing, if any
	const targetSink = import.meta.env.VITE_TEST_SINK;
	if (targetSink) {
		if (targetSink in filetypeOptions) {
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
			<select
				bind:value={filetype}
				name="filetype"
				id="filetype-select"
				class="w-80 cursor-pointer"
			>
				{#each Object.entries(filetypeOptions) as [value, item] (value)}
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
				class="w-80 cursor-pointer"
				disabled={disableEpsgOptions}
				class:opacity-50={disableEpsgOptions}
				class:cursor-auto={disableEpsgOptions}
			>
				{#each epsgOptions as option (option.value)}
					<option value={option.value}>{option.label}</option>
				{/each}
			</select>
		</div>

		{#if (transformerSettings && transformerSettings.configs.length > 0) || sinkOptionKeys.length > 0}
			<div class="flex flex-col">
				<label for="transform-select" class="font-bold mb-1.5">出力の詳細設定</label>
				<div class="flex flex-col gap-2">
					<TransformerOptions bind:transformerSettings />
					<SinkOptions bind:sinkOptionKeys bind:sinkParameters />
				</div>
			</div>
		{/if}

		<div class=" flex flex-col gap-1.5">
			<label for="mapping-rule-select" class="font-bold">属性マッピングルール</label>
			<div class="flex items-center gap-3">
				<button
					onclick={openRulesPathDialog}
					class="bg-accent1 font-semibold rounded-sm px-4 py-0.5 shadow-sm hover:opacity-75"
					>選択</button
				>
				<div class="text-sm" class:opacity-50={!rulesPath}>
					{#if rulesPath}
						<div class="flex justify-center items-center gap-1.5">
							<p><code>{rulesPath}</code></p>
							<button onclick={clearRulesPath} class="hover:opacity-75">
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
