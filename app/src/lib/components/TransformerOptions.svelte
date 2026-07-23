<script lang="ts">
	import {
		isBooleanConfig,
		isSelectionConfig,
		type DisabledTransformerConfigReasons,
		type TransformerSettings
	} from '$lib/transformer';

	let {
		transformerSettings = $bindable(),
		disabledConfigReasons = {}
	}: {
		transformerSettings: TransformerSettings | undefined;
		disabledConfigReasons?: DisabledTransformerConfigReasons;
	} = $props();
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
			{@const disabledReason = disabledConfigReasons[config.key]}
			<div class="flex flex-col gap-1">
				<div class="flex w-80 gap-2" class:opacity-50={Boolean(disabledReason)}>
					<label for={config.key} class="pointer-events-none w-2/4">{config.label}</label>
					{#if disabledReason}
						<select
							id={config.key}
							disabled
							aria-describedby={`${config.key}-disabled-reason`}
							class="w-2/4 cursor-not-allowed rounded-md border-2 border-gray-300 px-2"
						>
							<option>対象外</option>
						</select>
					{:else}
						<select
							id={config.key}
							class="w-2/4 cursor-pointer rounded-md border-2 border-gray-300 px-2"
							bind:value={config.parameter.Selection.selected_value}
						>
							{#each config.parameter.Selection.options as option, index (index)}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					{/if}
				</div>
				{#if disabledReason}
					<p id={`${config.key}-disabled-reason`} class="w-80 text-xs text-gray-500">
						{disabledReason}
					</p>
				{/if}
			</div>
		{/if}
	{/each}
{/if}
