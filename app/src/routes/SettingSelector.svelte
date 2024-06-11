<script lang="ts">
	import { dialog } from '@tauri-apps/api';
	import Icon from '@iconify/svelte';
	import { filetypeOptions } from '$lib/settings';
    import { invoke } from '@tauri-apps/api/tauri';
    import type { Parameters, ParameterType } from '$lib/parameters';

	export let filetype: string;
	export let epsg: number = 4979;
	export let rulesPath: string;
    export let paramsOption: Parameters;

    let debug: any;
    let optionParameter: string[] = [];

	$: epsgOptions = filetypeOptions[filetype]?.epsg || [];
	$: disableEpsgOptions = epsgOptions.length < 2;

	$: {
		// Reset the target CRS if the selected filetype does not support the current CRS
		if (!filetypeOptions[filetype]?.epsg.some((item) => item.value === epsg)) {
			epsg = filetypeOptions[filetype]?.epsg[0].value || 4979;
		}
	}

    async function setOptionParameter(filetype: string) {
        const parameters = await invoke('get_parameter', { filetype }) as Parameters;
        // '@output'を除外
        // delete parameters.items["@output"];
        
        const keys = Object.keys(parameters.items).filter(item => item !== '@output');

        // if (keys.length === 0) {
        //     optionParameter = [];
        //     return;
        // }

        optionParameter = keys;
        paramsOption = parameters
        debug = parameters
    }

    $:setOptionParameter(filetype);

	async function openRulesPathDialog() {
		const res = await dialog.open({
			filters: [
				{
					name: 'Mapping rule format',
					extensions: ['json']
				}
			]
		});
		if (!res) return;
		rulesPath = Array.isArray(res) ? res[0] : res;
	}

	function clearRulesPath() {
		rulesPath = '';
	}

     // 型ガード関数
    function isIntegerParameter(parameter: any): parameter is { Integer: { value: number; min: number; max: number; } } {
        return (parameter as { Integer?: unknown }).Integer !== undefined;
    }

    function isStringParameter(parameter: any): parameter is { String: { value: string; } } {
        return (parameter as { String?: unknown }).String !== undefined;
    }

    function isBooleanParameter(parameter: any): parameter is { Boolean: { value: boolean; } } {
        return (parameter as { Boolean?: unknown }).Boolean !== undefined;
    }

    // ParameterTypeの型を取得
    function getParameterType(key: string): ParameterType {
        const parameter = paramsOption.items[key].parameter;
        if (isIntegerParameter(parameter)) return "Integer";
        if (isStringParameter(parameter)) return "String";
        if (isBooleanParameter(parameter)) return "Boolean";
        throw new Error("Unknown parameter type");
    }

    // async function test() {
    //     await invoke ('set_parameter', {paramsOption});
    // }
</script>
{#if debug}
    <p>{JSON.stringify(debug)}</p>
{/if}
<!-- <button on:click={test}>test</button> -->
<div>
	<div class="flex items-center gap-1.5">
		<Icon class="text-xl" icon="material-symbols:settings" />
		<h2 class="font-bold text-xl">設定</h2>
	</div>
	<hr class="mt-0.5" />

	<div class="flex flex-col gap-5 mt-3 ml-2">
		<div class="flex flex-col gap-1.5">
			<label for="filetype-select" class="font-bold">ファイル形式</label>
			<select bind:value={filetype} name="filetype" id="filetype-select" class="w-80">
				{#each Object.entries(filetypeOptions) as [value, item]}
					<option {value}>{item.label}</option>
				{/each}
			</select>
		</div>

        {#if optionParameter.length > 0}
        <div class="flex flex-col gap-1.5">
			<label for="epsg-select" class="font-bold">出力オプション</label>
            <div>
		{#each optionParameter as key}
                <!-- Integerの場合 -->
                {#if getParameterType(key) === 'Integer'}
                <div class="flex gap-2 w-80">
                    <label for={key} class="w-3/4">{paramsOption.items[key].description}</label>
                    <input type="number" id={key} min={paramsOption.items[key].parameter.Integer.min} max={paramsOption.items[key].parameter.Integer.max} bind:value={paramsOption.items[key].parameter.Integer.value} class="w-1/4"/>
                </div>
                {:else if getParameterType(key) === 'String'}
                <!-- Stringの場合 -->
                <div class="flex gap-2 w-80">
                    <label for={key} class="w-3/4">{paramsOption.items[key].description}</label>
                    <input type="text" id={key} bind:value={paramsOption.items[key].parameter.String.value} class="w-1/4"/>
                </div>
                {:else if getParameterType(key) === 'Boolean'}
                <!-- Booleanの場合 -->
                <div class="flex gap-2 w-80">
                    <label for={key} class="w-3/4">{paramsOption.items[key].description}</label>
                    <input type="checkbox" id={key} bind:checked={paramsOption.items[key].parameter.Boolean.value} class="w-1/4"/>
                </div>
                {/if}
            {/each}

            </div>
		</div>
        {/if}

		<div class="flex flex-col gap-1.5">
			<label for="epsg-select" class="font-bold">座標参照系</label>
			<select
				bind:value={epsg}
				name="epsg"
				id="epsg-select"
				class="w-80"
				disabled={disableEpsgOptions}
				class:opacity-50={disableEpsgOptions}
			>
				{#each epsgOptions as option}
					<option value={option.value}>{option.label}</option>
				{/each}
			</select>
		</div>

		<div class=" flex flex-col gap-1.5">
			<label for="mapping-rule-select" class="font-bold">属性マッピングルール</label>
			<div class="flex items-center gap-3">
				<button
					on:click={openRulesPathDialog}
					class="bg-accent1 font-semibold rounded px-4 py-0.5 shadow hover:opacity-75">選択</button
				>
				<div class="text-sm" class:opacity-50={!rulesPath}>
					{#if rulesPath}
						<div class="flex justify-center items-center gap-1.5">
							<p><code>{rulesPath}</code></p>
							<button on:click={clearRulesPath} class="hover:opacity-75">
								<Icon icon="material-symbols:cancel" />
							</button>
						</div>
					{:else}
						<p>設定ファイルが選択されていません</p>
					{/if}
				</div>
			</div>
		</div>
	</div>
</div>

<style>
</style>
