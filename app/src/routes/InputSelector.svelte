<script lang="ts">
	import { dialog, fs } from '@tauri-apps/api';
	import Icon from '@iconify/svelte';

	let isFolderMode = true;
	let inputFolders: string[] = [];
	export let inputPaths: string[] = [];

	// Clear the inputs when the mode changes
	$: if (isFolderMode || !isFolderMode) {
		inputFolders = [];
		inputPaths = [];
	}

	async function openFolderDialog() {
		const res = await dialog.open({
			multiple: true,
			directory: true
		});
		if (!res) return;
		inputFolders = Array.isArray(res) ? res : [res];
		inputPaths = [];
		for (const folder of inputFolders) {
			const files = await fs.readDir(folder);
			const gmlFiles = files.filter((d) => d.name?.endsWith('.gml'));
			inputPaths = inputPaths.concat(gmlFiles.map((d) => d.path));
		}
	}

	async function openFileDialog() {
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
		if (!res) return;
		inputPaths = Array.isArray(res) ? res : [res];
	}
</script>

<div>
	<div class="flex items-center gap-1.5">
		<Icon class="text-xl" icon="material-symbols:input-rounded" />
		<h2 class="font-bold text-xl">入力</h2>
	</div>
	<hr class="mt-0.5" />

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
				on:click={isFolderMode ? openFolderDialog : openFileDialog}
				class="bg-accent1 font-semibold rounded px-4 py-0.5 shadow hover:opacity-75">選択</button
			>
			<div class="text-sm">
				{#if isFolderMode}
					{#if inputFolders.length === 0}
						<p class="opacity-50">フォルダが選択されていません</p>
					{:else}
						<p>{inputFolders.length}フォルダ（計{inputPaths.length}GMLファイル）</p>
					{/if}
				{:else if inputPaths.length === 0}
					<p class="opacity-50">ファイルが選択されていません</p>
				{:else}
					<p>{inputPaths.length}ファイル</p>
				{/if}
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
