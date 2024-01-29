# Nusamai: BRIDGE 都市デジタルツイン・GISコンバータの開発

[![Test Tauri App](https://github.com/MIERUNE/nusamai/actions/workflows/test_app.yml/badge.svg)](https://github.com/MIERUNE/nusamai/actions/workflows/test_app.yml)
[![Test Libraries](https://github.com/MIERUNE/nusamai/actions/workflows/test_libs.yml/badge.svg)](https://github.com/MIERUNE/nusamai/actions/workflows/test_libs.yml)
[![codecov](https://codecov.io/gh/MIERUNE/nusamai/graph/badge.svg?token=oa62wDWoqu)](https://codecov.io/gh/MIERUNE/nusamai)
[![Docs](https://github.com/MIERUNE/nusamai/actions/workflows/doc.yml/badge.svg)](https://mierune.github.io/nusamai/app/)

- Notion: [BRIDGE 都市デジタルツイン・GISコンバータの開発](https://www.notion.so/mierune/BRIDGE-GIS-461ba0355b3041619ed3f303a8b0166f)
- Rustdoc: <https://mierune.github.io/nusamai/app/>

## リポジトリ構成

- アプリケーション：
  - [`app`](./app/) &mdash; Tauri による GUI アプリケーション
  - [`nusamai`](./nusamai/) &mdash; アプリケーションのバックエンド (およびCLI実装)
- 基盤・ユーティリティ：
  - [`nusamai-geometry`](./nusamai-geometry/) &mdash; ジオメトリ型
  - [`nusamai-projection`](./nusamai-projection/) &mdash; 投影法変換
- データソース：
  - [`nusamai-citygml`](./nusamai-plateau/citygml/) &mdash; CityGML パーサ
  - [`nusamai-plateau`](./nusamai-plateau/) &mdash; PLATEAU 用の CityGML モデル
    - [`macros`](./nusamai-plateau/citygml/macros/) &mdash; パーサ導出用の proc macros
- 変換先形式のためのライブラリ：
  - [`nusamai-3dtiles`](./nusamai-3dtiles/) &mdash; 3D Tiles
  - [`nusamai-mvt`](./nusamai-mvt/) &mdash; Mapbox Vector Tiles (MVT)
  - [`nusamai-gpkg`](./nusamai-gpkg/) &mdash; GeoPackage
  - [`nusamai-gltf`](./nusamai-gltf/) &mdash; glTF
  - [`nusamai-geojson`](./nusamai-geojson/) &mdash; GeoJSON

### 外部リポジトリ

- [MIERUNE/earcut-rs](https://github.com/MIERUNE/earcut-rs) &mdash; ポリゴン三角形化アルゴリズム
- [MIERUNE/japan-geoid](https://github.com/MIERUNE/japan-geoid) &mdash; 日本のジオイドモデル (JGD2011 → WGS 84 の変換に必要)

## Build

### GUI

```bash
cd app
npm run tauri build --release
```

## Development

### CLI

```bash
cd ./nusamai/
# Debug (非常に低速)
cargo run -- ~/path/to/PLATEAU/15100_niigata-shi_2022_citygml_1_op/udx/bldg/*.gml --sink geojson --output foobar.geojson
# Release (最適化コンパイル、実用速度)
cargo run --release -- ~/path/to/PLATEAU/15100_niigata-shi_2022_citygml_1_op/udx/bldg/*.gml --sink geojson --output foobar.geojson
# Release (LTO有効のプロダクションビルド、最高速)
cargo run --profile release-lto -- ~/path/to/Desktop/PLATEAU/15100_niigata-shi_2022_citygml_1_op/udx/bldg/*.gml --sink geojson --output foobar.geojson
```

## 参考

- [MIERUNE/plateau-qgis-plugin](https://github.com/MIERUNE/plateau-qgis-plugin) &mdash; PLATEAU QGIS Plugin の実装
- [MIERUNE/plateau-schema-experiment](https://github.com/MIERUNE/plateau-schema-experiment) &mdash; CityGML 2.0 と i-UR の XML Schema を解析する実験コード群。QGIS Pluginの属性列挙に使用。
- [MIERUNE/3dtiles-research](https://github.com/MIERUNE/3dtiles-research) &mdash; 3D Tiles / glTF の実験コード群
