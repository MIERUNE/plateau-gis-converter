<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { fileTypeOptions, crsOptions } from '$lib/settings';
	import { Icon } from 'svelte-materialdesign-icons';

	let name = '';
	let greetMsg = '';

	async function greet() {
		greetMsg = await invoke('greet', { name });
	}
	async function mierune() {
		greetMsg = await invoke('mierune');
	}
</script>

<div class="mx-auto max-w-xl">
	<form action="" class="space-y-5">
		<div class="space-y-5">
			<div class="flex justify-end">
				<button
					class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded inline-flex items-center w-15 h-15"
				>
					<Icon name="cog-outline" class="shrink-0 h-5 w-5 mr-1" />

					<span>設定</span>
				</button>
			</div>

			<div class="grid grid-cols-3 items-center">
				<label for="form" class="col-span-1 block text-sm font-medium text-gray-700"
					>入力ファイル</label
				>
				<div class="col-span-2">
					<div class="bg-white justify-center">
						<input
							class="block w-full mb-5 text-sm text-gray-900 border border-gray-300 rounded-lg cursor-pointer"
							id="default_size"
							type="file"
						/>
					</div>
				</div>
			</div>

			<div class="grid grid-cols-3 items-center">
				<label for="form" class="col-span-1 block text-sm font-medium text-gray-700">出力形式</label
				>
				<div class="bg-white">
					<select
						id="File type"
						class="bg-gray-50 border border-gray-100 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5"
					>
						<option selected>出力形式を選択する</option>
						{#each fileTypeOptions as fileType}
							<option value={fileType}>{fileType}</option>
						{/each}
					</select>
				</div>
			</div>

			<div class="grid grid-cols-3 items-center">
				<label for="form" class="col-span-1 block text-sm font-medium text-gray-700"
					>マッピング</label
				>
				<div class="col-span-2">
					<div class="bg-white justify-center">
						<input
							class="block w-full mb-5 text-sm text-gray-900 border border-gray-300 rounded-lg cursor-pointer"
							id="default_size"
							type="file"
						/>
					</div>
				</div>
			</div>

			<div class="grid grid-cols-3 items-center">
				<label for="form" class="col-span-1 block text-sm font-medium text-gray-700">出力先</label>
				<div class="col-span-2">
					<div class="bg-white justify-center">
						<input
							class="block w-full mb-5 text-sm text-gray-900 border border-gray-300 rounded-lg cursor-pointer"
							id="default_size"
							type="file"
						/>
					</div>
				</div>
			</div>

			<div
				class="my-8 flex items-center gap-4 before:h-px before:flex-1 before:bg-gray-300 before:content-[''] after:h-px after:flex-1 after:bg-gray-300 after:content-['']"
			>
				高度な設定
			</div>

			<div class="grid grid-cols-3 items-center">
				<label for="form" class="col-span-1 block text-sm font-medium text-gray-700"
					>変換先CRS</label
				>
				<div class="bg-white">
					<select
						id="File type"
						class="bg-gray-50 border border-gray-100 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5"
					>
						<option selected>変換先CRSを選択する</option>
						{#each crsOptions as crs}
							<option value={crs.value}>{crs.label} ({crs.value})</option>
						{/each}
					</select>
				</div>
			</div>

			<div class="flex justify-around">
				<button
					class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded inline-flex items-center"
				>
					<Icon name="card-multiple" class="shrink-0 h-5 w-5 mr-1" />
					<span>一括処理</span>
				</button>

				<button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full">
					変換
				</button>
			</div>
		</div>
	</form>
</div>
