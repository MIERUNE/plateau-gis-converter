<script lang="ts">
	import type { Snippet } from 'svelte';

	let { buttonChildren, tooltipContent }: { buttonChildren: Snippet; tooltipContent: Snippet } =
		$props();
</script>

<div class="group relative flex items-center">
	<button class="cursor-pointer hover:text-accent1">
		{@render buttonChildren()}
	</button>
	<div
		class="invisible absolute bottom-0 left-1/2 z-10 inline-block max-h-64 -translate-x-1/2 translate-y-full overflow-y-auto rounded-sm border bg-white px-6 py-2 text-left whitespace-nowrap text-base opacity-0 shadow-sm transition-all duration-300 group-hover:visible group-hover:opacity-100"
	>
		{@render tooltipContent()}
	</div>
</div>
