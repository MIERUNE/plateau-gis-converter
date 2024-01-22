<script lang="ts">
	import { dialog } from '@tauri-apps/api';
	import { fileTypeOptions } from '$lib/settings';

	export let fileType: string;
	export let outputPath: string;

	async function openOutputDialog() {
		const res = await dialog.save();
		outputPath = Array.isArray(res) ? res[0] : res;
	}
</script>

<div>
	<h2 class="font-bold text-xl">出力</h2>

	<div class="flex flex-col gap-5 mt-3 ml-2">
		<div class=" flex flex-col gap-1.5">
			<label for="filetype-select" class="font-bold">形式</label>
			<select bind:value={fileType} name="filetype" id="filetype-select" class="w-36">
				{#each fileTypeOptions as fileType}
					<option value={fileType}>{fileType}</option>
				{/each}
			</select>
		</div>

		<div class="flex flex-col gap-1.5">
			<div class="font-bold">出力先</div>
			<div class="flex items-center gap-3">
				<button
					on:click={openOutputDialog}
					class="bg-accent1 font-semibold rounded px-4 py-0.5 shadow hover:opacity-75">選択</button
				>
				<div class="text-sm" class:opacity-50={!outputPath}>
					{(outputPath.length < 36 ? outputPath : `... ${outputPath.slice(-36)}`) ||
						'出力先が選択されていません'}
				</div>
			</div>
		</div>
	</div>
</div>
