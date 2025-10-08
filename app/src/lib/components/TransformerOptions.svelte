<script lang="ts">
	import { isBooleanConfig, isSelectionConfig } from '$lib/transformer';

	let { transformerSettings = $bindable() } = $props();
</script>

{#if transformerSettings && transformerSettings.configs.length > 0}
	{#each transformerSettings.configs as config (config.key)}
		{#if isBooleanConfig(config.parameter)}
			<div class="flex w-80 items-center gap-2">
				<label for={config.key} class="pointer-events-none w-3/4">
					{config.label}
				</label>
				<div class="relative ml-auto inline-block h-6 w-10 cursor-pointer rounded-full">
					<input
						bind:checked={config.parameter.Boolean}
						id={config.key}
						type="checkbox"
						class="peer absolute h-6 w-10 cursor-pointer appearance-none rounded-full bg-gray-200 transition-colors duration-300 peer-checked:before:bg-accent1 checked:bg-accent1"
					/>
					<label
						for={config.key}
						class="before:content[''] border-blue-gray-100 absolute top-2/4 -left-1 h-6 w-6 -translate-y-2/4 cursor-pointer rounded-full border bg-white shadow-md transition-all duration-300 peer-checked:translate-x-full"
					>
						<div
							class="top-2/4 left-2/4 inline-block -translate-x-2/4 -translate-y-2/4 rounded-full p-5"
						></div>
					</label>
				</div>
			</div>
		{:else if isSelectionConfig(config.parameter)}
			<div class="flex w-80 gap-2">
				<label for={config.key} class="pointer-events-none w-2/4">{config.label}</label>
				<select
					id={config.key}
					class="w-2/4 cursor-pointer rounded-md border-2 border-gray-300 px-2"
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
