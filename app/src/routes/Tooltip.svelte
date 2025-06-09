<script lang="ts">
	import type { Snippet } from 'svelte';

	let { buttonChildren, tooltipContent }: { buttonChildren: Snippet; tooltipContent: Snippet } =
		$props();
</script>

<div class="relative group flex items-center">
	<button class="cursor-pointer hover:text-accent1">
		{@render buttonChildren()}
	</button>
	<div
		class="absolute left-1/2 bottom-0 inline-block whitespace-nowrap text-left px-6 py-2 bg-white text-base border rounded-sm shadow-sm z-10 transition-all duration-300 -translate-x-1/2 translate-y-full group-hover:opacity-100 group-hover:visible invisible opacity-0 max-h-64 overflow-y-auto"
	>
		{@render tooltipContent()}
	</div>
</div>
