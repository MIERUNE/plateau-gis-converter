<script lang="ts">
	import type { HTMLButtonAttributes } from 'svelte/elements';

	let {
		class: className = '',
		disabled = false,
		children,
		...props
	}: HTMLButtonAttributes = $props();
</script>

<button
	{disabled}
	class={[
		'cursor-pointer rounded-sm bg-gray-500 px-4 py-1 text-sm text-nowrap text-white hover:opacity-75 disabled:cursor-auto disabled:opacity-50',
		className
	]}
	{...props}
>
	{#if children}
		{@render children()}
	{/if}
</button>
