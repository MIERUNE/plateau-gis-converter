# コマンドラインで利用する

本アプリはコマンドラインから利用することも可能です。

## Rustのインストール

[公式サイト](https://www.rust-lang.org/ja/tools/install)に従い、Rustをインストールしてください。macOSかLinuxの場合は、以下のコマンドでインストールできます。

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
たとえば、`~/Downloads/PLATEAU-GIS-Converter-main`に解凍した場合は、以下のコマンドで移動できます。

```sh
cd ~/Downloads/PLATEAU-GIS-Converter-main
```

## データを変換する

以下のようにコマンドを実行することで、GeoPackageに変換できます。
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

- `--` : 以降の引数はファイル名として解釈されます。`*`を使って複数ファイルを指定できます。
- `--sink` : 出力形式を指定します。以下のように指定することが可能です。
  - `3dtiles` : 3D Tiles
  - `gpkg` : GeoPackage
  - `mvt` : Mapbox Vector Tiles
  - `mlt` : MapLibre Tiles
  - `pmtiles` : PMTiles
  - `geojson` : GeoJSON
  - `czml` : CZML
  - `gltf` : glTF
  - `kml` : KML
  - `ply` : PLY
  - `minecraft` : Minecraft Java World Data
  - `obj`: Wavefront OBJ
  - `shapefile` : Shapefile
- `--output` : 出力先を指定します。拡張子なども指定してください。
- `--epsg` : 出力座標参照系のEPSGコードを指定します。MVT・MLT・PMTilesではWeb Mercator XYZタイルを採用しているため、EPSG:3857に固定されています。これらの形式では省略するか、`--epsg 3857`を指定してください。
- `-t`: 変換設定を指定します。利用可能なオプションはGUIと同様です。
  - `use_lod`
    - `max_lod`: 最大LODを抽出する
    - `min_lod`: 最小LODを抽出する
    - `lod0`: LOD0のみを抽出する
    - `lod1`: LOD1のみを抽出する
    - `lod2`: LOD2のみを抽出する
    - `lod3`: LOD3のみを抽出する
    - `lod4`: LOD4のみを抽出する
    - `textured_max_lod`: テクスチャ付きの最大LODを抽出し、テクスチャがない場合は最大のLODを抽出する
  - `split_components`: GeoPackage形式専用です。`true`で構成要素を独立した地物として型別テーブルへ出力します。未指定または`false`では従来どおり親へ統合します。
- `-o`: 出力ファイル形式固有のオプションを設定します。
  - `split`: OBJ形式専用です。オブジェクト分割についてbool値で設定します。
  - `limit_texture_resolution`: 3D形式専用です。距離（メートル）あたりのテクスチャ解像度を制限します。
    - 有効にすると、小さな地物の過剰に高解像度なテクスチャを適切に調整し、全体的なパフォーマンスを向上させます。

#### 設定例

- 建築物の構成要素をGeoPackageの型別テーブルへ出力する

```bash
cargo run --release -- ~/data/udx/bldg/*.gml \
--sink gpkg --output ~/data/output/components.gpkg -t split_components=true -t use_lod=lod2
```

外部天井面・壁面・屋根面など、元データにある構成要素を地物型ごとのテーブル（`bldg:OuterCeilingSurface`、`bldg:WallSurface`、`bldg:RoofSurface`など）の独立レコードにします。窓・扉・交通領域も分離の対象です。同じ型は同じテーブルに入り、ファイルは1つです。形状から構成要素を推定する機能ではなく、出力LODの設定に従います。

子のID・形状・属性と、直接の親を示す`parentId`・`parentType`を保持します。親属性は子へ複製しません。親自身に形状がある場合は親も出力し、子との形状の重なりは除去しません。分離後の親に形状がなければ親は出力せず、子から親形状を再構成しません。その場合、`parentId`に対応する親行が出力内に存在しないことがあります。

- 中央区すべての建築物を、テクスチャ付きで3D Tilesに変換する

```bash
cargo run --release -- ~/13102_chuo-ku_city_2023_citygml_1_op/udx/bldg/*.gml \
--sink 3dtiles --output ~/data/output/3dtiles_textured -t use_lod=textured_max_lod
```

- テクスチャ付きかつ、テクスチャ解像度を制限してOBJ形式を出力する

```bash
% cargo run --release -- ~/13104_shinjuku-ku_city_2023_citygml_1_op/udx/bldg/53394525_bldg_6697_op.gml \
--sink obj --output ~/data/output/obj -t use_lod=textured_max_lod -o limit_texture_resolution=true -o split=true
```

- 建築物をLOD1のみに絞ってGeoJSONを出力する

```bash
cargo run --release -- ~/13102_chuo-ku_city_2023_citygml_1_op/udx/bldg/*.gml \
--sink geojson --output ~/data/output/buildings_lod1.geojson -t use_lod=lod1
```

- 建築物をMVT・MLT・PMTilesへ出力する

```bash
# MVT（Z/X/Y.pbf）
cargo run --release -- ~/13102_chuo-ku_city_2023_citygml_1_op/udx/bldg/*.gml \
--sink mvt --output ~/data/output/mvt

# MLT（Z/X/Y.mlt）
cargo run --release -- ~/13102_chuo-ku_city_2023_citygml_1_op/udx/bldg/*.gml \
--sink mlt --output ~/data/output/mlt

# PMTiles（単一ファイル）
cargo run --release -- ~/13102_chuo-ku_city_2023_citygml_1_op/udx/bldg/*.gml \
--sink pmtiles --output ~/data/output/buildings.pmtiles
```
