<script lang="ts">
	import { page } from '$app/state';
	import Icon from '@iconify/svelte';
	import type { SvelteHTMLElements } from 'svelte/elements';

	let currentPath = page.url.pathname;

	let props: SvelteHTMLElements['nav'] = $props();
</script>

<nav {...props}>
	<div class="isolate inline-flex rounded-md shadow-sm">
		<a
			href="/"
			data-selected={currentPath === '/' ? '' : undefined}
			class="relative inline-flex items-center gap-2 rounded-l-md bg-white px-4 py-2 text-sm font-semibold text-gray-900 ring-1 ring-gray-300 ring-inset hover:bg-gray-50 focus:z-10 data-selected:pointer-events-none data-selected:bg-accent1"
		>
			<Icon icon="ph:files" />
			ファイル・フォルダから
		</a>
		<a
			href="/zip"
			data-selected={currentPath === '/zip' ? '' : undefined}
			class="relative -ml-px inline-flex items-center gap-2 bg-white px-4 py-2 text-sm font-semibold text-gray-900 ring-1 ring-gray-300 ring-inset hover:bg-gray-50 focus:z-10 data-selected:pointer-events-none data-selected:bg-accent1"
		>
			<Icon icon="material-symbols:folder-zip" />
			ZIPファイルから
		</a>
		<a
			href="/map"
			data-selected={currentPath === '/map' ? '' : undefined}
			class="relative -ml-px inline-flex items-center gap-2 rounded-r-md bg-white px-4 py-2 text-sm font-semibold text-gray-900 ring-1 ring-gray-300 ring-inset hover:bg-gray-50 focus:z-10 data-selected:pointer-events-none data-selected:bg-accent1"
		>
			<Icon icon="material-symbols:map" />
			地図から
		</a>
	</div>
</nav>
