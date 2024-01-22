<script lang="ts">
	import Icon from '@iconify/svelte';
	import { fileTypeOptions, crsOptions } from '$lib/settings';

	export let fileType: string;
</script>

<div>
	<div class="flex items-center gap-1.5">
		<Icon class="text-xl" icon="material-symbols:settings" />
		<h2 class="font-bold text-xl">設定</h2>
	</div>

	<div class="flex flex-col gap-5 mt-3 ml-2">
		<div class=" flex flex-col gap-1.5">
			<label for="filetype-select" class="font-bold">形式</label>
			<select bind:value={fileType} name="filetype" id="filetype-select" class="w-36">
				{#each fileTypeOptions as fileType}
					<option value={fileType}>{fileType}</option>
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

		<div class=" flex flex-col gap-1.5 opacity-50">
			<label for="crs-select" class="font-bold">属性マッピング</label>
			<div class="flex items-center gap-3">
				<button class="bg-accent1 font-semibold rounded px-4 py-0.5 shadow hover:opacity-75"
					>選択</button
				>
				<div class="text-sm">設定ファイルが選択されていません</div>
			</div>
		</div>
	</div>
</div>

<style>
</style>
