<script lang="ts">
	import { filetypeOptions } from '$lib/settings';
	import Icon from '@iconify/svelte';
	import { dialog } from '@tauri-apps/api';
	import { invoke } from '@tauri-apps/api/tauri';

	export let filetype: string;
	export let epsg: number = 4979;
	export let rulesPath: string;
	export let transformerRegistry: { key: string; label: string; is_enabled: boolean }[];

	$: epsgOptions = filetypeOptions[filetype]?.epsg || [];
	$: disableEpsgOptions = epsgOptions.length < 2;

	$: {
		// Reset the target CRS if the selected filetype does not support the current CRS
		if (!filetypeOptions[filetype]?.epsg.some((item) => item.value === epsg)) {
			epsg = filetypeOptions[filetype]?.epsg[0].value || 4979;
		}
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

	function clearRulesPath() {
		rulesPath = '';
	}

	async function getTransformerRegistry(filetype: string) {
		const registry = (await invoke('get_transform', { filetype })) as any;

		transformerRegistry = registry.configs.map(
			(transformerConfig: { key: string; label: string; is_enabled: boolean }) => {
				return {
					key: transformerConfig.key,
					label: transformerConfig.label,
					is_enabled: transformerConfig.is_enabled
				};
			}
		);
	}

	$: getTransformerRegistry(filetype);
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
		{#if transformerRegistry && transformerRegistry.length > 0}
			<div class="flex flex-col gap-1.5">
				<label for="transform-select" class="font-bold">出力の詳細設定</label>
				{#each transformerRegistry as config}
					<div class="inline-flex items-center gap-6">
						<label
							for={config.key}
							class="mt-px mb-0 ml-3 font-light text-gray-700 cursor-pointer select-none text-sm w-52"
						>
							{config.label}
						</label>
						<div class="relative inline-block w-10 h-6 rounded-full cursor-pointer">
							<input
								bind:checked={config.is_enabled}
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
				{/each}
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
						<p>設定ファイルが選択されていません</p>
					{/if}
				</div>
			</div>
		</div>
	</div>
</div>

<style>
</style>
