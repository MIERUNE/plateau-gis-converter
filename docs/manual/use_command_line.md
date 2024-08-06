# コマンドラインで利用する

本アプリはコマンドラインから利用することも可能です。

## Rustのインストール

[公式サイト](https://www.rust-lang.org/ja/tools/install)に従い、Rustをインストールしてください。MacOSかLinuxの場合は、以下のコマンドでインストールできます。

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

![alt text](../resources/useCommandLine_image.png)

Windowsの場合は、[公式サイト](https://www.rust-lang.org/ja/tools/install)からインストーラをダウンロードしてください。

![alt text](../resources/useCommandLine_image-1.png)

## リポジトリをクローンする

本ツールのソースコードをクローンしてください。
GitHubでの開発に慣れている方は、コマンドラインでcloneしても良いですし、そうではない方は、`Download ZIP`ボタンをクリックしてダウンロードしてください。

![alt text](../resources/useCommandLine_image-2.png)

ZIPファイルでダウンロードした方は、解凍した後に、コマンドラインで解凍したディレクトリに移動してください。
例えば、`~/Downloads/PLATEAU-GIS-Converter-main`に解凍した場合は、以下のコマンドで移動できます。

```sh
cd ~/Downloads/PLATEAU-GIS-Converter-main
```

## データを変換する

以下のようにコマンドを実行することで、GeoPackageに変換することができます。
実行速度が高速になればなるほど、コンパイルに時間がかかります。

```bash
cd ./nusamai/
# Debug (非常に低速)
cargo run -- ~/path/to/PLATEAU/15100_niigata-shi_2022_citygml_1_op/udx/bldg/*.gml --sink geojson --output foobar.geojson
# Release (最適化コンパイル、実用速度)
cargo run --release -- ~/path/to/PLATEAU/15100_niigata-shi_2022_citygml_1_op/udx/bldg/*.gml --sink geojson --output foobar.geojson
# Release (LTO有効のプロダクションビルド、最高速)
cargo run --profile release-lto -- ~/path/to/PLATEAU/15100_niigata-shi_2022_citygml_1_op/udx/bldg/*.gml --sink geojson --output foobar.geojson
```

### 設定項目

- `--` : 以降の引数はファイル名として解釈されます。`*`を使って複数ファイルを指定することができます。
- `--sink` : 出力形式を指定します。以下のように指定することが可能です。
  - `3dtiles` : 3D Tiles
  - `gpkg` : GeoPackage
  - `mvt` : Mapbox Vector Tiles
  - `geojson` : GeoJSON
  - `czml` : CZML
  - `gltf` : glTF
  - `kml` : KML
  - `ply` : PLY
  - `minecraft` : Minecraft Java World Data
  - `shapefile` : Shapefile
- `--output` : 出力先を指定します。拡張子なども指定してください。
