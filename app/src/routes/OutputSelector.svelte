<script lang="ts">
	import {} from '@tauri-apps/api';
	import Icon from '@iconify/svelte';
	import { abbreviatePath } from '$lib/utils';
	import { filetypeOptions } from '$lib/settings';
	import * as dialog from '@tauri-apps/plugin-dialog';
	import { untrack } from 'svelte';

	interface Props {
		filetype: string;
		outputPath: string;
	}

	let { filetype, outputPath = $bindable() }: Props = $props();

	async function openOutputDialog() {
		const res = await dialog.save({
			filters: [
				{
					name: 'Output format',
					extensions: filetypeOptions[filetype].extensions
				}
			]
		});
		outputPath = Array.isArray(res) ? res[0] : res;
	}

	$effect(() => {
		// eslint-disable-next-line @typescript-eslint/no-unused-expressions
		filetype;
		untrack(() => {
			outputPath = import.meta.env.VITE_TEST_OUTPUT_PATH ?? '';
		});
	});

	function clearSelected() {
		outputPath = '';
	}
</script>

<div>
	<div class="flex items-center gap-1.5">
		<Icon class="text-xl" icon="material-symbols:output-rounded" />
		<h2 class="font-bold text-xl">出力</h2>
	</div>
	<hr class="mt-0.5" />

	<div class="flex flex-col gap-5 mt-3 ml-2">
		<div class="flex items-center gap-3">
			<button
				onclick={openOutputDialog}
				class="bg-accent1 font-semibold rounded-sm px-4 py-0.5 shadow-sm hover:opacity-75"
				>選択</button
			>
			<div class="text-sm">
				{#if outputPath}
					<div class="flex justify-center items-center gap-1.5">
						<p><code>{abbreviatePath(outputPath, 40)}</code></p>
						<button onclick={clearSelected} class="hover:opacity-75">
							<Icon icon="material-symbols:cancel" />
						</button>
					</div>
				{:else}
					<p class="text-red-400">出力先が選択されていません</p>
				{/if}
			</div>
		</div>
	</div>
</div>
