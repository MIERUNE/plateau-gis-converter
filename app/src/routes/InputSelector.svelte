<script lang="ts">
	import { dialog } from '@tauri-apps/api';
	import Icon from '@iconify/svelte';

	export let inputPath = '';

	async function openInputDialog() {
		const res = await dialog.open({
			multiple: true,
			directory: false,
			filters: [
				{
					name: 'CityGML',
					extensions: ['gml']
				}
			]
		});
		inputPath = Array.isArray(res) ? res[0] : res ?? '';
	}

	let isFolderMode = true;
</script>

<div>
	<h2 class="font-bold text-xl">入力</h2>

	<div class="ml-3">
		<div>
			<span class="isolate inline-flex rounded-md shadow-sm my-3">
				<button
					type="button"
					class="relative inline-flex gap-1 items-center rounded-l-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
					class:active={isFolderMode}
					on:click={() => (isFolderMode = true)}
					><Icon icon="material-symbols:folder" />フォルダ選択</button
				>
				<button
					type="button"
					class="relative -ml-px inline-flex gap-1 items-center rounded-r-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
					class:active={!isFolderMode}
					on:click={() => (isFolderMode = false)}><Icon icon="ph:files" />ファイル選択</button
				>
			</span>
		</div>

		<div class="flex items-center gap-3">
			<button
				on:click={openInputDialog}
				class="bg-accent1 font-semibold rounded px-4 py-0.5 shadow hover:opacity-75">選択</button
			>
			<div class="text-sm" class:opacity-50={!inputPath}>
				{(inputPath.length < 36 ? inputPath : `... ${inputPath.slice(-36)}`) ||
					`入力${isFolderMode ? 'フォルダ' : 'ファイル'}が選択されていません`}
			</div>
		</div>
	</div>
</div>

<style>
	.active {
		@apply bg-accent1;
		pointer-events: none;
	}
</style>
