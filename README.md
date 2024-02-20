# BRIDGE 都市デジタルツイン・GISコンバータ（PLATEAU-GIS-Converter）

[![Test Tauri App](https://github.com/MIERUNE/nusamai/actions/workflows/test_app.yml/badge.svg)](https://github.com/MIERUNE/nusamai/actions/workflows/test_app.yml)
[![Test Libraries](https://github.com/MIERUNE/nusamai/actions/workflows/test_libs.yml/badge.svg)](https://github.com/MIERUNE/nusamai/actions/workflows/test_libs.yml)
[![codecov](https://codecov.io/gh/MIERUNE/nusamai/graph/badge.svg?token=oa62wDWoqu)](https://codecov.io/gh/MIERUNE/nusamai)
[![Docs](https://github.com/MIERUNE/nusamai/actions/workflows/doc.yml/badge.svg)](https://mierune.github.io/nusamai/app/)

PLATEAU-GIS-ConverterはCityGML形式の3D都市モデルを標準的なGISデータに変換するオーサリングツールです。本ツールでは3D都市モデルを以下の形式に変換することが可能です。

- GeoJSON
- Shapefile
- GeoPackage
- 3DTiles
- KML
- CZML
- Mapbox Vector Tiles

[静岡県沼津市の CityGML (v2)](https://www.geospatial.jp/ckan/dataset/plateau-22203-numazu-shi-2021/resource/758fe63a-b20f-4d5d-817d-c35eac530143) を読み込んで、3DTilesに変換した例

※todo: 画像を貼る

PLATEAU-GIS-Converterを利用することで、3D都市モデルを用いた様々な分析・開発を行うことが可能です。

- KMLによるGoogle Earthでの可視化
- GeoPackageによるQGIS等での解析
- Mapbox Vector Tilesを用いた大規模データの高速描画
- 3DTilesを利用したCesiumでの可視化
- など

PLATEAU-GIS-Converterは利用者アンケートを実施しています。
今後の開発の参考にするため、ユーザーの皆様の忌憚ないご意見をお聞かせください。

※todo: Googleフォームのリンクを貼る

## プラグインの利用方法

### 動作環境

※todo: 動作環境は要検証

- Windows（x86_64）
- MacOS（ARM）
- MacOS (Intel)

# 利用手順

※todo: リンク先を更新

- ソフトウェアの最新版は[Releaseページ]()からダウンロードしてください。
- 詳しい利用方法については、こちらの[マニュアル]() をご覧ください。

## ライセンス

- 本リポジトリはMITライセンスで提供されています。
- 本ソフトウェアの開発は[株式会社 MIERUNE](https://www.mierune.co.jp/)が行なっています。
- ソースコードおよび関連ドキュメントの著作権は国土交通省に帰属します。

## 注意事項

- 本リポジトリは参考資料として提供しているものです。動作の保証は行っておりません。
- 本リポジトリの内容は予告なく変更・削除する場合があります。
- 本リポジトリおよび本プラグインの利用により生じた損失及び損害等について、国土交通省はいかなる責任も負わないものとします。

## 開発者向け情報

### リポジトリ構成

- アプリケーション：
  - [`app`](./app/) &mdash; Tauri による GUI 実装
  - [`nusamai`](./nusamai/) &mdash; バックエンドおよびコマンドライン版実装
- 基盤・ユーティリティ：
  - [`nusamai-geometry`](./nusamai-geometry/) &mdash; ジオメトリ型
  - [`nusamai-projection`](./nusamai-projection/) &mdash; 投影法変換
- データソース：
  - [`nusamai-citygml`](./nusamai-plateau/citygml/) &mdash; CityGML パーサ実装支援ライブラリ
    - [`macros`](./nusamai-plateau/citygml/macros/) &mdash; パーサ導出用の proc macros
  - [`nusamai-plateau`](./nusamai-plateau/) &mdash; PLATEAU 用の CityGML モデルおよびパーサ
- 変換先形式のための支援ライブラリ（本プロジェクトのユースケースが混入しないように開発する）
  - [`nusamai-3dtiles`](./nusamai-3dtiles/) &mdash; 3D Tiles
  - [`nusamai-mvt`](./nusamai-mvt/) &mdash; Mapbox Vector Tiles (MVT)
  - [`nusamai-gpkg`](./nusamai-gpkg/) &mdash; GeoPackage
  - [`nusamai-gltf`](./nusamai-gltf/) &mdash; glTF
  - [`nusamai-geojson`](./nusamai-geojson/) &mdash; GeoJSON
  - [`nusamai-kml`](./nusamai-kml/) &mdash; KML
  - [`nusamai-czml`](./nusamai-kml/) &mdash; CZML
  - [`nusamai-shapefile`](./nusamai-shapefile/) &mdash; Shapefile

### 外部リポジトリ

- [MIERUNE/earcut-rs](https://github.com/MIERUNE/earcut-rs) &mdash; ポリゴン三角形化アルゴリズム
- [MIERUNE/japan-geoid](https://github.com/MIERUNE/japan-geoid) &mdash; 日本のジオイドモデル (JGD2011 → WGS 84 の変換に必要)

### その他のリポジトリ

- [MIERUNE/plateau-qgis-plugin](https://github.com/MIERUNE/plateau-qgis-plugin) &mdash; PLATEAU QGIS Plugin の実装
- [MIERUNE/plateau-schema-experiment](https://github.com/MIERUNE/plateau-schema-experiment) &mdash; CityGML 2.0 と i-UR の XML Schema を解析する実験コード群。QGIS Pluginの属性列挙に使用。
- [MIERUNE/3dtiles-research](https://github.com/MIERUNE/3dtiles-research) &mdash; 3D Tiles / glTF の実験コード群

### Build &amp; Run

#### CLI

```bash
cd ./nusamai/
# Debug (非常に低速)
cargo run -- ~/path/to/PLATEAU/15100_niigata-shi_2022_citygml_1_op/udx/bldg/*.gml --sink geojson --output foobar.geojson
# Release (最適化コンパイル、実用速度)
cargo run --release -- ~/path/to/PLATEAU/15100_niigata-shi_2022_citygml_1_op/udx/bldg/*.gml --sink geojson --output foobar.geojson
# Release (LTO有効のプロダクションビルド、最高速)
cargo run --profile release-lto -- ~/path/to/Desktop/PLATEAU/15100_niigata-shi_2022_citygml_1_op/udx/bldg/*.gml --sink geojson --output foobar.geojson
```

#### GUI

Dev:

```console
cd ./app/
npm install
RUST_BACKTRACE=1 npx tauri dev
```

Build:

```bash
cd ./app
npx tauri build
```

### Test

#### Coverage

Codecov: <https://app.codecov.io/gh/MIERUNE/nusamai>

```bash
cargo llvm-cov --workspace --exclude app --html --all-features
```
