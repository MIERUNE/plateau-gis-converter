<script lang="ts">
	import Icon from '@iconify/svelte';
	import { abbreviatePath } from '$lib/utils';
	import * as dialog from '@tauri-apps/plugin-dialog';
	import { untrack } from 'svelte';

	export type ZipOutputSelectorProps = {
		zipOutputPath: string;
	};

	let { zipOutputPath = $bindable() }: ZipOutputSelectorProps = $props();

	async function openOutputDialog() {
		const res = await dialog.save({
			filters: [
				{
					name: 'Zip (.zip)',
					extensions: ['zip']
				}
			]
		});
		if (!res) {
			return;
		}
		zipOutputPath = Array.isArray(res) ? res[0] : res;
	}

	$effect(() => {
		untrack(() => {
			zipOutputPath = import.meta.env.VITE_TEST_ZIP_OUTPUT_PATH ?? '';
		});
	});

	function clearSelected() {
		zipOutputPath = '';
	}
</script>

<div>
	<div class="flex items-center gap-1.5">
		<Icon class="text-xl" icon="material-symbols:folder-zip-outline" />
		<h2 class="text-lg font-bold">パックしたZipファイルの保存先</h2>
	</div>
	<hr class="mt-0.5" />

	<div class="mt-3 ml-2 flex flex-col gap-5">
		<div class="flex items-center gap-3">
			<button
				onclick={openOutputDialog}
				class="min-w-16 rounded-sm bg-accent1 px-4 py-0.5 font-semibold shadow-sm hover:opacity-75"
				>選択</button
			>
			<div class="min-w-20 flex-1 text-sm">
				{#if zipOutputPath}
					<div class="flex items-center justify-center gap-1.5">
						<p class="break-all">
							<code class="break-all">{abbreviatePath(zipOutputPath, 40)}</code>
						</p>
						<button onclick={clearSelected} class="hover:opacity-75">
							<Icon icon="material-symbols:cancel" />
						</button>
					</div>
				{:else}
					<p>一時ファイルを利用します</p>
				{/if}
			</div>
		</div>
	</div>
</div>
