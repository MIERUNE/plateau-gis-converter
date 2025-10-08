<script lang="ts">
	import {
		isIntegerParameter,
		isStringParameter,
		isBooleanParameter,
		createRangeArray
	} from '$lib/sinkparams';

	let { sinkOptionKeys = $bindable(), sinkParameters = $bindable() } = $props();
</script>

{#if sinkOptionKeys.length > 0}
	<div class="flex flex-col gap-2">
		{#each sinkOptionKeys as key (key)}
			{#if isIntegerParameter(sinkParameters.items[key].parameter)}
				<div class="flex w-80 gap-2">
					<label for={key} class="pointer-events-none w-3/4"
						>{sinkParameters.items[key].label}</label
					>
					<select
						bind:value={sinkParameters.items[key].parameter.Integer.value}
						id={key}
						class="w-1/4 cursor-pointer rounded-md border-2 border-gray-300 px-2"
					>
						{#if sinkParameters.items[key].parameter.Integer.min !== undefined && sinkParameters.items[key].parameter.Integer.max !== undefined}
							{#each createRangeArray(sinkParameters.items[key].parameter.Integer.min, sinkParameters.items[key].parameter.Integer.max) as value (value)}
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
				<div class="flex w-80 items-center gap-2">
					<label for={key} class="pointer-events-none w-3/4"
						>{sinkParameters.items[key].label}</label
					>
					<div class="relative ml-auto inline-block h-6 w-10 cursor-pointer rounded-full">
						<input
							bind:checked={sinkParameters.items[key].parameter.Boolean.value}
							id={key}
							type="checkbox"
							class="peer absolute h-6 w-10 cursor-pointer appearance-none rounded-full bg-gray-200 transition-colors duration-300 peer-checked:before:bg-accent1 checked:bg-accent1"
						/>
						<label
							for={key}
							class="before:content[''] border-blue-gray-100 absolute top-2/4 -left-1 h-6 w-6 -translate-y-2/4 cursor-pointer rounded-full border bg-white shadow-md transition-all duration-300 peer-checked:translate-x-full"
						>
							<div
								class="top-2/4 left-2/4 inline-block -translate-x-2/4 -translate-y-2/4 rounded-full p-5"
							></div>
						</label>
					</div>
				</div>
			{/if}
		{/each}
	</div>
{/if}
