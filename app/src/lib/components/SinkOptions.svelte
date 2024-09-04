<script lang="ts">
	import {
		isIntegerParameter,
		isStringParameter,
		isBooleanParameter,
		createRangeArray
	} from '$lib/sinkparams';

	export let sinkOptionKeys;
	export let sinkParameters;
</script>

{#if sinkOptionKeys.length > 0}
	<div class="flex flex-col gap-2">
		{#each sinkOptionKeys as key}
			{#if isIntegerParameter(sinkParameters.items[key].parameter)}
				<div class="flex gap-2 w-80">
					<label for={key} class="w-3/4 pointer-events-none"
						>{sinkParameters.items[key].label}</label
					>
					<select
						bind:value={sinkParameters.items[key].parameter.Integer.value}
						id={key}
						class="w-1/4 border-2 border-gray-300 px-2 rounded-md"
					>
						{#if sinkParameters.items[key].parameter.Integer.min !== undefined && sinkParameters.items[key].parameter.Integer.max !== undefined}
							{#each createRangeArray(sinkParameters.items[key].parameter.Integer.min, sinkParameters.items[key].parameter.Integer.max) as value}
								<option {value} class="text-center">{value}</option>
							{/each}
						{/if}
					</select>
				</div>
			{:else if isStringParameter(sinkParameters.items[key].parameter)}
				<!-- TODO String input -->
				<!-- <div class="flex gap-2 w-80">
								<label for={key} class="w-3/4">{sinkParameters.items[key].label}</label>
								<input
									type="text"
									id={key}
									bind:value={sinkParameters.items[key].parameter.String.value}
									class="w-1/4"
								/>
							</div> -->
			{:else if isBooleanParameter(sinkParameters.items[key].parameter)}
				<div class="flex gap-2 w-80 items-center">
					<label for={key} class="w-3/4 pointer-events-none"
						>{sinkParameters.items[key].label}</label
					>
					<div class="relative inline-block w-10 h-6 rounded-full cursor-pointer ml-auto">
						<input
							bind:checked={sinkParameters.items[key].parameter.Boolean.value}
							id={key}
							type="checkbox"
							class="absolute w-10 h-6 transition-colors duration-300 rounded-full appearance-none cursor-pointer peer bg-gray-200 checked:bg-accent1 peer-checked:before:bg-accent1"
						/>
						<label
							for={key}
							class="before:content[''] absolute top-2/4 -left-1 h-6 w-6 -translate-y-2/4 cursor-pointer rounded-full border border-blue-gray-100 bg-white shadow-md transition-all duration-300 peer-checked:translate-x-full"
						>
							<div
								class="inline-block p-5 rounded-full top-2/4 left-2/4 -translate-x-2/4 -translate-y-2/4"
							></div>
						</label>
					</div>
				</div>
			{/if}
		{/each}
	</div>
{/if}
