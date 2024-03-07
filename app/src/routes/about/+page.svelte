<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<div class="grid place-items-center h-screen">
	<div class="max-w-md flex flex-col gap-9">
		<h1 class="font-semibold text-lg">PLATEAU 都市デジタルツイン・GISコンバータ</h1>

		<p>国土交通省によるPLATEAUデータを、各種ファイル形式へ変換するためのツールです。</p>
		<p>
			<a href="https://www.mlit.go.jp/plateau/" target="_blank"
				>PLATEAU [プラトー] |
				国土交通省が主導する、日本全国の3D都市モデルの整備・オープンデータ化プロジェクト</a
			>
		</p>
		<p>
			当ツールの開発は<a href="https://mierune.co.jp" target="_blank">MIERUNE Inc.</a>が行いました。
		</p>

		<div class="mx-auto">
			<a href="/">
				<Icon class="text-4xl" icon="material-symbols:arrow-circle-left" />
			</a>
		</div>
	</div>
</div>

<style>
	a {
		@apply underline;
	}
	a:hover {
		@apply text-accent1 no-underline;
	}
</style>
