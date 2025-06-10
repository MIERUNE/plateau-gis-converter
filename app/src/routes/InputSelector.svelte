<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import Icon from '@iconify/svelte';
	import { abbreviatePath } from '$lib/utils';
	import * as dialog from '@tauri-apps/plugin-dialog';
	import { untrack } from 'svelte';
	import Tooltip from './Tooltip.svelte';

	// let isFolderMode = true;
	let isFolderMode = $state(!import.meta.env.VITE_TEST_INPUT_PATH);
	let inputDirectories: string[] = $state([]);
	let {
		inputPaths = $bindable([])
	}: {
		inputPaths?: string[];
	} = $props();

	$effect(() => {
		// eslint-disable-next-line @typescript-eslint/no-unused-expressions
		isFolderMode;
		untrack(() => {
			inputDirectories = [];
			inputPaths = import.meta.env.VITE_TEST_INPUT_PATH
				? [import.meta.env.VITE_TEST_INPUT_PATH]
				: [];
		});
	});

	async function openDirectoryDialog() {
		const res = await dialog.open({
			multiple: true,
			directory: true
		});
		if (!res) return;

		inputDirectories = Array.isArray(res) ? res : [res];
		
		try {
			// Rust側でファイル列挙を実行
			inputPaths = await invoke<string[]>('list_supported_files', {
				directories: inputDirectories
			});

			if (inputPaths.length === 0) {
				await dialog.message('選択したフォルダにGMLファイルが含まれていません', {
					kind: 'warning'
				});
				inputDirectories = [];
			}
		} catch (error) {
			console.error('Failed to list files:', error);
			await dialog.message('ファイルの列挙中にエラーが発生しました', {
				kind: 'error'
			});
			inputDirectories = [];
			inputPaths = [];
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
		inputDirectories = [];
		inputPaths = [];
	}
</script>

<div>
	<div class="flex items-center gap-1.5">
		<Icon class="text-xl" icon="material-symbols:input-rounded" />
		<h2 class="text-xl font-bold">入力</h2>
	</div>
	<hr class="mt-0.5" />

	<div class="ml-3">
		<div>
			<span class="isolate my-3 inline-flex rounded-md">
				<button
					type="button"
					data-active={isFolderMode ? '' : undefined}
					class="relative inline-flex items-center gap-1 rounded-l-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-gray-300 ring-inset hover:bg-gray-50 focus:z-10 data-active:pointer-events-none data-active:bg-accent1"
					onclick={() => (isFolderMode = true)}
					><Icon icon="material-symbols:folder" />フォルダ選択</button
				>
				<button
					type="button"
					data-active={!isFolderMode ? '' : undefined}
					class="relative -ml-px inline-flex items-center gap-1 rounded-r-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-gray-300 ring-inset hover:bg-gray-50 focus:z-10 data-active:pointer-events-none data-active:bg-accent1"
					onclick={() => (isFolderMode = false)}><Icon icon="ph:files" />ファイル選択</button
				>
			</span>
		</div>

		<div class="flex items-center gap-3">
			<button
				onclick={isFolderMode ? openDirectoryDialog : openFileDialog}
				class="rounded-sm bg-accent1 px-4 py-0.5 font-semibold shadow-sm hover:opacity-75"
				>選択</button
			>
			<div class="text-sm">
				{#if isFolderMode}
					{#if inputDirectories.length === 0}
						<p class="text-red-400">フォルダが選択されていません</p>
					{:else}
						<div class="flex items-center gap-1">
							<p>
								<b>{inputDirectories.length}</b> フォルダ （計 <b>{inputPaths.length}</b> GMLファイル）
							</p>
							{#snippet buttonChildren()}
								<Icon class="text-2xl" icon="material-symbols-light:list-alt-rounded" />
							{/snippet}
							{#snippet tooltipContent()}
								<ol class="list-decimal pl-4">
									{#each inputDirectories as folder (folder)}
										<li class="text-xs">{abbreviatePath(folder, 30)}</li>
									{/each}
								</ol>
							{/snippet}
							<Tooltip {buttonChildren} {tooltipContent} />
							<button onclick={clearSelected} class="hover:opacity-75">
								<Icon icon="material-symbols:cancel" />
							</button>
						</div>
					{/if}
				{:else if inputPaths.length === 0}
					<p class="text-red-400">ファイルが選択されていません</p>
				{:else}
					<div class="flex items-center gap-1">
						<p><b>{inputPaths.length}</b> ファイル</p>
						{#snippet buttonChildren2()}
							<Icon class="text-2xl" icon="material-symbols-light:list-alt-rounded" />
						{/snippet}
						{#snippet tooltipContent2()}
							<ol class="list-decimal pl-4">
								{#each inputPaths as path (path)}
									<li class="text-xs">{abbreviatePath(path, 30)}</li>
								{/each}
							</ol>
						{/snippet}
						<Tooltip buttonChildren={buttonChildren2} tooltipContent={tooltipContent2} />
						<button onclick={clearSelected} class="hover:opacity-75">
							<Icon icon="material-symbols:cancel" />
						</button>
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>
