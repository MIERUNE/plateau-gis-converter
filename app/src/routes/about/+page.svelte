<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<div class="grid place-items-center h-screen">
	<div class="max-w-md flex flex-col gap-6">
		<h1 class="font-semibold text-lg">PLATEAU GIS Converter</h1>

		<p>
			国土交通省によるPLATEAUデータを、各種ファイル形式へ変換するためのツールです。
			<br />

			<a href="https://www.mlit.go.jp/plateau/" target="_blank" rel="noopener noreferrer"
				>PLATEAU [プラトー] |
				国土交通省が主導する、日本全国の3D都市モデルの整備・オープンデータ化プロジェクト</a
			>
		</p>

		<p>
			PLATEAUの標準仕様に準拠したCityGML2.0形式の3D都市モデルは、専門のGISツールやCUIツールを用いて他のGIS形式に変換して用いられることが一般的ですが、一般ユーザーが簡易に利用可能な汎用的ツールは存在しません。このため、流通や活用の範囲が専門家や技術者に限られていました。「PLATEAU
			GIS Converter」を利用することで、3D都市モデルを用いた様々な分析・開発を行うことができます。
		</p>

		<ul>
			<li>・GeoPackage 形式による QGIS 等での解析</li>
			<li>・Mapbox Vector Tiles (MVT) 形式による、大規模データのWeb等での高速描画</li>
			<li>・KML 形式による Google Earth での可視化</li>
			<li>・3D Tiles 形式による Cesium 等での可視化</li>
			<li>など</li>
		</ul>

		<p>
			当ツールの開発は<a href="https://mierune.co.jp" target="_blank">MIERUNE Inc.</a>が行いました。
		</p>

		<h1 class="font-semibold text-lg">利用手順</h1>

		<p>
			安定版のソフトウェアは、
			<a
				href="https://github.com/Project-PLATEAU/PLATEAU-GIS-Converter/releases"
				target="_blank"
				rel="noopener noreferrer">Releaseページ</a
			>
			からダウンロードしてください。
			<br />
			詳しい利用方法については、こちらの
			<a
				href="https://project-plateau.github.io/PLATEAU-GIS-Converter"
				target="_blank"
				rel="noopener noreferrer">マニュアル</a
			>をご覧ください。
		</p>

		<p>
			最新版は
			<a
				href="https://github.com/MIERUNE/plateau-gis-converter/releases"
				target="_blank"
				rel="noopener noreferrer">こちら</a
			>
			からダウンロードしてください。
		</p>

		<div class="mx-auto">
			<a href="/">
				<Icon class="text-4xl" icon="material-symbols:arrow-circle-left" />
			</a>
		</div>
	</div>
</div>

<style lang="postcss">
	a {
		@apply underline;
	}
	a:hover {
		@apply text-accent1 no-underline;
	}
</style>
