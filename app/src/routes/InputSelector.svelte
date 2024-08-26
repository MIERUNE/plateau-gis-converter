<script lang="ts">
	import { dialog, fs } from '@tauri-apps/api';
	import Icon from '@iconify/svelte';
	import { abbreviatePath } from '$lib/utils';

	// let isFolderMode = true;
	let isFolderMode = import.meta.env.VITE_TEST_INPUT_PATH ? false : true; // NOTE debug
	let inputFolders: string[] = [];
	export let inputPaths: string[] = [];

	// Clear the inputs when the mode changes
	$: if (isFolderMode || !isFolderMode) {
		inputFolders = [];
		// inputPaths = [];
		inputPaths = import.meta.env.VITE_TEST_INPUT_PATH ? [import.meta.env.VITE_TEST_INPUT_PATH] : []; // NOTE debug
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

		if (inputPaths.length === 0) {
			await dialog.message('選択したフォルダにGMLファイルが含まれていません', {
				type: 'warning'
			});
			inputFolders = [];
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

	function clearSelected() {
		inputFolders = [];
		inputPaths = [];
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
			<span class="isolate inline-flex rounded-md my-3">
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
						<p class="text-red-400">フォルダが選択されていません</p>
					{:else}
						<div class="flex items-center gap-1">
							<p>
								<b>{inputFolders.length}</b> フォルダ （計 <b>{inputPaths.length}</b> GMLファイル）
							</p>
							<button class="tooltip hover:text-accent1">
								<Icon class="text-2xl" icon="material-symbols-light:list-alt-rounded" />
								<div class="tooltip-text max-h-64 overflow-y-auto">
									<ol>
										{#each inputFolders as folder}
											<li class="text-xs">{abbreviatePath(folder, 30)}</li>
										{/each}
									</ol>
								</div>
							</button>
							<button on:click={clearSelected} class="hover:opacity-75">
								<Icon icon="material-symbols:cancel" />
							</button>
						</div>
					{/if}
				{:else if inputPaths.length === 0}
					<p class="text-red-400">ファイルが選択されていません</p>
				{:else}
					<div class="flex items-center gap-1">
						<p><b>{inputPaths.length}</b> ファイル</p>
						<button class="tooltip hover:text-accent1">
							<Icon class="text-2xl" icon="material-symbols-light:list-alt-rounded" />
							<div class="tooltip-text max-h-64 overflow-y-auto">
								<ol>
									{#each inputPaths as path}
										<li class="text-xs">{abbreviatePath(path, 30)}</li>
									{/each}
								</ol>
							</div>
						</button>
						<button on:click={clearSelected} class="hover:opacity-75">
							<Icon icon="material-symbols:cancel" />
						</button>
					</div>
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

	ol {
		@apply pl-4;
		@apply list-decimal;
	}

	.tooltip {
		position: relative;
		cursor: pointer;
	}

	.tooltip-text {
		opacity: 0;
		visibility: hidden;
		position: absolute;
		left: 50%;
		transform: translateX(-50%) translateY(100%);
		bottom: 0px;
		display: inline-block;
		white-space: nowrap;
		@apply text-left;
		@apply px-6 py-2;
		@apply bg-white text-base border rounded shadow;
		transition: 0.3s ease-in;
		z-index: 10;
	}

	.tooltip:hover .tooltip-text {
		opacity: 1;
		visibility: visible;
	}
</style>
