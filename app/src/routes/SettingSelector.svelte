<script lang="ts">
	import { dialog } from '@tauri-apps/api';
	import Icon from '@iconify/svelte';
	import { filetypeOptions, crsOptions } from '$lib/settings';

	export let filetype: string;
	export let mappingRulePath: string;

	async function openMappingRuleDialog() {
		const res = await dialog.open({
			filters: [
				{
					name: 'Mapping rule format',
					extensions: ['json']
				}
			]
		});
		if (!res) return;
		mappingRulePath = Array.isArray(res) ? res[0] : res;
	}

	function clearMappingRule() {
		mappingRulePath = '';
	}
</script>

<div>
	<div class="flex items-center gap-1.5">
		<Icon class="text-xl" icon="material-symbols:settings" />
		<h2 class="font-bold text-xl">設定</h2>
	</div>
	<hr class="mt-0.5" />

	<div class="flex flex-col gap-5 mt-3 ml-2">
		<div class=" flex flex-col gap-1.5">
			<label for="filetype-select" class="font-bold">ファイル形式</label>
			<select bind:value={filetype} name="filetype" id="filetype-select" class="w-36">
				{#each Object.entries(filetypeOptions) as [value, item]}
					<option {value}>{item.label}</option>
				{/each}
			</select>
		</div>

		<div class=" flex flex-col gap-1.5 opacity-50">
			<label for="crs-select" class="font-bold">座標参照系</label>
			<select name="crs" id="crs-select" class="w-80">
				{#each crsOptions as crs}
					<option value={crs.value}>{crs.label}</option>
				{/each}
			</select>
		</div>

		<div class=" flex flex-col gap-1.5">
			<label for="crs-select" class="font-bold">属性マッピングルール</label>
			<div class="flex items-center gap-3">
				<button
					on:click={openMappingRuleDialog}
					class="bg-accent1 font-semibold rounded px-4 py-0.5 shadow hover:opacity-75">選択</button
				>
				<div class="text-sm" class:opacity-50={!mappingRulePath}>
					{#if mappingRulePath}
						<div class="flex justify-center items-center gap-1.5">
							<p><code>{mappingRulePath}</code></p>
							<button on:click={clearMappingRule} class="hover:opacity-75">
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
