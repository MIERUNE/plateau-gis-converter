<script lang="ts">
	import { isBooleanConfig, isSelectionConfig } from '$lib/transformer';

	let { transformerSettings = $bindable() } = $props();
</script>

{#if transformerSettings && transformerSettings.configs.length > 0}
	{#each transformerSettings.configs as config (config.key)}
		{#if isBooleanConfig(config.parameter)}
			<div class="flex gap-2 w-80 items-center">
				<label for={config.key} class="w-3/4 pointer-events-none">
					{config.label}
				</label>
				<div class="relative inline-block w-10 h-6 rounded-full cursor-pointer ml-auto">
					<input
						bind:checked={config.parameter.Boolean}
						id={config.key}
						type="checkbox"
						class="absolute w-10 h-6 transition-colors duration-300 rounded-full appearance-none cursor-pointer peer bg-gray-200 checked:bg-accent1 peer-checked:before:bg-accent1"
					/>
					<label
						for={config.key}
						class="before:content[''] absolute top-2/4 -left-1 h-6 w-6 -translate-y-2/4 cursor-pointer rounded-full border border-blue-gray-100 bg-white shadow-md transition-all duration-300 peer-checked:translate-x-full"
					>
						<div
							class="inline-block p-5 rounded-full top-2/4 left-2/4 -translate-x-2/4 -translate-y-2/4"
						></div>
					</label>
				</div>
			</div>
		{:else if isSelectionConfig(config.parameter)}
			<div class="flex gap-2 w-80">
				<label for={config.key} class="w-2/4 pointer-events-none">{config.label}</label>
				<select
					id={config.key}
					class="w-2/4 border-2 border-gray-300 px-2 rounded-md cursor-pointer"
					bind:value={config.parameter.Selection.selected_value}
				>
					{#each config.parameter.Selection.options as option, index (index)}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>
			</div>
		{/if}
	{/each}
{/if}
