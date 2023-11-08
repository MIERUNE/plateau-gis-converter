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
</script>

<div class="text-center flex flex-col gap-12 my-12">
	<h1 class="text-2xl font-bold text-red-700">Hello MIERUNE</h1>

	<div>
		<input class="border-4" id="greet-input" placeholder="Enter a name..." bind:value={name} />
		<button on:click={greet}>Greet</button>
		<p>{greetMsg}</p>
	</div>
	<div>
		<button on:click={mierune}>MIERUNE</button>
	</div>
</div>
