<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let name = '';
	let greetMsg = '';

	async function greet() {
		greetMsg = await invoke('greet', { name });
	}
	async function mierune() {
		greetMsg = await invoke('mierune');
	}

	let fileTypeOptions = ['GeoJSON', 'GeoPackage', 'CZML'];

	let crsOptions = [
		{ value: 'EPSG:6678', label: 'JGD2011 / Japan Plane Rectangular CS X' },
		{ value: 'EPSG:4326', label: 'WGS 84' },
		{ value: 'EPSG:3857', label: 'Web Mercator' }
	];
</script>

<div class="mx-auto max-w-xl">
	<form action="" class="space-y-5">
		<div class="space-y-5">
			<div class="flex justify-end">
				<button
					class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded inline-flex items-center w-15 h-15"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="1.5"
						stroke="currentColor"
						class="w-6 h-6"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M21.75 6.75a4.5 4.5 0 01-4.884 4.484c-1.076-.091-2.264.071-2.95.904l-7.152 8.684a2.548 2.548 0 11-3.586-3.586l8.684-7.152c.833-.686.995-1.874.904-2.95a4.5 4.5 0 016.336-4.486l-3.276 3.276a3.004 3.004 0 002.25 2.25l3.276-3.276c.256.565.398 1.192.398 1.852z"
						/>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M4.867 19.125h.008v.008h-.008v-.008z"
						/>
					</svg>

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
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="1.5"
						stroke="currentColor"
						class="w-6 h-6"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M3.75 12h16.5m-16.5 3.75h16.5M3.75 19.5h16.5M5.625 4.5h12.75a1.875 1.875 0 010 3.75H5.625a1.875 1.875 0 010-3.75z"
						/>
					</svg>
					<span>一括処理</span>
				</button>

				<button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full">
					変換
				</button>
			</div>
		</div>
	</form>
</div>
