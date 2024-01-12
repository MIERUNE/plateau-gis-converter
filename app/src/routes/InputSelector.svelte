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

<div class="">
	<h2 class="font-bold text-xl">入力</h2>
	<div>
		<span class="isolate inline-flex rounded-md shadow-sm my-3">
			<button
				type="button"
				class="relative inline-flex items-center rounded-l-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
				class:active={isFolderMode}
				on:click={() => (isFolderMode = true)}
				><Icon icon="material-symbols:folder" />フォルダ選択</button
			>
			<button
				type="button"
				class="relative -ml-px inline-flex items-center rounded-r-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
				class:active={!isFolderMode}
				on:click={() => (isFolderMode = false)}><Icon icon="ph:files" />ファイル選択</button
			>
		</span>
	</div>

	<div class="rounded-xl flex flex-col gap-4 justify-center items-center">
		{#if isFolderMode}
			<div>
				<button
					on:click={openInputDialog}
					class="bg-blue-500 hover:bg-blue-700 text-white font-bold rounded-lg px-2 py-2 shadow"
					>フォルダ選択</button
				>
			</div>
		{:else}
			<div>
				<button
					on:click={openInputDialog}
					class="bg-blue-500 hover:bg-blue-700 text-white font-bold rounded-lg px-2 py-2 shadow"
					>ファイル選択</button
				>（複数選択可）
			</div>
			<div class="flex items-center gap-3">
				<div class={inputPath ? 'text-gray-800' : 'text-gray-500'}>
					{inputPath.length}ファイルを選択中
				</div>
				<Icon icon="material-symbols:format-list-bulleted" />
			</div>
		{/if}
	</div>
</div>

<style>
	.active {
		@apply bg-blue-500;
		@apply text-white;
		pointer-events: none;
	}
</style>
