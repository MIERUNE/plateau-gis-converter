<script lang="ts">
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';
	import VirtualScroll from 'svelte-virtual-scroll-list';

	async function cancelConversion() {
		await invoke('cancel_conversion');
	}

	type Item = {
		id: number;
		message: string;
		level: string;
		error_message?: string;
		source: string;
	};

	let items: Item[] = $state([]);

	let logView: VirtualScroll;

	// Setup log monitor
	onMount(() => {
		let promise = listen<{
			message: string;
			level: string;
			error_message?: string;
			source: string;
		}>('conversion-log', (event) => {
			items.push({
				id: items.length,
				...event.payload
			});
			items = items;
			logView.scrollToBottom();
		});

		return () => {
			promise.then((unlisten) => {
				unlisten();
			});
		};
	});
</script>

<div class="flex flex-col place-items-center gap-4 p-12">
	<p class="text-center text-2xl font-semibold text-white">変換中 &hellip;</p>
	<div class="[&>div] flex justify-center [&>div]:bg-white">
		<div class="m-2 h-4 w-4 rounded-full bg-white animate-loading-pulse-[-0.24s]"></div>
		<div class="m-2 h-4 w-4 rounded-full bg-white animate-loading-pulse-[-0.12s]"></div>
		<div class="m-2 h-4 w-4 rounded-full bg-white animate-loading-pulse-[0s]"></div>
	</div>

	<div
		class="my-5 h-96 max-h-96 w-full rounded-sm bg-slate-900/70 p-1 font-mono text-xs text-slate-300"
	>
		<div class="h-full w-full">
			<VirtualScroll bind:this={logView} data={items} key="id">
				{#snippet children({ data }: { data: Item })}
					<div>
						{#if data.level === 'ERROR'}
							<span
								class="inline-flex items-center rounded-md bg-red-400/10 px-1 py-0.5 text-xs font-medium text-red-400 ring-1 ring-red-400/20 ring-inset"
								>ERROR</span
							>
						{:else if data.level === 'WARN'}
							<span
								class="inline-flex items-center rounded-md bg-yellow-400/10 px-1 py-0.5 text-xs font-medium text-yellow-500 ring-1 ring-yellow-400/20 ring-inset"
								>WARN</span
							>
						{:else if data.level === 'INFO'}
							<span
								class="inline-flex items-center rounded-md bg-blue-400/10 px-1 py-0.5 text-xs font-medium text-blue-400 ring-1 ring-blue-400/30 ring-inset"
								>INFO</span
							>
						{/if}

						<span
							class="inline-flex items-center rounded-md bg-gray-400/10 px-1 py-0.5 text-xs font-medium text-gray-400 ring-1 ring-gray-400/20 ring-inset"
							>{data.source}</span
						>

						{data.message}

						{#if data.error_message}
							<span class="text-red-600">{data.error_message}</span>
						{/if}
					</div>
				{/snippet}
			</VirtualScroll>
		</div>
	</div>

	<div>
		<button
			onclick={cancelConversion}
			class="shawdow-2xl flex items-center gap-1 rounded-full bg-red-300 py-1.5 pr-5 pl-3 font-bold hover:bg-red-400"
		>
			<Icon class="text-lg" icon="ic:baseline-cancel" />
			キャンセル
		</button>
	</div>
</div>
